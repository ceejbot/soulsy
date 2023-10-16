#include "cosave.h"
#include "hooks.h"
#include "papyrus.h"
#include "sinks.h"
#include "ui_renderer.h"

#include "lib.rs.h"

void init_logger()
{
	if (static bool initialized = false; !initialized) { initialized = true; }
	else { return; }

	try
	{
		auto path = logger::log_directory();
		if (!path) { stl::report_and_fail("failed to get standard log path"sv); }

		*path /= fmt::format("{}.log"sv, Version::PROJECT);
		auto sink = std::make_shared<spdlog::sinks::basic_file_sink_mt>(path->string(), true);
		auto log  = std::make_shared<spdlog::logger>("global log"s, std::move(sink));

		log->set_level(spdlog::level::info);
		log->flush_on(spdlog::level::info);

		spdlog::set_default_logger(std::move(log));
		spdlog::set_pattern("%H:%M:%S.%f [%l] %s(%#) %v"s);

		const auto input = path->generic_wstring();
		std::vector<uint16_t> bytes;
		bytes.reserve(input.length());
		for (auto iter = input.cbegin(); iter != input.cend(); iter++)
		{
			bytes.push_back(static_cast<uint16_t>(*iter));
		}
		initialize_rust_logging(std::move(bytes));
	}
	catch (const std::exception& e)
	{
		logger::critical("failed, what={}"sv, e.what());
	}
}

// Our handler for plugin-level SKSE messages.
// We care about new game, game loaded, and data loaded messages.
void message_callback(SKSE::MessagingInterface::Message* msg)
{
	switch (msg->type)
	{
		case SKSE::MessagingInterface::kDataLoaded:
			if (ui::ui_renderer::d_3d_init_hook::initialized)
			{
				logger::debug("SKSE data loaded callback; UI is initialized."sv);
				logger::trace("Loading images with scale values width={}; height={};"sv,
					ui::ui_renderer::get_resolution_scale_width(),
					ui::ui_renderer::get_resolution_scale_height());

				ui::ui_renderer::load_all_images();
				register_all_sinks();
				hooks::install_hooks();
				papyrus::registerPapyrusFunctions();
				initialize_hud();
			}
			break;
		case SKSE::MessagingInterface::kPostLoadGame:
		case SKSE::MessagingInterface::kNewGame:
			// logger::debug("SKSE post load-game / new game callback; type={}"sv, static_cast<uint32_t>(msg->type));
			logger::info("SKSE kNewGame post-hook done: type={};"sv, static_cast<uint32_t>(msg->type));
			break;
		default: break;
	}
}

// This is our entry point from SKSE.
//
// We initialize our logger, try to read our settings, and register our plugin-level listener.
// When we get back our listener callback, we will do our main hooking and event listening.
EXTERN_C [[maybe_unused]] __declspec(dllexport) bool SKSEAPI SKSEPlugin_Load(const SKSE::LoadInterface* a_skse)
{
	init_logger();

	logger::info(
		"---------- {} @ {}.{}.{} loading"sv, Version::PROJECT, Version::MAJOR, Version::MINOR, Version::PATCH);
	logger::info("Game version {}", a_skse->RuntimeVersion().string());
	auto settings = user_settings();

	if (settings->debug())
	{
		spdlog::set_level(spdlog::level::trace);
		spdlog::flush_on(spdlog::level::trace);
	}

	Init(a_skse);
	cosave::initializeCosaves();

	SKSE::AllocTrampoline(14 * 3);

	stl::write_thunk_call<ui::ui_renderer::d_3d_init_hook>();
	stl::write_thunk_call<ui::ui_renderer::dxgi_present_hook>();

	auto* g_message = SKSE::GetMessagingInterface();
	if (!g_message)
	{
		logger::error("Cannot get the SKSE messaging interface. Stopping initialization."sv);
		return false;
	}

	g_message->RegisterListener(message_callback);

	logger::info("{} load successful."sv, Version::PROJECT);
	return true;
}

EXTERN_C [[maybe_unused]] __declspec(dllexport) constinit auto SKSEPlugin_Version = []() noexcept {
	SKSE::PluginVersionData v;
	v.PluginName(Version::PROJECT.data());
	v.AuthorName(Version::AUTHOR);
	v.PluginVersion({ Version::MAJOR, Version::MINOR, Version::PATCH, Version::BETA });
	v.UsesAddressLibrary(true);
	v.CompatibleVersions({ SKSE::RUNTIME_SSE_LATEST });
	v.UsesNoStructs();
	return v;
}();

EXTERN_C [[maybe_unused]] __declspec(dllexport) bool SKSEAPI
	SKSEPlugin_Query(const SKSE::QueryInterface* a_skse, SKSE::PluginInfo* pluginInfo)
{
	pluginInfo->name        = SKSEPlugin_Version.pluginName;
	pluginInfo->infoVersion = SKSE::PluginInfo::kVersion;
	pluginInfo->version     = SKSEPlugin_Version.pluginVersion;

	if (a_skse->IsEditor())
	{
		logger::critical("Loaded in editor, marking as incompatible"sv);
		return false;
	}

	const auto ver = a_skse->RuntimeVersion();
	if (ver < SKSE::RUNTIME_SSE_1_5_39)
	{
		logger::critical(FMT_STRING("Unsupported runtime version {}"), ver.string());
		return false;
	}

	return true;
}

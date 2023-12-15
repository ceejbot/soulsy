#include "cosave.h"
#include "inventory.h"
#include "log.h"
#include "menus.h"
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
		auto path = SKSE::log::log_directory();
		if (!path) { stl::report_and_fail("failed to get standard log path"sv); }

		*path /= fmt::format("{}.log"sv, Version::PROJECT);
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
		rlog::critical("failed, what={}"sv, e.what());
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
				rlog::debug("SKSE data loaded callback; UI is initialized."sv);
				ui::ui_renderer::preloadImages();
				MenuHook::install();
				PlayerHook::install();
				papyrus::registerPapyrusFunctions();
			}
			break;
		case SKSE::MessagingInterface::kPostLoadGame:
		case SKSE::MessagingInterface::kNewGame:
			// rlog::debug("SKSE post load-game / new game callback; type={}"sv, static_cast<uint32_t>(msg->type));
			rlog::info("SKSE kNewGame post-hook done: type={};"sv, static_cast<uint32_t>(msg->type));
			registerAllListeners();
			initialize_hud();
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

	rlog::info("---------- {} @ {}.{}.{} loading"sv, Version::PROJECT, Version::MAJOR, Version::MINOR, Version::PATCH);
	rlog::info("Game version {}", a_skse->RuntimeVersion().string());
	auto settings = user_settings();

	auto loglevel = static_cast<spdlog::level::level_enum>(settings->log_level_number());
	spdlog::set_level(loglevel);
	spdlog::flush_on(loglevel);

	Init(a_skse);
	cosave::initializeCosaves();

	SKSE::AllocTrampoline(14 * 3);

	stl::write_thunk_call<ui::ui_renderer::d_3d_init_hook>();
	stl::write_thunk_call<ui::ui_renderer::dxgi_present_hook>();

	auto* g_message = SKSE::GetMessagingInterface();
	if (!g_message)
	{
		rlog::error("Cannot get the SKSE messaging interface. Stopping initialization."sv);
		return false;
	}

	g_message->RegisterListener(message_callback);

	rlog::info("{} load successful."sv, Version::PROJECT);
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
		rlog::critical("Loaded in editor, marking as incompatible"sv);
		return false;
	}

	const auto ver = a_skse->RuntimeVersion();
	if (ver < SKSE::RUNTIME_SSE_1_5_39)
	{
		rlog::critical("Unsupported runtime version {}", ver.string());
		return false;
	}

	return true;
}

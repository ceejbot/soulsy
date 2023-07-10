
#include "include/hooks.h"
#include "include/user_settings.h"
#include "include/sinks.h"
#include "include/ui_renderer.h"
#include "include/file_setting.h"

#include "processing/set_setting_data.h"

// #include "PCH.h"  // this will actually be force-included...
// #include "control/binding.h" // this is already gone

void init_logger()
{
	if (static bool initialized = false; !initialized)
	{
		initialized = true;
	}
	else
	{
		return;
	}

	try
	{
		auto path = logger::log_directory();
		if (!path)
		{
			stl::report_and_fail("failed to get standard log path"sv);
		}

		*path /= fmt::format("{}.log"sv, Version::PROJECT);
		auto sink = std::make_shared<spdlog::sinks::basic_file_sink_mt>(path->string(), true);
		auto log  = std::make_shared<spdlog::logger>("global log"s, std::move(sink));

		log->set_level(spdlog::level::info);
		log->flush_on(spdlog::level::info);

		spdlog::set_default_logger(std::move(log));
		spdlog::set_pattern("[%H:%M:%S.%f][%s(%#)][%!][%l] %v"s);

		logger::info("{} v{}"sv, Version::PROJECT, Version::NAME);
	}
	catch (const std::exception& e)
	{
		logger::critical("failed, cause {}"sv, e.what());
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
				logger::trace("Added Callback for UI. Now load Images, scale values width {}, height {}"sv,
					ui::ui_renderer::get_resolution_scale_width(),
					ui::ui_renderer::get_resolution_scale_height());

				ui::ui_renderer::load_all_images();
				register_all_sinks();
				hooks::install_hooks();
				papyrus::Register();
				// next line is rustland now, and we want to go with defaults instead...
				control::binding::get_singleton()->set_all_keys();
				logger::info("done with data loaded"sv);
			}
			break;
		case SKSE::MessagingInterface::kPostLoadGame:
		case SKSE::MessagingInterface::kNewGame:
			logger::info("Running checks for data and hud settings after {}"sv, static_cast<uint32_t>(msg->type));
			// TODO: replace with whatever it is we need to do in Rust.
			// processing::set_setting_data::read_and_set_data();
			// processing::set_setting_data::get_actives_and_equip();
			// processing::set_setting_data::check_config_data();
			ui::ui_renderer::set_show_ui(file_setting::get_show_ui());
			logger::info("Done running after {}"sv, static_cast<uint32_t>(msg->type));
			break;
		default:
			break;
	}
}

// This is our entry point from SKSE.
//
// We initialize our logger, try to read our settings, and register our plugin-level listener.
// When we get back our listener callback, we will do our main hooking and event listening.
EXTERN_C [[maybe_unused]] __declspec(dllexport) bool SKSEAPI SKSEPlugin_Load(const SKSE::LoadInterface* a_skse)
{
	init_logger();

	logger::info("{} loading"sv, Version::PROJECT);
	logger::info("Game version {}", a_skse->RuntimeVersion().string());

	// We now load our user settings. This is our first dip into the Rust side.
	Settings settings = settings();
	if (settings.debug)
	{
		spdlog::set_level(spdlog::level::trace);
		spdlog::flush_on(spdlog::level::trace);
	}

	Init(a_skse);

	SKSE::AllocTrampoline(14 * 3);

	stl::write_thunk_call<ui::ui_renderer::d_3d_init_hook>();
	stl::write_thunk_call<ui::ui_renderer::dxgi_present_hook>();

	auto* g_message = SKSE::GetMessagingInterface();
	if (!g_message)
	{
		logger::error("Messaging Interface Not Found. return."sv);
		return false;
	}

	g_message->RegisterListener(message_callback);

	logger::info("{} loaded"sv, Version::PROJECT);
	return true;
}

EXTERN_C [[maybe_unused]] __declspec(dllexport) constinit auto SKSEPlugin_Version = []() noexcept {
	SKSE::PluginVersionData v;
	v.PluginName(Version::PROJECT.data());
	v.AuthorName(Version::AUTHOR);
	v.PluginVersion({ Version::MAJOR, Version::MINOR, Version::PATCH, 0 });
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

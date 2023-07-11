
#include "include/papyrus.h"
#include "include/constant.h"
#include "include/custom_setting.h"
#include "include/enums.h"
#include "include/file_setting.h"
#include "include/helper.h"
#include "include/ui_renderer.h"
#include "include/user_settings.h"

#include "processing/set_setting_data.h"

#include "lib.rs.h"

static const char* mcm_name = "SoulsyHUD_MCM";

// TODO in this file: remove functions at the same time they're removed from the papyrus script
// trimming everything down to the 1 page of settings defined in the mcm config json
// rename as needed

void register_papyrus_functions()
{
	const auto* papyrus = SKSE::GetPapyrusInterface();
	papyrus->Register(PapyrusGlue::Register);
	logger::info("Registered papyrus functions. return."sv);
}

bool Register(RE::BSScript::IVirtualMachine* a_vm)
{
	a_vm->RegisterFunction("OnConfigClose", mcm_name, PapyrusGlue::on_config_close);
	a_vm->RegisterFunction("GetResolutionWidth", mcm_name, PapyrusGlue::get_resolution_width);
	a_vm->RegisterFunction("GetResolutionHeight", mcm_name, PapyrusGlue::get_resolution_height);

	logger::info("Registered {} class. return."sv, mcm_name);
	return true;
}

void PapyrusGlue::on_config_close(RE::TESQuest*)
{
	logger::info("on config close"sv);
	rust::Box<UserSettings> old_settings = user_settings();
	refresh_user_settings();
	rust::Box<UserSettings> new_settings = user_settings();

	if (old_settings->maxlen() > new_settings->maxlen()) {
		// TODO trim cycles from the end
	}

	// force a redraw if the settings changed
	ui::ui_renderer::set_fade(true, 1.f);

	logger::debug("on config close done. return."sv);
}

RE::BSFixedString PapyrusGlue::get_resolution_width(RE::TESQuest*)
{
	return fmt::format(FMT_STRING("{:.2f}"), ui::ui_renderer::get_resolution_width());
}

RE::BSFixedString PapyrusGlue::get_resolution_height(RE::TESQuest*)
{
	return fmt::format(FMT_STRING("{:.2f}"), ui::ui_renderer::get_resolution_height());
}

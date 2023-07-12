
#include "papyrus.h"
#include "constant.h"
#include "custom_setting.h"
#include "enums.h"
#include "file_setting.h"
#include "helpers.h"
#include "ui_renderer.h"
#include "user_settings.h"

#include "processing/set_setting_data.h"

#include "lib.rs.h"

namespace papyrus
{

	static const char* mcm_name = "SoulsyHUD_MCM";

	void register_papyrus_functions()
	{
		const auto* papyrus = SKSE::GetPapyrusInterface();
		papyrus->Register(Register);
		logger::info("Registered papyrus functions. return."sv);
	}

	bool Register(RE::BSScript::IVirtualMachine* a_vm)
	{
		a_vm->RegisterFunction("OnConfigClose", mcm_name, on_config_close);
		a_vm->RegisterFunction("GetResolutionWidth", mcm_name, get_resolution_width);
		a_vm->RegisterFunction("GetResolutionHeight", mcm_name, get_resolution_height);

		logger::info("Registered {} class. return."sv, mcm_name);
		return true;
	}

	void on_config_close(RE::TESQuest*)
	{
		logger::info("on config close"sv);
		rust::Box<UserSettings> old_settings = user_settings();
		refresh_user_settings();
		rust::Box<UserSettings> new_settings = user_settings();

		if (old_settings->maxlen() > new_settings->maxlen())
		{
			// TODO trim cycles from the end
		}

		// force a redraw if the settings changed
		ui::ui_renderer::set_fade(true, 1.f);

		logger::debug("on config close done. return."sv);
	}

	RE::BSFixedString get_resolution_width(RE::TESQuest*)
	{
		return fmt::format(FMT_STRING("{:.2f}"), ui::ui_renderer::get_resolution_width());
	}

	RE::BSFixedString get_resolution_height(RE::TESQuest*)
	{
		return fmt::format(FMT_STRING("{:.2f}"), ui::ui_renderer::get_resolution_height());
	}

}

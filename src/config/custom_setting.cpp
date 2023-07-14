#include "custom_setting.h"
#include "constant.h"
#include "file_setting.h"
#include "user_settings.h"

// To be deleted.

namespace config
{
	CSimpleIniA custom_ini;

	void custom_setting::read_setting()
	{
		custom_ini.Reset();
		custom_ini.SetUnicode();
		if (config::mcm_setting::get_elden_demon_souls())
		{
			custom_ini.LoadFile((util::ini_path + file_setting::get_config_elden()).c_str());
		}
		else
		{
			custom_ini.LoadFile((util::ini_path + file_setting::get_config_default()).c_str());
		}
	}

	CSimpleIniA::TNamesDepend custom_setting::get_sections()
	{
		//just to be sure, after the reorder feature
		read_setting();
		CSimpleIniA::TNamesDepend sections;
		custom_ini.GetAllSections(sections);

		return sections;
	}

	uint32_t custom_setting::get_page_by_section(const std::string& a_section)
	{
		return static_cast<uint32_t>(custom_ini.GetLongValue(a_section.c_str(), "uPage", 0));
	}

	uint32_t custom_setting::get_position_by_section(const std::string& a_section)
	{
		return static_cast<uint32_t>(custom_ini.GetLongValue(a_section.c_str(), "uPosition", 0));
	}

	uint32_t custom_setting::get_type_by_section(const std::string& a_section)
	{
		return static_cast<uint32_t>(custom_ini.GetLongValue(a_section.c_str(), "uType", 0));
	}

	std::string custom_setting::get_item_form_by_section(const std::string& a_section)
	{
		return custom_ini.GetValue(a_section.c_str(), "sSelectedItemForm", "");
	}

	uint32_t custom_setting::get_slot_action_by_section(const std::string& a_section)
	{
		return static_cast<uint32_t>(custom_ini.GetLongValue(a_section.c_str(), "uSlotAction", 0));
	}

	uint32_t custom_setting::get_hand_selection_by_section(const std::string& a_section)
	{
		return static_cast<uint32_t>(custom_ini.GetLongValue(a_section.c_str(), "uHandSelection", 1));
	}

	int custom_setting::get_effect_actor_value(const std::string& a_section)
	{
		return static_cast<int>(custom_ini.GetLongValue(a_section.c_str(), "iEffectActorValue", -1));
	}

	uint32_t custom_setting::get_type_left_by_section(const std::string& a_section)
	{
		return static_cast<uint32_t>(custom_ini.GetLongValue(a_section.c_str(), "uTypeLeft", 0));
	}

	std::string custom_setting::get_item_form_left_by_section(const std::string& a_section)
	{
		return custom_ini.GetValue(a_section.c_str(), "sSelectedItemFormLeft", "");
	}

	uint32_t custom_setting::get_slot_action_left_by_section(const std::string& a_section)
	{
		return static_cast<uint32_t>(custom_ini.GetLongValue(a_section.c_str(), "uSlotActionLeft", 0));
	}

	void custom_setting::reset_section(const std::string& a_section)
	{
		read_setting();
		logger::trace("resetting section {}"sv, a_section);
		custom_ini.Delete(a_section.c_str(), nullptr);

		save_setting();
	}

	void custom_setting::write_slot_action_by_section(const std::string& a_section, const uint32_t a_action)
	{
		read_setting();
		custom_ini.SetLongValue(a_section.c_str(), "uSlotAction", static_cast<long>(a_action));

		save_setting();
	}

	void custom_setting::write_slot_action_left_by_section(const std::string& a_section, const uint32_t a_action)
	{
		read_setting();
		custom_ini.SetLongValue(a_section.c_str(), "uSlotActionLeft", static_cast<long>(a_action));

		save_setting();
	}

	void custom_setting::write_section_setting(const std::string& a_section,
		uint32_t a_page,
		uint32_t a_position,
		uint32_t a_type,
		const std::string& a_form,
		uint32_t a_action,
		uint32_t a_hand,
		uint32_t a_type_left,
		const std::string& a_form_left,
		uint32_t a_action_left,
		int a_effect_actor_value)
	{
		logger::trace(
			"writing section {}, page {}, position {}, type {}, form {}, action {}, hand {}, type_left {}, a_form_left {}, action_left {}, a_effect_actor_value {}"sv,
			a_section,
			a_page,
			a_position,
			a_type,
			a_form,
			a_action,
			a_hand,
			a_type_left,
			a_form_left,
			a_action_left,
			a_effect_actor_value);

		const auto section = a_section.c_str();

		reset_section(section);

		custom_ini.SetLongValue(section, "uPage", static_cast<long>(a_page));
		custom_ini.SetLongValue(section, "uPosition", static_cast<long>(a_position));
		custom_ini.SetLongValue(section, "uType", static_cast<long>(a_type));
		custom_ini.SetValue(section, "sSelectedItemForm", a_form.c_str());
		custom_ini.SetLongValue(section, "uSlotAction", static_cast<long>(a_action));
		custom_ini.SetLongValue(section, "uHandSelection", static_cast<long>(a_hand));
		custom_ini.SetLongValue(section, "iEffectActorValue", a_effect_actor_value);
		custom_ini.SetLongValue(section, "uTypeLeft", static_cast<long>(a_type_left));
		custom_ini.SetValue(section, "sSelectedItemFormLeft", a_form_left.c_str());
		custom_ini.SetLongValue(section, "uSlotActionLeft", static_cast<long>(a_action_left));

		save_setting();
	}

	void custom_setting::save_setting()
	{
		if (config::mcm_setting::get_elden_demon_souls())
		{
			(void)custom_ini.SaveFile((util::ini_path + file_setting::get_config_elden()).c_str());
		}
		else
		{
			(void)custom_ini.SaveFile((util::ini_path + file_setting::get_config_default()).c_str());
		}
		read_setting();
	}
}

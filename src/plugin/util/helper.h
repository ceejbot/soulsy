#pragma once
#include "handle/data/data_helper.h"
#include "handle/data/page/position_setting.h"

namespace util
{
	class helper
	{
	public:
		using position_type = handle::position_setting::position_type;
		using slot_type     = handle::slot_setting::slot_type;

		static std::string get_mod_and_form(const RE::FormID& a_form_id);
		static std::vector<std::string> get_configured_section_page_names(
			uint32_t a_position = static_cast<uint32_t>(position_type::total));
		static RE::TESForm* get_form_from_mod_id_string(const std::string& a_str);
		static bool is_two_handed(RE::TESForm*& a_form);
		static slot_type get_type(RE::TESForm*& a_form);
		static void rewrite_settings();
		static std::string get_section_name_for_page_position(uint32_t a_page, uint32_t a_position);
		static RE::ActorValue get_actor_value_effect_from_potion(RE::TESForm* a_form, bool a_check = true);
		static void write_setting_to_file(uint32_t a_page,
			uint32_t a_position,
			const std::vector<data_helper*>& a_data,
			uint32_t a_hand);
		static bool can_instant_cast(RE::TESForm* a_form, slot_type a_type);
	};
}

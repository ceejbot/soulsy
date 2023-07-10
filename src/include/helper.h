#pragma once

#include "enums.h"

namespace helpers
{
	using slot_type = enums::slot_type;
	using action_type = enums::action_type;
	using position_type = enums::position_type;

	struct data_helper
	{
		RE::TESForm* form          = nullptr;
		enums::slot_type type             = enums::slot_type::empty;
		enums::action_type action_type    = enums::action_type::default_action;
		bool left                  = false;
		bool two_handed            = false;
		RE::ActorValue actor_value = RE::ActorValue::kNone;
	};

	struct ItemData
	{
		RE::TESForm* form              = nullptr;
		RE::ActorValue actor_value     = RE::ActorValue::kNone;
		RE::BGSEquipSlot* slot         = nullptr;
		std::string formspec           = "";
		enums::slot_type type          = enums::slot_type::empty;
		enums::action_type action_type = enums::action_type::default_action;
		bool left                      = false;
		bool two_handed                = false;
		bool has_count                 = false;
	};

	std::string get_mod_and_form(const RE::FormID& a_form_id);
	std::vector<std::string> get_configured_section_page_names(
		uint32_t a_position = static_cast<uint32_t>(position_type::total));
	RE::TESForm* get_form_from_mod_id_string(const std::string& a_str);
	bool is_two_handed(RE::TESForm*& a_form);
	slot_type get_type(RE::TESForm*& a_form);
	void rewrite_settings();
	std::string get_section_name_for_page_position(uint32_t a_page, uint32_t a_position);
	RE::ActorValue get_actor_value_effect_from_potion(RE::TESForm* a_form, bool a_check = true);
	void write_setting_to_file(uint32_t a_page,
		uint32_t a_position,
		const std::vector<data_helper*>& a_data,
		uint32_t a_hand);
	bool can_instant_cast(RE::TESForm* a_form, slot_type a_type);
}

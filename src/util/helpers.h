#pragma once

#include "enums.h"

// This namespace must never use a type from the crate,
// but it can use bridge types.
#include "rust/cxx.h"

namespace helpers
{
	using slot_type     = enums::slot_type;
	using action_type   = enums::action_type;
	using position_type = enums::position_type;

	struct data_helper
	{
		RE::TESForm* form              = nullptr;
		enums::slot_type type          = enums::slot_type::empty;
		enums::action_type action_type = enums::action_type::default_action;
		bool left                      = false;
		bool two_handed                = false;
		RE::ActorValue actor_value     = RE::ActorValue::kNone;
	};

	struct config_writer_helper
	{
		std::string section{};
		uint32_t page{};
		uint32_t position{};
		uint32_t type{};
		std::string form{};
		uint32_t action{};
		uint32_t hand{};
		uint32_t type_left{};
		std::string form_left{};
		uint32_t action_left{};
		int actor_value{};
	};

	std::string get_form_spec(RE::TESForm* form);
	std::string get_mod_and_form(const RE::FormID& a_form_id);
	std::vector<std::string> get_configured_section_page_names(
		uint32_t a_position = static_cast<uint32_t>(position_type::total));
	RE::TESForm* get_form_from_mod_id_string(const std::string& a_str);
	std::string get_section_name_for_page_position(uint32_t a_page, uint32_t a_position);
	RE::ActorValue get_actor_value_effect_from_potion(RE::TESForm* a_form, bool a_check = true);
	void write_setting_to_file(uint32_t a_page,
		uint32_t a_position,
		const std::vector<data_helper*>& a_data,
		uint32_t a_hand);

	void notify_player(const std::string& message);
	void set_alpha_transition(const bool shift, const float target);
	bool get_is_transitioning();
	void toggle_hud_visibility();
	void show_hud();

	uint32_t getSelectedFormFromMenu(RE::UI*& a_ui);
}

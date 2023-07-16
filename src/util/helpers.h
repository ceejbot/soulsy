#pragma once

// This namespace must never use a type from the crate,
// but it can use bridge types.
#include "rust/cxx.h"

namespace helpers
{
	RE::TESForm* formSpecToFormItem(const std::string& a_str);
	RE::ActorValue get_actor_value_effect_from_potion(RE::TESForm* a_form, bool a_check = true);

	std::string makeFormSpecString(RE::TESForm* form);
	uint32_t getSelectedFormFromMenu(RE::UI*& a_ui);

	// These are helpers for the rust side.
	void notify_player(const std::string& message);
	void set_alpha_transition(const bool shift, const float target);
	bool get_is_transitioning();
	void toggle_hud_visibility();
	void show_hud();
}

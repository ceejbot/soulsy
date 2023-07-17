#pragma once

#include "rust/cxx.h"

// This namespace is for rust/C++ bridge helpers.
// Functions for use by C++ use camelCase.
// Functions for use by Rust use snake_case.
// This is not a compiler-facing distinction, but instead a
// hint to the human who's calling whom.

namespace helpers
{
	RE::TESForm* formSpecToFormItem(const std::string& a_str);
	std::string makeFormSpecString(RE::TESForm* form);
	uint32_t getSelectedFormFromMenu(RE::UI*& a_ui);

	void notifyPlayer(const std::string& message);
	void fadeToAlpha(const bool shift, const float target);
	bool getIsFading();
	void toggleHUD();
	void show_hud();
}

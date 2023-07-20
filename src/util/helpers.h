#pragma once

#include "rust/cxx.h"

// This namespace is for rust/C++ bridge helpers.

namespace helpers
{
	RE::TESForm* formSpecToFormItem(const std::string& a_str);
	std::string makeFormSpecString(RE::TESForm* form);
	uint32_t getSelectedFormFromMenu(RE::UI*& a_ui);

	void notifyPlayer(const std::string& message);
	void fadeToAlpha(const bool shift, const float target);
	bool getIsFading();
	void toggleHUD();
	void showHUD();
	bool hudMustNotBeDrawn();
	bool playerNotInControl();
}

﻿#pragma once

#include "rust/cxx.h"

// This namespace is for rust/C++ bridge helpers.

namespace helpers
{
	RE::TESForm* formSpecToFormItem(const std::string& a_str);
	std::string makeFormSpecString(RE::TESForm* form);
	uint32_t getSelectedFormFromMenu(RE::UI*& a_ui);

	void notifyPlayer(const std::string& message);
	void fadeToAlpha(const bool shift, const float target);
	bool hudAllowedOnScreen();  // the authority on whether we should show the hud or not
	bool hudShouldAutoFadeOut();
	bool hudShouldAutoFadeIn();
	bool ignoreKeyEvents();

	// Called by the controller if the user has started cycling.
	void enterSlowMotion();
	// Called by the controller when the cycle timeout fires.
	void exitSlowMotion();

	//void addCycleKeyword(const std::string& form_spec);
	//void removeCycleKeyword(const std::string& form_spec);
}

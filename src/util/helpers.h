#pragma once

#include "rust/cxx.h"

// This namespace is for rust/C++ bridge helpers as well as any
// decision-making that needs a single source of truth. It's
// badly-named.

struct HudItem;

namespace helpers
{
	RE::TESForm* formSpecToFormItem(const std::string& spec);
	rust::Box<HudItem> formSpecToHudItem(const std::string& spec);
	std::string makeFormSpecString(RE::TESForm* form);
	// uint32_t getSelectedFormFromMenu(RE::UI*& a_ui);

	// play failure sound
	void honk();

	void notifyPlayer(const std::string& message);
	rust::String lookupTranslation(const std::string& key);

	bool hudAllowedOnScreen();  // the authority on whether we should show the hud or not
	bool hudShouldAutoFadeOut();
	bool hudShouldAutoFadeIn();
	bool ignoreKeyEvents();
	bool gamepadInUse();
	bool relevantMenuOpen();

	// Called by the controller if the user has started cycling.
	void enterSlowMotion();
	// Called by the controller when the cycle timeout fires.
	void exitSlowMotion();

	bool isFavoritedByFormSpec(const std::string& form_spec);
	bool isPoisonedByFormSpec(const std::string& form_spec);
	bool hasChargeByFormSpec(const std::string& form_spec);
	float chargeLevelByFormSpec(const std::string& form_spec);

	std::string vec_to_stdstring(rust::Vec<uint8_t> input);
	std::vector<uint8_t> chars_to_vec(const char* input);

	//void addCycleKeyword(const std::string& form_spec);
	//void removeCycleKeyword(const std::string& form_spec);
}

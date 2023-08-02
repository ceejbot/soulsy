#pragma once

#include "rust/cxx.h"

// This namespace is for rust/C++ bridge helpers as well as any
// decision-making that needs a single source of truth. It's
// badly-named.

namespace helpers
{
	RE::TESForm* formSpecToFormItem(const std::string& a_str);
	std::string makeFormSpecString(RE::TESForm* form);
	// uint32_t getSelectedFormFromMenu(RE::UI*& a_ui);

	// play failure sound
	void honk();

	void notifyPlayer(const std::string& message);
	void startAlphaTransition(const bool shift, const float target);
	void show_briefly();
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

	bool itemIsFavorited(RE::TESForm* item_form);

	std::string vec_to_stdstring(rust::Vec<uint8_t> input);
	std::vector<uint8_t> chars_to_vec(const char* input);

	//void addCycleKeyword(const std::string& form_spec);
	//void removeCycleKeyword(const std::string& form_spec);

	struct MenuSelection
	{
		static uint32_t getSelectionFromMenu(RE::UI*& a_ui, MenuSelection*& outSelection);
		static void makeFromFavoritesMenu(RE::FavoritesMenu* menu, MenuSelection*& outSelection);
		static void makeFromMagicMenu(RE::MagicMenu* menu, MenuSelection*& outSelection);
		static void makeFromInventoryMenu(RE::InventoryMenu* menu, MenuSelection*& outSelection);

		MenuSelection(RE::FormID formid);

		RE::FormID form_id;
		bool favorite;
		bool poisoned;
		bool equipped;
		uint32_t count;
		RE::TESBoundObject* bound_obj;
		RE::TESForm* form;
	};
}

#pragma once

#include "rust/cxx.h"

// A set of helpers for exposing item data to the Rust side, because
// not all of TESForm's methods can be punched through easily. It ends
// up being convenient for serialization to use the ItemData type,
// which is a side benefit.

struct HudItem;

namespace equippable
{
	// rust::Box<ItemData> makeItemDataFromForm(RE::TESForm* form);

	rust::Box<HudItem> hudItemFromForm(RE::TESForm* form);
	rust::Box<HudItem> subKindForConsumable(RE::TESForm*& form);

	bool requiresTwoHands(RE::TESForm*& form);
	RE::ActorValue getPotionEffect(RE::TESForm* form, bool filter);

	struct KeywordAccumulator
	{
		static inline std::vector<std::string>* keywords = new std::vector<std::string>();

		static RE::BSContainer::ForEachResult collect(RE::BGSKeyword& kwd);
		static void printKeywords();
	};
}

#pragma once

#include "rust/cxx.h"

// Builds the rust HudItem struct from game data, inspecting forms,
// keywords, and inventory data as needed.

struct HudItem;
struct SpellData;

namespace equippable
{
	rust::Box<HudItem> hudItemFromForm(RE::TESForm* form);
	rust::Box<SpellData> fillOutSpellData(bool two_handed, int32_t skill_level, const RE::EffectSetting* effect);

	bool requiresTwoHands(RE::TESForm*& form);
	RE::ActorValue getPotionEffect(RE::TESForm* form, bool filter);

	struct KeywordAccumulator
	{
		static inline std::vector<std::string>* mKeywords = new std::vector<std::string>();
		static inline void clear() { mKeywords->clear(); }

		static RE::BSContainer::ForEachResult collect(RE::BGSKeyword& kwd);
		static void printKeywords();
	};
}

#pragma once

#include "rust/cxx.h"
#include "soulsy.h"

// Builds the rust HudItem struct from game data, inspecting forms,
// keywords, and inventory data as needed.

inline const std::set<RE::FormType> RELEVANT_FORMTYPES_ALL{
	RE::FormType::AlchemyItem,
	RE::FormType::Ammo,
	RE::FormType::Armor,
	RE::FormType::Light,
	RE::FormType::Scroll,
	RE::FormType::Shout,
	RE::FormType::Spell,
	RE::FormType::Weapon,
};

inline const std::set<RE::FormType> RELEVANT_FORMTYPES_INVENTORY{
	RE::FormType::AlchemyItem,
	RE::FormType::Ammo,
	RE::FormType::Armor,
	RE::FormType::Light,
	RE::FormType::Scroll,
	RE::FormType::Weapon,
};

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

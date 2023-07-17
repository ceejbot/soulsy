#pragma once

#include "rust/cxx.h"

// A set of helpers for exposing item data to the Rust side, because
// not all of TESForm's methods can be punched through easily. It ends
// up being convenient for serialization to use the TesItemData type,
// which is a side benefit.

struct TesItemData;
enum class TesItemKind : ::std::uint8_t;

namespace equippable
{
	rust::Box<TesItemData> makeTESItemDataFromForm(RE::TESForm* form);
	TesItemKind itemKindFromForm(RE::TESForm*& item_form);

	bool requiresTwoHands(RE::TESForm*& form);
	bool canInstantCast(RE::TESForm* form, TesItemKind kind);
	RE::ActorValue getPotionEffect(RE::TESForm* form, bool filter);

	TesItemKind subKindForWeapon(RE::TESForm*& form);
	TesItemKind subKindForMagic(RE::TESForm*& form);
	TesItemKind subKindForConsumable(RE::TESForm*& form);
	TesItemKind subKindForArmor(RE::TESForm*& form);
	TesItemKind subKindForConsumableByEffect(RE::ActorValue& actor_value);
}

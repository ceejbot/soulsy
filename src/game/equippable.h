#pragma once

#include "rust/cxx.h"

// A set of helpers for exposing item data to the Rust side, because
// not all of TESForm's methods can be punched through easily. It ends
// up being convenient for serialization to use the ItemData type,
// which is a side benefit.

struct ItemData;
enum class ItemKind : ::std::uint8_t;

namespace equippable
{
	rust::Box<ItemData> makeItemDataFromForm(RE::TESForm* form);
	ItemKind itemKindFromForm(RE::TESForm*& item_form);

	bool requiresTwoHands(RE::TESForm*& form);
	bool canInstantCast(RE::TESForm* form, ItemKind kind);
	RE::ActorValue getPotionEffect(RE::TESForm* form, bool filter);

	ItemKind subKindForWeapon(RE::TESForm*& form);
	ItemKind subKindForMagic(RE::TESForm*& form);
	ItemKind subKindForConsumable(RE::TESForm*& form);
	ItemKind subKindForArmor(RE::TESForm*& form);
	ItemKind subKindForConsumableByEffect(RE::ActorValue& actor_value);

	bool isAmulet(const RE::TESObjectARMO* armor);
	bool isCloak(const RE::TESObjectARMO* armor);
	bool isFeet(const RE::TESObjectARMO* armor);
	bool isHands(const RE::TESObjectARMO* armor);
	bool isHead(const RE::TESObjectARMO* armor);
	bool isRing(const RE::TESObjectARMO* armor);

}

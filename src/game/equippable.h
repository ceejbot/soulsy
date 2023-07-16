#pragma once

#include "rust/cxx.h"

struct TesItemData;
enum class EntryKind : ::std::uint8_t;

namespace equippable
{
	rust::Box<TesItemData> makeTESItemDataFromForm(RE::TESForm* form);
	EntryKind entryKindFromForm(RE::TESForm*& item_form);

	bool requiresTwoHands(RE::TESForm*& form);
	bool canInstantCast(RE::TESForm* form, EntryKind kind);
	RE::ActorValue getPotionEffect(RE::TESForm* form, bool filter);

	EntryKind subKindForWeapon(RE::TESForm*& form);
	EntryKind subKindForMagic(RE::TESForm*& form);
	EntryKind subKindForConsumable(RE::TESForm*& form);
	EntryKind subKindForArmor(RE::TESForm*& form);
	EntryKind subKindForConsumableByEffect(RE::ActorValue& actor_value);

}

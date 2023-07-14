#pragma once

#include "enums.h"

#include "lib.rs.h"

namespace equippable
{
	using slot_type = enums::slot_type;

	rust::Box<CycleEntry> cycle_entry_from_form(RE::TESForm*& item_form);

	slot_type get_type(RE::TESForm*& item_form);
	bool is_two_handed(RE::TESForm*& item_form);
	bool can_instant_cast(RE::TESForm* item_form, slot_type item_type);

	EntryKind get_icon_type(const slot_type item_type, RE::TESForm*& item_form);

	EntryKind get_weapon_type_icon(RE::TESForm*& form);
	EntryKind get_spell_icon(RE::TESForm*& form);
	EntryKind get_consumable_icon(RE::TESForm*& form);
	EntryKind get_armor_icon(RE::TESForm*& form);
	EntryKind get_consumable_icon_by_actor_value(RE::ActorValue& actor_value);
}

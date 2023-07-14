#pragma once

#include "enums.h"

enum class EntryKind : ::std::uint8_t;

namespace equip
{
	void equip_item(const RE::TESForm* a_form,
		RE::BGSEquipSlot*& a_slot,
		RE::PlayerCharacter*& a_player,
		EntryKind kind);
	void equip_armor(const RE::TESForm* a_form, RE::PlayerCharacter*& a_player);
	void consume_potion(const RE::TESForm* a_form, RE::PlayerCharacter*& a_player);
	void equip_ammo(const RE::TESForm* a_form, RE::PlayerCharacter*& a_player);
	void unequip_ammo();
	void find_and_consume_fitting_option(RE::ActorValue a_actor_value, RE::PlayerCharacter*& a_player);
	void poison_weapon(RE::PlayerCharacter*& a_player, RE::AlchemyItem*& a_poison, uint32_t a_count);
}

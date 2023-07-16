#pragma once

#include "enums.h"

enum class EntryKind : ::std::uint8_t;

namespace equip
{
	void equipArmorByForm(const RE::TESForm* form, RE::PlayerCharacter*& player);
	// reurns true if anything was unequipped.
	bool unequipArmor(RE::TESBoundObject*& a_obj,
		RE::PlayerCharacter*& a_player,
		RE::ActorEquipManager*& a_actor_equip_manager);

	void equip_item(const RE::TESForm* a_form,
		RE::BGSEquipSlot*& a_slot,
		RE::PlayerCharacter*& a_player);
	void consume_potion(const RE::TESForm* a_form, RE::PlayerCharacter*& a_player);
	void equip_ammo(const RE::TESForm* a_form, RE::PlayerCharacter*& a_player);
	void unequip_ammo();
	void find_and_consume_fitting_option(RE::ActorValue a_actor_value, RE::PlayerCharacter*& a_player);
	void poison_weapon(RE::PlayerCharacter*& a_player, RE::AlchemyItem*& a_poison, uint32_t a_count);
}

#pragma once

#include "enums.h"

enum class Action : ::std::uint8_t;

// Equipping and unequipping armor and weapons.
namespace equip
{
	using action_type = enums::action_type;

	RE::BGSEquipSlot* right_hand_equip_slot();
	RE::BGSEquipSlot* left_hand_equip_slot();
	RE::BGSEquipSlot* power_equip_slot();

	void unequipHand(RE::PlayerCharacter*& player, Action which);

	// reurns true if anything was unequipped.
	bool unequipArmor(RE::TESBoundObject*& a_obj,
		RE::PlayerCharacter*& a_player,
		RE::ActorEquipManager*& a_actor_equip_manager);
	void unequip_slot(RE::BGSEquipSlot*& a_slot, RE::PlayerCharacter*& a_player, action_type a_action);
	void unequip_object_ft_dummy_dagger(RE::BGSEquipSlot*& a_slot,
		RE::PlayerCharacter*& a_player,
		RE::ActorEquipManager*& a_actor_equip_manager);
	// 0 - Left hand,  1 - Right hand, 2 Other

	bool is_item_worn(RE::TESBoundObject*& a_obj, RE::PlayerCharacter*& a_player);
}

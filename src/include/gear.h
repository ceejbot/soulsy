#pragma once

#include "enums.h"

namespace equip
{
	using action_type = enums::action_type;

	RE::BGSEquipSlot* right_hand_equip_slot();
	RE::BGSEquipSlot* left_hand_equip_slot();
	RE::BGSEquipSlot* power_equip_slot();
	// Turns true if anything was unequipped.
	bool unequip_amor(RE::TESBoundObject*& a_obj,
		RE::PlayerCharacter*& a_player,
		RE::ActorEquipManager*& a_actor_equip_manager);
	void unequip_slot(RE::BGSEquipSlot*& a_slot, RE::PlayerCharacter*& a_player, action_type a_action);
	void unequip_object_ft_dummy_dagger(RE::BGSEquipSlot*& a_slot,
		RE::PlayerCharacter*& a_player,
		RE::ActorEquipManager*& a_actor_equip_manager);
	// 0 - Left hand,  1 - Right hand, 2 Other
	void unequip_spell(RE::BSScript::IVirtualMachine* a_vm,
		RE::VMStackID a_stack_id,
		RE::Actor* a_actor,
		RE::SpellItem* a_spell,
		uint32_t a_slot);
	void unequip_shout(RE::BSScript::IVirtualMachine* a_vm,
		RE::VMStackID a_stack_id,
		RE::Actor* a_actor,
		RE::TESShout* a_shout);
	void un_equip_shout_slot(RE::PlayerCharacter*& a_player);

	bool is_item_worn(RE::TESBoundObject*& a_obj, RE::PlayerCharacter*& a_player);
}

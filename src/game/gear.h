#pragma once

enum class Action : ::std::uint8_t;

// Equipping and unequipping armor and weapons.
namespace equip
{
	RE::BGSEquipSlot* right_hand_equip_slot();
	RE::BGSEquipSlot* left_hand_equip_slot();
	RE::BGSEquipSlot* power_equip_slot();

	int boundObjectForForm(const RE::TESForm* form, RE::PlayerCharacter*& the_player, RE::TESBoundObject* outval);

	void unequipLeftOrRightSlot(RE::BGSEquipSlot*& a_slot, RE::PlayerCharacter*& a_player);
	void unequip_object_ft_dummy_dagger(RE::BGSEquipSlot*& a_slot,
		RE::PlayerCharacter*& a_player,
		RE::ActorEquipManager*& a_actor_equip_manager);
	// 0 - Left hand,  1 - Right hand, 2 Other

	bool is_item_worn(RE::TESBoundObject*& a_obj, RE::PlayerCharacter*& a_player);

	void equipShoutByForm(RE::TESForm* a_form, RE::PlayerCharacter*& a_player);
	void unequipShoutSlot(RE::PlayerCharacter*& a_player);

	void unequipHand(RE::PlayerCharacter*& player, Action which);

	// Implementation details.
	void unequip_spell(RE::BSScript::IVirtualMachine* a_vm,
		RE::VMStackID a_stack_id,
		RE::Actor* a_actor,
		RE::SpellItem* a_spell,
		uint32_t a_slot);
	void un_equip_shout(RE::BSScript::IVirtualMachine* a_vm,
		RE::VMStackID a_stack_id,
		RE::Actor* a_actor,
		RE::TESShout* a_shout);
}

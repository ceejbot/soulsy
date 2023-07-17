#pragma once

// Equipping and unequipping armor and weapons.
namespace game
{
	RE::BGSEquipSlot* right_hand_equip_slot();
	RE::BGSEquipSlot* left_hand_equip_slot();
	RE::BGSEquipSlot* power_equip_slot();

	int boundObjectForForm(const RE::TESForm* form,
		RE::PlayerCharacter*& the_player,
		RE::TESBoundObject*& outval,
		RE::ExtraDataList*& outextra);

	bool isItemWorn(RE::TESBoundObject*& a_obj, RE::PlayerCharacter*& a_player);
	// bottleneck for equipping everything
	void equipItemByFormAndSlot(const RE::TESForm* a_form, RE::BGSEquipSlot*& a_slot, RE::PlayerCharacter*& a_player);
}

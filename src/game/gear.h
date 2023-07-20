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

	bool isItemWorn(RE::TESBoundObject*& object, RE::PlayerCharacter*& the_player);
	// bottleneck for equipping everything
	void equipItemByFormAndSlot(RE::TESForm* form, RE::BGSEquipSlot*& slot, RE::PlayerCharacter*& the_player);
	void equipSpellByFormAndSlot(RE::TESForm* form, RE::BGSEquipSlot*& slot, RE::PlayerCharacter*& the_player);
}

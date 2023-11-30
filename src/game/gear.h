#pragma once

// Equipping and unequipping armor and weapons.

enum class Action : ::std::uint8_t;

namespace game
{
	RE::BGSEquipSlot* right_hand_equip_slot();
	RE::BGSEquipSlot* left_hand_equip_slot();
	RE::BGSEquipSlot* power_equip_slot();

	int boundObjectForForm(const RE::TESForm* form,
		RE::PlayerCharacter*& the_player,
		RE::TESBoundObject*& outval,
		RE::ExtraDataList*& outextra);
	bool inventoryEntryDataFor(const RE::TESForm* form, RE::TESBoundObject*& outobj, RE::InventoryEntryData*& outentry);

	bool isItemWorn(RE::TESBoundObject*& object, RE::PlayerCharacter*& the_player);
	bool isItemFavorited(const RE::TESForm* form);
	bool isItemPoisoned(const RE::TESForm* form);
	double itemChargeLevel(const RE::TESForm* form);

	// bottleneck for equipping everything
	void equipItemByFormAndSlot(RE::TESForm* form, RE::BGSEquipSlot*& slot, RE::PlayerCharacter*& the_player);
	void equipSpellByFormAndSlot(RE::TESForm* form, RE::BGSEquipSlot*& slot, RE::PlayerCharacter*& the_player);

	void unequipHand(RE::PlayerCharacter*& the_player, Action which);
	void unequipLeftOrRightSlot(RE::BGSEquipSlot*& slot, RE::PlayerCharacter*& the_player);
}

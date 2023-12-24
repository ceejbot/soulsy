#pragma once

// Equipping and unequipping armor and weapons, as well as answering questions
// about equipped gear.

#include <string>
#include "soulsy.h"

namespace game
{
	using namespace soulsy;

	// This struct holds useful information gleaned from item extra data,
	// for convenience when building hud items, equipping an item, or
	// unequipping it. If you make one, you are responsible for deleting it.
	struct EquippableItemData
	{
		int count        = 0;
		bool isWorn      = false;
		bool isWornLeft  = false;
		bool isFavorite  = false;
		bool isPoisoned  = false;
		std::string name = std::string("");
		// enchantment charge?

		RE::ExtraDataList* itemExtraList;

		RE::ExtraDataList* wornExtraList;
		RE::ExtraDataList* wornLeftExtraList;

		EquippableItemData();
	};

	// Ask the game for the right hand slot.
	RE::BGSEquipSlot* right_hand_equip_slot();
	// Ask the game for the left hand slot.
	RE::BGSEquipSlot* left_hand_equip_slot();
	// Ask the game for the shouts/powers slot.
	RE::BGSEquipSlot* power_equip_slot();

	// Find a bound object matching this form in the player's inventory. Caller must provide
	// pointers to bound object and extra data list references to receive found data. Returns
	// the number of such items the player has in their inventory.
	int boundObjectForForm(const RE::TESForm* form,
		RE::PlayerCharacter*& the_player,
		RE::TESBoundObject*& outval,
		EquippableItemData*& outdata);
	// Similar to boundObjectForForm(), but fills out an inventory entry instead of extra data lists.
	bool inventoryEntryDataFor(const RE::TESForm* form, RE::TESBoundObject*& outobj, RE::InventoryEntryData*& outentry);

	// Is the player wearing this item?
	bool isItemWorn(RE::TESBoundObject*& object, RE::PlayerCharacter*& the_player);
	// Is this item favorited? Probably doesn't work for spells, which are not inventory items.
	bool isItemFavorited(const RE::TESForm* form);
	// Is this weapon poisoned?
	bool isItemPoisoned(const RE::TESForm* form);
	// If this item is enchanted, what is its charge level? Or if a torch, what is its burn time?
	float itemChargeLevel(const RE::TESForm* form);
	// Get the display name for this item, looking up a player-set custom name if the item has one.
	const char* displayName(const RE::TESForm* form);

	// Equip a form in either the left or right hand. Handles weapons/shields directly, but delegates spells.
	void equipItemByFormAndSlot(RE::TESForm* form, RE::BGSEquipSlot*& slot, RE::PlayerCharacter*& the_player);
	// Equip a spell in either the left or right hand.
	void equipSpellByFormAndSlot(RE::TESForm* form, RE::BGSEquipSlot*& slot, RE::PlayerCharacter*& the_player);

	// Unequip the hand indicated by the shared enum.
	void unequipHand(RE::PlayerCharacter*& the_player, Action which);
	// Unequip the hand indicated by the game's slot data. If the item is a spell, equips and
	// then immediately unequips the dummy dagger item (if found) to make sure the item shown
	// in the hand is updated properly.
	void unequipLeftOrRightSlot(RE::PlayerCharacter*& the_player, RE::BGSEquipSlot*& slot);

}

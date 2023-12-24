#pragma once

#include "helpers.h"

#include "rust/cxx.h"
#include "soulsy.h"

namespace player
{
	std::map<RE::TESBoundObject*, std::pair<int, std::unique_ptr<RE::InventoryEntryData>>>
		getInventoryForType(RE::PlayerCharacter*& a_player, RE::FormType a_type);

	uint32_t getInventoryCountByForm(const RE::TESForm* a_form);
	uint32_t inventoryCount(const RE::TESForm* a_form, RE::FormType a_type, RE::PlayerCharacter*& a_player);

	// Here I start carving out an API that the rust controller can call to
	// manipulate things about the player, as well as ask questions of it.

	rust::String specEquippedLeft();
	rust::String specEquippedRight();
	rust::String specEquippedPower();
	rust::String specEquippedAmmo();
	rust::Vec<rust::String> getAmmoInventory();
	bool compare(RE::TESAmmo* left, RE::TESAmmo* right);

	rust::Box<EquippedData> getEquippedItems();

	rust::Vec<uint16_t> playerName();

	bool isInCombat();
	bool weaponsAreDrawn();
	bool hasRangedEquipped();

	void unequipSlot(Action slot);
	void unequipShout();
	void equipShout(const std::string& form_spec);
	bool has_shout(RE::Actor* a_actor, RE::TESShout* a_shout);
	void reequipHand(Action which, const std::string& form_spec);
	void toggleArmor(const std::string& form_spec);
	void equipArmor(const std::string& form_spec);
	void unequipSlotByShift(uint8_t shift);
	void equipMagic(const std::string& form_spec, Action slot);
	void equipWeapon(const std::string& form_spec, Action slot);
	void equipAmmo(const std::string& form_spec);

	void consumePotion(const std::string& form_spec);

	bool hasItemOrSpell(const std::string& form_spec);
	uint32_t itemCount(const std::string& form_spec);
	uint32_t staminaPotionCount();
	uint32_t healthPotionCount();
	uint32_t magickaPotionCount();

	void chooseMagickaPotion();
	void chooseHealthPotion();
	void chooseStaminaPotion();

	bool useCGOAltGrip();
}

#pragma once

#include "helpers.h"
#include "rust/cxx.h"

struct TesItemData;
enum class Action : ::std::uint8_t;
enum class EntryKind : ::std::uint8_t;

namespace player
{
	std::map<RE::TESBoundObject*, std::pair<int, std::unique_ptr<RE::InventoryEntryData>>>
		get_inventory(RE::PlayerCharacter*& a_player, RE::FormType a_type);
	bool has_shout(RE::Actor* a_actor, RE::TESShout* a_shout);
	void play_sound(RE::BGSSoundDescriptor* a_sound_descriptor_form, RE::PlayerCharacter*& a_player);

	uint32_t getInventoryCountByForm(const RE::TESForm* a_form);
	uint32_t inventoryCount(const RE::TESForm* a_form, RE::FormType a_type, RE::PlayerCharacter*& a_player);

	// Here I start carving out an API that the rust controller can call to
	// manipulate things about the player, as well as ask questions of it.

	rust::Box<TesItemData> equippedLeftHand();
	rust::Box<TesItemData> equippedRightHand();
	rust::Box<TesItemData> equippedPower();
	rust::Box<TesItemData> equippedAmmo();

	void unequipSlot(Action slot);
	void unequipShout();
	void equipShout(const std::string& form_spec);
	void equipArmor(const std::string& form_spec);
	void equipMagic(const std::string& form_spec, Action slot);
	void equipWeapon(const std::string& form_spec, Action slot);
	void equipAmmo(const std::string& form_spec);
	void consumePotion(const std::string& form_spec);
	bool playerHasItemOrSpell(const std::string& form_spec);
	void reequipLeftHand(const std::string& form_spec);

	void equip_item(const RE::TESForm* a_form,
		RE::BGSEquipSlot*& a_slot,
		RE::PlayerCharacter*& a_player,
		enums::slot_type a_type);
	void equip_armor(const RE::TESForm* a_form, RE::PlayerCharacter*& a_player);
	void equip_ammo(const RE::TESForm* a_form, RE::PlayerCharacter*& a_player);
	void unequip_ammo();
	void find_and_consume_fitting_option(RE::ActorValue a_actor_value, RE::PlayerCharacter*& a_player);
	void poison_weapon(RE::PlayerCharacter*& a_player, RE::AlchemyItem*& a_poison, uint32_t a_count);
}

#pragma once

#include "helpers.h"

#include "rust/cxx.h"

struct TesItemData;
enum class Action : ::std::uint8_t;
enum class TesItemKind : ::std::uint8_t;

namespace player
{
	std::map<RE::TESBoundObject*, std::pair<int, std::unique_ptr<RE::InventoryEntryData>>>
		getInventoryForType(RE::PlayerCharacter*& a_player, RE::FormType a_type);
	void play_sound(RE::BGSSoundDescriptor* a_sound_descriptor_form, RE::PlayerCharacter*& a_player);

	uint32_t getInventoryCountByForm(const RE::TESForm* a_form);
	uint32_t inventoryCount(const RE::TESForm* a_form, RE::FormType a_type, RE::PlayerCharacter*& a_player);

	// Here I start carving out an API that the rust controller can call to
	// manipulate things about the player, as well as ask questions of it.

	rust::Box<TesItemData> equippedLeftHand();
	rust::Box<TesItemData> equippedRightHand();
	rust::Box<TesItemData> equippedPower();
	rust::Box<TesItemData> equippedAmmo();
	rust::Box<TesItemData> boundObjectLeftHand();
	rust::Box<TesItemData> boundObjectRightHand();

	rust::String playerName();

	bool isInCombat();
	bool weaponsAreDrawn();

	void unequipSlot(Action slot);
	void unequipShout();
	void equipShout(const std::string& form_spec);
	bool has_shout(RE::Actor* a_actor, RE::TESShout* a_shout);
	void equipArmor(const std::string& form_spec);
	void equipMagic(const std::string& form_spec, Action slot);
	void equipWeapon(const std::string& form_spec, Action slot);
	void equipAmmo(const std::string& form_spec);
	void consumePotion(const std::string& form_spec);
	bool hasItemOrSpell(const std::string& form_spec);
	void reequipHand(Action which, const std::string& form_spec);

	void find_and_consume_fitting_option(RE::ActorValue a_actor_value, RE::PlayerCharacter*& a_player);
	void poison_weapon(RE::PlayerCharacter*& a_player, RE::AlchemyItem*& a_poison, uint32_t a_count);
}

#pragma once

enum class Action : ::std::uint8_t;

namespace game
{
	void unequipHand(RE::PlayerCharacter*& player, Action which);
	void unequipLeftOrRightSlot(RE::BGSEquipSlot*& a_slot, RE::PlayerCharacter*& a_player);
	void unequip_object_ft_dummy_dagger(RE::BGSEquipSlot*& a_slot,
		RE::PlayerCharacter*& a_player,
		RE::ActorEquipManager*& a_actor_equip_manager);
	// 0 - Left hand,  1 - Right hand, 2 Other
}

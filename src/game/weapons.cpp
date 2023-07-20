#include "weapons.h"

#include "lib.rs.h"
#include "offset.h"
#include "string_util.h"

namespace game
{
	RE::BGSEquipSlot* right_hand_equip_slot()
	{
		using func_t = decltype(&right_hand_equip_slot);
		const REL::Relocation<func_t> func{ REL::ID(offset::right_hand_equip_slot) };
		return func();
	}

	RE::BGSEquipSlot* left_hand_equip_slot()
	{
		using func_t = decltype(&left_hand_equip_slot);
		const REL::Relocation<func_t> func{ REL::ID(offset::left_hand_equip_slot) };
		return func();
	}

	RE::BGSEquipSlot* power_equip_slot()
	{
		using func_t = decltype(&power_equip_slot);
		const REL::Relocation<func_t> func{ REL::ID(offset::getPowerEquipSlot) };
		return func();
	}

	// ---------- right and left hands

	void unequipHand(RE::PlayerCharacter*& player, Action which)
	{
		RE::BGSEquipSlot* slot = nullptr;
		if (which == Action::Left) { slot = left_hand_equip_slot(); }
		else if (which == Action::Right) { slot = right_hand_equip_slot(); }
		else
		{
			logger::debug("somebody called unequipHand() with slot={};"sv, static_cast<uint8_t>(which));
			return;
		}
		unequipLeftOrRightSlot(slot, player);
	}

	void unequipLeftOrRightSlot(RE::BGSEquipSlot*& slot, RE::PlayerCharacter*& player)
	{
		auto* equip_manager = RE::ActorEquipManager::GetSingleton();
		auto* task = SKSE::GetTaskInterface();
		if (!task) {
			logger::warn("unable to get SKSE task interface! Cannot equip or unequip anything."sv);
			return;
		}

		auto* dummy = RE::TESForm::LookupByID<RE::TESForm>(0x00020163)->As<RE::TESObjectWEAP>();
		// no extra data, count 1, slot, queue false, force true, sound false, apply now defaults to false
		task->AddTask([=]() { equip_manager->EquipObject(player, dummy, nullptr, 1, slot, false, true, false); });
		task->AddTask([=]() { equip_manager->UnequipObject(player, dummy, nullptr, 1, slot, false, true, false); });
	}

	void unequip_object_ft_dummy_dagger(RE::BGSEquipSlot*& slot,
		RE::PlayerCharacter*& player,
		RE::ActorEquipManager*& equip_manager)
	{
		auto* dummy = RE::TESForm::LookupByID<RE::TESForm>(0x00020163)->As<RE::TESObjectWEAP>();
		// no extra data, count 1, slot, queue false, force true, sound false, apply now defaults to false
		equip_manager->EquipObject(player, dummy, nullptr, 1, slot, false, true, false);
		equip_manager->UnequipObject(player, dummy, nullptr, 1, slot, false, true, false);
	}

}

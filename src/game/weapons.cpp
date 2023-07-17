#include "weapons.h"

#include "string_util.h"
#include "offset.h"
#include "lib.rs.h"

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
		if (which == Action::Right) { slot = left_hand_equip_slot(); }
		else if (which == Action::Left) { slot = right_hand_equip_slot(); }
		else
		{
			logger::debug("somebody called unequipHand() with slot={};"sv, static_cast<uint8_t>(which));
			return;
		}
		unequipLeftOrRightSlot(slot, player);
	}

	void unequipLeftOrRightSlot(RE::BGSEquipSlot*& slot, RE::PlayerCharacter*& player)
	{
		bool did_call       = false;
		auto* equip_manager = RE::ActorEquipManager::GetSingleton();

		RE::TESForm* equipped_object = nullptr;
		if (slot == left_hand_equip_slot())
		{
			equipped_object = player->GetActorRuntimeData().currentProcess->GetEquippedLeftHand();
		}
		else if (slot == right_hand_equip_slot())
		{
			equipped_object = player->GetActorRuntimeData().currentProcess->GetEquippedRightHand();
		}
		if (!equipped_object) return;

		if (equipped_object->IsWeapon())
		{
			const auto weapon = equipped_object->As<RE::TESObjectWEAP>();
			equip_manager->UnequipObject(player, weapon, nullptr, 1, slot);
			did_call = true;
		}
		else if (equipped_object->Is(RE::FormType::Armor))
		{
			if (const auto armor = equipped_object->As<RE::TESObjectARMO>(); armor->IsShield())
			{
				equip_manager->UnequipObject(player, armor, nullptr, 1, slot);
				did_call = true;
			}
		}
		else if (equipped_object->Is(RE::FormType::Spell))
		{
			unequip_object_ft_dummy_dagger(slot, player, equip_manager);
			did_call = true;
		}
		else if (equipped_object->Is(RE::FormType::Light))
		{
			const auto light = equipped_object->As<RE::TESObjectLIGH>();
			equip_manager->UnequipObject(player, light, nullptr, 1, slot);
			did_call = true;
		}

		logger::trace("unequipped item from slot; item={}; did_call={};"sv, equipped_object->GetName(), did_call);
	}

	void unequip_object_ft_dummy_dagger(RE::BGSEquipSlot*& slot,
		RE::PlayerCharacter*& player,
		RE::ActorEquipManager*& equip_manager)
	{
		auto* dummy = RE::TESForm::LookupByID<RE::TESForm>(0x00020163)->As<RE::TESObjectWEAP>();
		//sound false, queue false, force true
		equip_manager->EquipObject(player, dummy, nullptr, 1, slot, false, true, false);
		equip_manager->UnequipObject(player, dummy, nullptr, 1, slot, false, true, false);
	}

}

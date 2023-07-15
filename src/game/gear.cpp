﻿#include "gear.h"
#include "offset.h"
#include "player.h"
#include "string_util.h"

#include "lib.rs.h"

namespace equip
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
		// I guess this is what we do when we don't have let-if.
		RE::TESForm* equipped_object = nullptr;
		RE::BGSEquipSlot* slot       = nullptr;

		if (which == Action::Right)
		{
			slot            = left_hand_equip_slot();
			equipped_object = player->GetActorRuntimeData().currentProcess->GetEquippedLeftHand();
		}
		else if (which == Action::Left)
		{
			slot            = right_hand_equip_slot();
			equipped_object = player->GetActorRuntimeData().currentProcess->GetEquippedRightHand();
		}
		else
		{
			logger::debug("somebody called unequipHand() with slot={};"sv, static_cast<uint8_t>(which));
			return;
		}

		if (!equipped_object)
		{
			return;
		}

		bool did_call       = false;
		auto* equip_manager = RE::ActorEquipManager::GetSingleton();
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

		logger::trace("unequippd item from slot; item={}; slot={}; did_call={};"sv,
			equipped_object->GetName(),
			static_cast<uint8_t>(which),
			did_call);
	}

	// used by utility_items.cpp -- still relevant?
	void unequip_slot(RE::BGSEquipSlot*& slot, RE::PlayerCharacter*& player, const action_type action)
	{
		if (action != action_type::un_equip)
		{
			return;
		}

		RE::TESForm* equipped_object = nullptr;
		if (slot == left_hand_equip_slot())
		{
			equipped_object = player->GetActorRuntimeData().currentProcess->GetEquippedLeftHand();
		}

		if (slot == right_hand_equip_slot())
		{
			equipped_object = player->GetActorRuntimeData().currentProcess->GetEquippedRightHand();
		}

		if (equipped_object)
		{
			logger::debug("Object {} is equipped, is left {}."sv,
				equipped_object->GetName(),
				slot == left_hand_equip_slot());
			bool did_call       = false;
			auto* equip_manager = RE::ActorEquipManager::GetSingleton();
			if (equipped_object->IsWeapon())
			{
				const auto weapon = equipped_object->As<RE::TESObjectWEAP>();
				equip_manager->UnequipObject(player, weapon, nullptr, 1, slot);
				did_call = true;
			}
			if (equipped_object->Is(RE::FormType::Armor))
			{
				if (const auto armor = equipped_object->As<RE::TESObjectARMO>(); armor->IsShield())
				{
					equip_manager->UnequipObject(player, armor, nullptr, 1, slot);
					did_call = true;
				}
			}

			if (equipped_object->Is(RE::FormType::Spell))
			{
				unequip_object_ft_dummy_dagger(slot, player, equip_manager);
				did_call = true;
			}

			if (equipped_object->Is(RE::FormType::Light))
			{
				const auto light = equipped_object->As<RE::TESObjectLIGH>();
				equip_manager->UnequipObject(player, light, nullptr, 1, slot);
				did_call = true;
			}

			logger::trace("called un equip for {}, left {}, did call {}"sv,
				equipped_object->GetName(),
				slot == left_hand_equip_slot(),
				did_call);
		}
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


	// ---------- Shouts & spells.

	void unequip_spell(RE::BSScript::IVirtualMachine* a_vm,
		RE::VMStackID a_stack_id,
		RE::Actor* a_actor,
		RE::SpellItem* a_spell,
		uint32_t slot)
	{
		using func_t = decltype(&unequip_spell);
		const REL::Relocation<func_t> func{ REL::ID(offset::get_un_equip_spell) };
		func(a_vm, a_stack_id, a_actor, a_spell, slot);
	}

	void un_equip_shout(RE::BSScript::IVirtualMachine* a_vm,
		RE::VMStackID a_stack_id,
		RE::Actor* a_actor,
		RE::TESShout* a_shout)
	{
		using func_t = decltype(&un_equip_shout);
		const REL::Relocation<func_t> func{ REL::ID(offset::get_un_equipShout) };
		func(a_vm, a_stack_id, a_actor, a_shout);
	}

	void unequipShoutSlot(RE::PlayerCharacter*& player)
	{
		auto* selected_power = player->GetActorRuntimeData().selectedPower;
		if (selected_power)
		{
			logger::trace("Equipped form is {}, try to un equip"sv,
				util::string_util::int_to_hex(selected_power->formID));
			if (selected_power->Is(RE::FormType::Shout))
			{
				un_equip_shout(nullptr, 0, player, selected_power->As<RE::TESShout>());
			}
			else if (selected_power->Is(RE::FormType::Spell))
			{
				//power
				//2=other
				unequip_spell(nullptr, 0, player, selected_power->As<RE::SpellItem>(), 2);
			}
		}
	}

	void equipShoutByForm(RE::TESForm* a_form, RE::PlayerCharacter*& a_player)
	{
		logger::trace("try to equip shout {}"sv, a_form->GetName());

		if (!a_form->Is(RE::FormType::Shout))
		{
			logger::warn("object {} is not a shout. return."sv, a_form->GetName());
			return;
		}

		if (const auto selected_power = a_player->GetActorRuntimeData().selectedPower; selected_power)
		{
			logger::trace("current selected power is {}, is shout {}, is spell {}"sv,
				selected_power->GetName(),
				selected_power->Is(RE::FormType::Shout),
				selected_power->Is(RE::FormType::Spell));
			if (selected_power->formID == a_form->formID)
			{
				logger::debug("no need to equip shout {}, it is already equipped. return."sv, a_form->GetName());
				return;
			}
		}

		auto* shout = a_form->As<RE::TESShout>();
		if (!player::has_shout(a_player, shout))
		{
			logger::warn("player does not have spell {}. return."sv, shout->GetName());
			return;
		}

		RE::ActorEquipManager::GetSingleton()->EquipShout(a_player, shout);
		logger::trace("equipped shout {}. return."sv, a_form->GetName());
	}

}

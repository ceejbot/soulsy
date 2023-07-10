#include "equip_slot.h"
#include "util/offset.h"
#include "util/player/player.h"
#include "util/string_util.h"

// 90% mlthelama with ceej naming

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
		using func_t = decltype(&getPowerEquipSlot);
		const REL::Relocation<func_t> func{ REL::ID(offset::getPowerEquipSlot) };
		return func();
	}

	bool unequip_amor(RE::TESBoundObject*& item, RE::PlayerCharacter*& player, RE::ActorEquipManager*& equip_manager)
	{
		const auto is_worn = is_item_worn(item, player);
		if (is_worn)
		{
			equip_manager->UnequipObject(player, item);
			logger::trace("unequipped {} armor"sv, item->GetName());
		}
		return is_worn;
	}

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

	void unequip_shout(RE::BSScript::IVirtualMachine* a_vm,
		RE::VMStackID a_stack_id,
		RE::Actor* a_actor,
		RE::TESShout* a_shout)
	{
		using func_t = decltype(&unequip_shout);
		const REL::Relocation<func_t> func{ REL::ID(offset::get_un_equip_shout) };
		func(a_vm, a_stack_id, a_actor, a_shout);
	}

	void un_equip_shout_slot(RE::PlayerCharacter*& player)
	{
		auto* selected_power = player->GetActorRuntimeData().selectedPower;
		if (selected_power)
		{
			logger::trace("Equipped form is {}, try to un equip"sv,
				util::string_util::int_to_hex(selected_power->formID));
			if (selected_power->Is(RE::FormType::Shout))
			{
				equip::unequip_shout(nullptr, 0, player, selected_power->As<RE::TESShout>());
			}
			else if (selected_power->Is(RE::FormType::Spell))
			{
				//power
				//2=other
				equip::unequip_spell(nullptr, 0, player, selected_power->As<RE::SpellItem>(), 2);
			}
		}
	}

	bool is_item_worn(RE::TESBoundObject*& item, RE::PlayerCharacter*& player)
	{
		auto worn = false;
		for (const auto& [item, inv_data] : player::get_inventory(player, RE::FormType::Armor))
		{
			if (const auto& [count, entry] = inv_data; entry->object->formID == item->formID && entry->IsWorn())
			{
				worn = true;
				break;
			}
		}
		return worn;
	}
}

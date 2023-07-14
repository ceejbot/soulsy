#include "gear.h"
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

	// ---------- armor

	bool unequipArmor(RE::TESBoundObject*& item, RE::PlayerCharacter*& player, RE::ActorEquipManager*& equip_manager)
	{
		const auto is_worn = is_item_worn(item, player);
		if (is_worn)
		{
			equip_manager->UnequipObject(player, item);
			logger::trace("unequipped {} armor"sv, item->GetName());
		}
		return is_worn;
	}

	void equipArmor(const std::string& form_spec)
	{
		auto RE::TESForm* form = helpers::get_form_from_mod_id_string(form_spec);
		if (!form)
		{
			return;
		}
		auto* player = RE::PlayerCharacter::GetSingleton();

		// Now do the work!
		logger::trace("attempting to equip armor; name='{}';"sv, form->GetName());

		RE::TESBoundObject* obj = nullptr;
		auto item_count         = 0;
		for (const auto& [item, inv_data] : player::get_inventory(a_player, RE::FormType::Armor))
		{
			if (const auto& [num_items, entry] = inv_data; entry->object->formID == a_form->formID)
			{
				obj        = entry->object;
				item_count = num_items;
				break;
			}
		}

		if (!obj || item_count == 0)
		{
			logger::warn("could not find armor in player inventory; name='{}';"sv, form->GetName());
			// TODO the armor is gone! inform the controller
			return;
		}

		if (auto* equip_manager = RE::ActorEquipManager::GetSingleton(); !unequipArmor(obj, player, equip_manager))
		{
			equip_manager->EquipObject(player, obj);
			logger::trace("successfully equipped armor; name='{}';"sv, a_form->GetName());
		}
	}

	bool is_item_worn(RE::TESBoundObject*& bound_obj, RE::PlayerCharacter*& player)
	{
		auto worn = false;
		for (const auto& [item, inv_data] : player::get_inventory(player, RE::FormType::Armor))
		{
			if (const auto& [count, entry] = inv_data; entry->object->formID == bound_obj->formID && entry->IsWorn())
			{
				worn = true;
				break;
			}
		}
		return worn;
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
			equipped_object = player->GetActorRuntimeData().currentProcess->GetequippedLeftHand();
		}
		else if (which == Action::Left)
		{
			auto slot       = right_hand_equip_slot();
			equipped_object = player->GetActorRuntimeData().currentProcess->GetEquippedRightHand();
		}
		else
		{
			logger::debug("somebody called unequipHand() with slot={};"sv, which);
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

		loger::trace("unequippd item from slot; item={}; slot={}; did_call={};"sv,
			equipped_object->GetName(),
			which,
			did_call);
	}

	// TODO remove
	void unequip_slot(RE::BGSEquipSlot*& slot, RE::PlayerCharacter*& player, const action_type action)
	{
		if (action != action_type::un_equip)
		{
			return;
		}

		RE::TESForm* equipped_object = nullptr;
		if (slot == left_hand_equip_slot())
		{
			equipped_object = player->GetActorRuntimeData().currentProcess->GetequippedLeftHand();
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

}

#include "utility_items.h"

#include "constant.h"
#include "gear.h"
#include "helpers.h"
#include "perk_visitor.h"
#include "player.h"
#include "string_util.h"

#include "lib.rs.h"

namespace equip
{
	// Bottleneck for equipping all left or right hand items.
	void equip_item(const RE::TESForm* form, RE::BGSEquipSlot*& slot, RE::PlayerCharacter*& player)
	{
		auto slot_is_left = slot == equip::left_hand_equip_slot();
		logger::trace("attempting to equip item in slot; name='{}'; is-left='{}'; type={};"sv,
			form->GetName(),
			left,
			form->GetFormType());

		if (form->formID == util::unarmed)
		{
			logger::trace("this slot should be unarmed; unequipping slot"sv);
			equip::unequipLeftOrRightSlot(slot, player);
			return;
		}

		RE::TESBoundObject* bound_obj = nullptr;
		auto item_count               = equip::boundObjectForForm(form, player, bound_obj);

		if (!bound_obj)
		{
			logger::warn("could not find selected weapon/shield, maybe it is gone"sv);
			return;
		}

		const auto* obj_right = player->GetActorRuntimeData().currentProcess->GetEquippedRightHand();
		const auto* obj_left  = player->GetActorRuntimeData().currentProcess->GetEquippedLeftHand();

		const auto obj_equipped_left  = obj_left && obj_left->formID == bound_obj->formID;
		const auto obj_equipped_right = obj_right && obj_right->formID == bound_obj->formID;

		if (slot_is_left && obj_equipped_left)
		{
			logger::debug("item already equipped in left hand. name='{}'"sv, bound_obj->GetName());
			return;
		}

		if (!slot_is_left && obj_equipped_right)
		{
			logger::debug("item already equipped in right hand. name='{}'"sv, bound_obj->GetName());
			return;
		}

		auto equipped_count = 0;
		if (obj_equipped_left) { equipped_count++; }
		if (obj_equipped_right) { equipped_count++; }
		logger::trace("checking how many '{}' we have available; count={}; equipped_count={}"sv,
			bound_obj->GetName(),
			item_count,
			equipped_count);

		if (item_count == equipped_count)
		{
			// The game might try to equip something else, according to mlthelama.
			equip::unequipLeftOrRightSlot(slot, player);
			return;
		}

		logger::trace("adding task to equip '{}'; left={};"sv, form->GetName(), slot_is_left);
		auto* task = SKSE::GetTaskInterface();
		if (task)
		{
			task->AddTask(
				[=]() { RE::ActorEquipManager::GetSingleton()->EquipObject(player, bound_obj, extra, 1, slot); });
		}
	}

	void consume_potion(const RE::TESForm* potion_form, RE::PlayerCharacter*& player)
	{
		logger::trace("consume_potion called; form_id=0x{}; potion='{}';"sv,
			util::string_util::int_to_hex(potion_form->formID),
			potion_form->GetName());

		RE::TESBoundObject* obj = nullptr;
		uint32_t remaining      = 0;
		for (auto potential_items = player::get_inventory(player, RE::FormType::AlchemyItem);
			 const auto& [item, inv_data] : potential_items)
		{
			if (const auto& [num_items, entry] = inv_data; entry->object->formID == potion_form->formID)
			{
				obj       = item;
				remaining = num_items;
				break;
			}
		}

		if (obj && obj->IsDynamicForm() && remaining == 1)
		{
			logger::warn(
				"The game crashes on potions with dynamic id if the count is 0 (happens with or without the mod). Skipping. formid=0x{};, name='{}';"sv,
				util::string_util::int_to_hex(obj->formID),
				obj->GetName());
			return;
		}

		if (!obj || remaining == 0)
		{
			logger::warn("could not find selected potion, maybe all have been consumed"sv);
			// TODO honk
			return;
		}

		if (!obj->Is(RE::FormType::AlchemyItem))
		{
			// TODO honk
			logger::warn("object {} is not an alchemy item. return."sv, obj->GetName());
			return;
		}

		auto* alchemy_item = obj->As<RE::AlchemyItem>();
		if (alchemy_item->IsPoison())
		{
			logger::trace("poison applied!"sv);
			poison_weapon(player, alchemy_item, remaining);
			return;
		}

		logger::trace("adding task to drink/eat potion/food; name='{}'; remaining={};"sv, obj->GetName(), remaining);
		auto* task = SKSE::GetTaskInterface();
		if (task)
		{
			task->AddTask([=]() { RE::ActorEquipManager::GetSingleton()->EquipObject(player, alchemy_item); });
		}
	}

	void equip_ammo(const RE::TESForm* a_form, RE::PlayerCharacter*& a_player)
	{
		logger::trace("try to equip {}"sv, a_form->GetName());

		RE::TESBoundObject* obj = nullptr;
		auto left               = 0;
		for (auto candidates = player::get_inventory(a_player, RE::FormType::Ammo);
			 const auto& [item, inv_data] : candidates)
		{
			if (const auto& [num_items, entry] = inv_data; entry->object->formID == a_form->formID)
			{
				obj       = item;
				remaining = num_items;
				break;
			}
		}

		if (!obj || remaining == 0)
		{
			logger::warn("ammo type not found in inventory! name='{}';"sv, a_form->GetName());
			return;
		}

		if (const auto* current_ammo = a_player->GetCurrentAmmo(); current_ammo && current_ammo->formID == obj->formID)
		{
			return;
		}

		logger::trace("adding task to equip ammo; name='';"sv, obj->GetName());
		auto* task = SKSE::GetTaskInterface();
		if (task)
		{
			task->AddTask([=]() { RE::ActorEquipManager::GetSingleton()->EquipObject(a_player, obj); });
		}
	}

	void unequip_ammo()
	{
		logger::debug("check if we need to un equip ammo"sv);
		auto player = RE::PlayerCharacter::GetSingleton();

		auto* obj = player->GetCurrentAmmo();
		if (!obj || !obj->IsAmmo()) { return; }

		auto* ammo = obj->As<RE::TESAmmo>();
		if (ammo->GetRuntimeData().data.flags.all(RE::AMMO_DATA::Flag::kNonBolt) ||
			ammo->GetRuntimeData().data.flags.none(RE::AMMO_DATA::Flag::kNonBolt))
		{
			RE::ActorEquipManager::GetSingleton()->UnequipObject(player, ammo);
			logger::trace("Called to un equip {}"sv, ammo->GetName());
		}
	}

	// TODO dry this up for sure
	void find_and_consume_fitting_option(RE::ActorValue a_actor_value, RE::PlayerCharacter*& a_player)
	{
		// get player missing value
		auto current_actor_value   = a_player->AsActorValueOwner()->GetActorValue(a_actor_value);
		auto permanent_actor_value = a_player->AsActorValueOwner()->GetPermanentActorValue(a_actor_value);
		auto temporary_actor_value =
			a_player->GetActorValueModifier(RE::ACTOR_VALUE_MODIFIER::kTemporary, RE::ActorValue::kHealth);
		auto max_actor_value = permanent_actor_value + temporary_actor_value;
		auto missing         = max_actor_value - current_actor_value;
		logger::trace("actor value {}, current {}, max {}, missing {}"sv,
			static_cast<int>(a_actor_value),
			fmt::format(FMT_STRING("{:.2f}"), current_actor_value),
			fmt::format(FMT_STRING("{:.2f}"), max_actor_value),
			fmt::format(FMT_STRING("{:.2f}"), missing));

		// min heal, max heal
		auto min_perfect = 0.7f;
		auto max_perfect = 1.2f;
		logger::trace("min perfect {}, max perfect {}, missing {}"sv,
			fmt::format(FMT_STRING("{:.2f}"), missing * min_perfect),
			fmt::format(FMT_STRING("{:.2f}"), missing * max_perfect),
			fmt::format(FMT_STRING("{:.2f}"), missing));

		RE::TESBoundObject* obj = nullptr;
		for (auto potential_items = player::get_inventory(a_player, RE::FormType::AlchemyItem);
			 const auto& [item, inv_data] : potential_items)
		{
			const auto& [num_items, entry] = inv_data;

			auto* alchemy_item = item->As<RE::AlchemyItem>();
			if (alchemy_item->IsPoison() || alchemy_item->IsFood()) { continue; }
			// returns currently only the types we want
			auto actor_value = equippable::getPotionEffect(item, true);
			if (actor_value == RE::ActorValue::kNone) { continue; }

			if (actor_value == a_actor_value)
			{
				// set obj here, because if we do not have a perfect hit, we still need to
				// consume something
				if (alchemy_item->IsDynamicForm() && num_items == 1)
				{
					logger::warn(
						"Somehow the game crashes on potions with dynamic id if the count is 0 (happens with or without the mod). So I am not consuming it. Form {}, Name {}"sv,
						util::string_util::int_to_hex(alchemy_item->formID),
						alchemy_item->GetName());
					continue;
				}
				obj = alchemy_item;

				auto magnitude = alchemy_item->GetCostliestEffectItem()->GetMagnitude();
				auto duration  = alchemy_item->GetCostliestEffectItem()->GetDuration();
				if (duration == 0) { duration = 1; }

				auto max_healed = magnitude * duration;
				if (max_healed >= (missing * min_perfect) && max_healed <= (missing * max_perfect))
				{
					logger::trace("found potion {}, magnitude * duration {}"sv,
						obj->GetName(),
						fmt::format(FMT_STRING("{:.2f}"), max_healed));
					break;
				}
			}
		}

		if (obj)
		{
			logger::trace("calling to consume potion {}"sv, obj->GetName());
			consume_potion(obj, a_player);
		}
		else { logger::warn("No suitable potion found. return."); }
	}

	void poison_weapon(RE::PlayerCharacter*& a_player, RE::AlchemyItem*& a_poison, uint32_t a_count)
	{
		logger::trace("try to apply poison to weapon, count left {}"sv, a_count);
		uint32_t potion_doses = 1;
		/* it works for vanilla and adamant
* vanilla does a basic set value to 3
* adamant does 2 times add value 2
* ordinator we could handle it "dirty" because the Information we need needs to
be RE, but if perk xy Is set we could calculate it ourselves. It is basically AV
multiply base + "alchemy level" * 0.1 * 3 = dose count
* vokrii should be fine as well
* other add av multiply implementations need to be handled by getting the data
from the game
* the MCM setting will be left for overwrite handling */
		if (a_player->HasPerkEntries(RE::BGSEntryPoint::ENTRY_POINTS::kModPoisonDoseCount))
		{
			auto perk_visit = util::perk_visitor(a_player, static_cast<float>(potion_doses));
			a_player->ForEachPerkEntry(RE::BGSEntryPoint::ENTRY_POINTS::kModPoisonDoseCount, perk_visit);
			potion_doses = static_cast<int>(perk_visit.get_result());
		}
		logger::trace("Poison dose set value is {}"sv, potion_doses);

		RE::BGSSoundDescriptor* sound_descriptor;
		if (a_poison->data.consumptionSound) { sound_descriptor = a_poison->data.consumptionSound->soundDescriptor; }
		else
		{
			sound_descriptor = RE::TESForm::LookupByID(0x00106614)->As<RE::BGSSoundDescriptorForm>()->soundDescriptor;
		}

		// check if there is a weapon to apply it to
		// check count here as well, since we need max 2
		auto* equipped_object = a_player->GetEquippedEntryData(false);
		if (equipped_object && equipped_object->object->IsWeapon() && !equipped_object->IsPoisoned())
		{
			logger::trace("try to add poison {} to right {}"sv, a_poison->GetName(), equipped_object->GetDisplayName());
			equipped_object->PoisonObject(a_poison, potion_doses);
			player::play_sound(sound_descriptor, a_player);
			a_player->RemoveItem(a_poison, 1, RE::ITEM_REMOVE_REASON::kRemove, nullptr, nullptr);
			a_count--;
		}

		auto* equipped_object_left = a_player->GetEquippedEntryData(true);
		if (equipped_object_left && equipped_object_left->object->IsWeapon() && !equipped_object_left->IsPoisoned() &&
			a_count > 0)
		{
			logger::trace(
				"try to add poison {} to left {}"sv, a_poison->GetName(), equipped_object_left->GetDisplayName());
			equipped_object_left->PoisonObject(a_poison, potion_doses);
			a_player->RemoveItem(a_poison, 1, RE::ITEM_REMOVE_REASON::kRemove, nullptr, nullptr);
			a_count--;
		}
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

	void equipArmorByForm(const RE::TESForm* form, RE::PlayerCharacter*& player)
	{
		logger::trace("attempting to equip armor; name='{}';"sv, form->GetName());

		RE::TESBoundObject* obj = nullptr;
		auto item_count         = 0;
		for (const auto& [item, inv_data] : player::get_inventory(player, RE::FormType::Armor))
		{
			if (const auto& [num_items, entry] = inv_data; entry->object->formID == form->formID)
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
			logger::trace("successfully equipped armor; name='{}';"sv, form->GetName());
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

}  // namespace equip

#include "utility.h"

#include "constant.h"
#include "equippable.h"
#include "gear.h"
#include "helpers.h"
#include "player.h"
#include "string_util.h"

#include "lib.rs.h"

namespace game
{
	using string_util = util::string_util;

	// ---------- ammo

	void equipAmmoByForm(const RE::TESForm* form, RE::PlayerCharacter*& player)
	{
		logger::debug(
			"trying to equip ammo; name='{}'; formID={}"sv, form->GetName(), string_util::int_to_hex(form->formID));

		RE::TESBoundObject* obj  = nullptr;
		RE::ExtraDataList* extra = nullptr;
		auto remaining           = boundObjectForForm(form, player, obj, extra);

		if (!obj || remaining == 0)
		{
			logger::warn("ammo not found in inventory! name='{}';"sv, form->GetName());
			return;
		}

		if (const auto* current_ammo = player->GetCurrentAmmo(); current_ammo && current_ammo->formID == obj->formID)
		{
			logger::trace("ammo is already equipped; bound formID={}"sv, string_util::int_to_hex(obj->formID));
			return;
		}

		logger::trace("queuing task to equip ammo; name='{}'; bound formID={}"sv,
			obj->GetName(),
			string_util::int_to_hex(obj->formID));
		auto* task = SKSE::GetTaskInterface();
		if (task)
		{
			task->AddTask([=]() { RE::ActorEquipManager::GetSingleton()->EquipObject(player, obj); });
		}
	}

	void unequipCurrentAmmo()
	{
		logger::debug("unequipping current ammo if needed"sv);
		auto player = RE::PlayerCharacter::GetSingleton();

		auto* obj = player->GetCurrentAmmo();
		if (!obj || !obj->IsAmmo()) { return; }

		auto* ammo = obj->As<RE::TESAmmo>();
		if (ammo->GetRuntimeData().data.flags.all(RE::AMMO_DATA::Flag::kNonBolt) ||
			ammo->GetRuntimeData().data.flags.none(RE::AMMO_DATA::Flag::kNonBolt))
		{
			auto* task = SKSE::GetTaskInterface();
			if (task)
			{
				task->AddTask([=]() { RE::ActorEquipManager::GetSingleton()->UnequipObject(player, ammo); });
			}
			logger::debug("ammo unequipped; name='{}'; formID={}"sv,
				ammo->GetName(),
				util::string_util::int_to_hex(ammo->formID));
		}
	}

	// ---------- armor

	bool unequipArmor(RE::TESBoundObject*& item, RE::PlayerCharacter*& player, RE::ActorEquipManager*& equip_manager)
	{
		const auto is_worn = isItemWorn(item, player);
		if (is_worn)
		{
			auto* task = SKSE::GetTaskInterface();
			if (task)
			{
				task->AddTask([=]() { equip_manager->UnequipObject(player, item); });
			}
			logger::trace("unequipped armor; name='{}';"sv, item->GetName());
		}
		return is_worn;
	}

	void equipArmorByForm(const RE::TESForm* form, RE::PlayerCharacter*& player)
	{
		// This is a toggle in reality. Also, use this as a model for other equip funcs.
		logger::trace("attempting to equip armor; name='{}';"sv, form->GetName());
		RE::TESBoundObject* obj  = nullptr;
		RE::ExtraDataList* extra = nullptr;
		auto remaining           = boundObjectForForm(form, player, obj, extra);

		if (!obj || remaining == 0)
		{
			logger::warn("could not find armor in player inventory; name='{}';"sv, form->GetName());
			return;
		}

		auto* task = SKSE::GetTaskInterface();
		if (!task)
		{
			logger::warn("could not find SKSE task interface! Cannot act."sv);
			return;
		}

		const auto is_worn  = isItemWorn(obj, player);
		auto* equip_manager = RE::ActorEquipManager::GetSingleton();
		if (is_worn)
		{
			task->AddTask([=]() { equip_manager->UnequipObject(player, obj, extra); });
		}
		else
		{
			task->AddTask([=]() { equip_manager->EquipObject(player, obj, extra); });
		}
	}

	// ---------- potions

	void consumePotion(const RE::TESForm* potion_form, RE::PlayerCharacter*& player)
	{
		logger::trace("consumePotion called; form_id={}; potion='{}';"sv,
			util::string_util::int_to_hex(potion_form->formID),
			potion_form->GetName());

		RE::TESBoundObject* obj  = nullptr;
		RE::ExtraDataList* extra = nullptr;
		auto remaining           = boundObjectForForm(potion_form, player, obj, extra);

		if (!obj || remaining == 0)
		{
			logger::warn("couldn't find requested potion in inventory!"sv);
			// TODO honk
			return;
		}

		if (obj->IsDynamicForm() && remaining == 1)
		{
			logger::warn(
				"The game crashes on potions with dynamic id if the count is 0 (happens with or without the mod). Skipping. formID={};, name='{}';"sv,
				string_util::int_to_hex(obj->formID),
				obj->GetName());
			return;
		}

		if (!obj->Is(RE::FormType::AlchemyItem))
		{
			// TODO honk
			logger::warn("bound object is not an alchemy item? name='{}'; formID={};"sv,
				obj->GetName(),
				string_util::int_to_hex(obj->formID));
			return;
		}

		auto* alchemy_item = obj->As<RE::AlchemyItem>();
		if (alchemy_item->IsPoison())
		{
			poison_weapon(player, alchemy_item, remaining);
			return;
		}

		// Let's get the consumable's sound.
		RE::BGSSoundDescriptor* sound;
		if (alchemy_item->data.consumptionSound) { sound = alchemy_item->data.consumptionSound->soundDescriptor; }
		else
		{
			sound = RE::TESForm::LookupByID(0x000b6435)->As<RE::BGSSoundDescriptorForm>()->soundDescriptor;
		}

		logger::trace("queuing task to use consumable; name='{}'; remaining={}; formID={};"sv,
			obj->GetName(),
			remaining,
			string_util::int_to_hex(obj->formID));
		auto* task = SKSE::GetTaskInterface();
		if (task)
		{
			task->AddTask([=]() { RE::ActorEquipManager::GetSingleton()->EquipObject(player, alchemy_item, extra); });
			player::play_sound(sound, player);
		}
	}

	void poison_weapon(RE::PlayerCharacter*& the_player, RE::AlchemyItem*& a_poison, uint32_t a_count)
	{
		logger::trace("try to apply poison to weapon, inventory count={}"sv, a_count);
		uint32_t poison_doses = 1;
		/*
comment preserved from mlthelama
it works for vanilla and adamant
vanilla does a basic set value to 3
adamant does 2 times add value 2
ordinator we could handle it "dirty" because the Information we need needs to
be RE, but if perk xy Is set we could calculate it ourselves. It is basically AV
multiply base + "alchemy level" * 0.1 * 3 = dose count
vokrii should be fine as well
other add av multiply implementations need to be handled by getting the data
from the game
the MCM setting will be left for overwrite handling
*/
		if (the_player->HasPerkEntries(RE::BGSEntryPoint::ENTRY_POINTS::kModPoisonDoseCount))
		{
			auto perk_visit = perk_visitor(the_player, static_cast<float>(poison_doses));
			the_player->ForEachPerkEntry(RE::BGSEntryPoint::ENTRY_POINTS::kModPoisonDoseCount, perk_visit);
			poison_doses = static_cast<int>(perk_visit.get_result());
		}
		logger::trace("poison doses read from perks; poison_doses={};"sv, poison_doses);

		RE::BGSSoundDescriptor* sound_descriptor;
		if (a_poison->data.consumptionSound) { sound_descriptor = a_poison->data.consumptionSound->soundDescriptor; }
		else
		{
			sound_descriptor = RE::TESForm::LookupByID(0x00106614)->As<RE::BGSSoundDescriptorForm>()->soundDescriptor;
		}

		auto used             = 0;
		auto* equipped_object = the_player->GetEquippedEntryData(false);
		if (equipped_object && equipped_object->object->IsWeapon() && !equipped_object->IsPoisoned() && a_count > 0)
		{
			logger::trace("about to poison right-hand weapon; poison='{}'; weapon='{}';"sv,
				a_poison->GetName(),
				equipped_object->GetDisplayName());
			equipped_object->PoisonObject(a_poison, poison_doses);
			// We only play the sound once, even if we also dose the left-hand item.
			player::play_sound(sound_descriptor, the_player);
			used++;
			a_count--;
		}

		logger::trace("now considering left-hand item."sv);
		auto* equipped_object_left = the_player->GetEquippedEntryData(true);
		if (equipped_object_left && equipped_object_left->object->IsWeapon() && !equipped_object_left->IsPoisoned() &&
			a_count > 0)
		{
			logger::trace("about to poison left-hand weapon; poison='{}'; weapon='{}';"sv,
				a_poison->GetName(),
				equipped_object_left->GetDisplayName());
			equipped_object_left->PoisonObject(a_poison, poison_doses);
			used++;
		}
		auto* task = SKSE::GetTaskInterface();
		if (task)
		{
			logger::trace("queuing remove item tasks..."sv);
			task->AddTask(
				[=]() { the_player->RemoveItem(a_poison, used, RE::ITEM_REMOVE_REASON::kRemove, nullptr, nullptr); });
		}
	}

	// Not yet in use. Remove if I decide to definitely never do a choose-consumable feature.
	void find_and_consume_fitting_option(RE::ActorValue a_actor_value, RE::PlayerCharacter*& the_player)
	{
		// get player missing value
		auto current_actor_value   = the_player->AsActorValueOwner()->GetActorValue(a_actor_value);
		auto permanent_actor_value = the_player->AsActorValueOwner()->GetPermanentActorValue(a_actor_value);
		auto temporary_actor_value =
			the_player->GetActorValueModifier(RE::ACTOR_VALUE_MODIFIER::kTemporary, RE::ActorValue::kHealth);
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
		for (auto candidates = player::getInventoryForType(the_player, RE::FormType::AlchemyItem);
			 const auto& [item, inv_data] : candidates)
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
			logger::trace("Calling consume potion; name='{}';"sv, obj->GetName());
			consumePotion(obj, the_player);
		}
		else { logger::warn("No suitable potion found. return."); }
	}

	// ---------- perk visitor, used only by the actor value potion selection

	RE::PerkEntryVisitor::ReturnType perk_visitor::Visit(RE::BGSPerkEntry* perk_entry)
	{
		const auto* entry_point = static_cast<RE::BGSEntryPointPerkEntry*>(perk_entry);
		const auto* perk        = entry_point->perk;

		logger::trace("perk formID={}; name='{}';"sv, string_util::int_to_hex(perk->formID), perk->GetName());

		if (entry_point->functionData)
		{
			const RE::BGSEntryPointFunctionDataOneValue* value =
				static_cast<RE::BGSEntryPointFunctionDataOneValue*>(entry_point->functionData);
			if (entry_point->entryData.function == RE::BGSEntryPointPerkEntry::EntryData::Function::kSetValue)
			{
				result_ = value->data;
			}
			else if (entry_point->entryData.function == RE::BGSEntryPointPerkEntry::EntryData::Function::kAddValue)
			{
				result_ += value->data;
			}
			else if (entry_point->entryData.function == RE::BGSEntryPointPerkEntry::EntryData::Function::kMultiplyValue)
			{
				result_ *= value->data;
			}
			else if (entry_point->entryData.function ==
					 RE::BGSEntryPointPerkEntry::EntryData::Function::kAddActorValueMult)
			{
				if (perk_entry->GetFunction() == RE::BGSPerkEntry::EntryPoint::kModPoisonDoseCount)
				{
					auto av = actor_->AsActorValueOwner()->GetActorValue(RE::ActorValue::kAlchemy);
					result_ += static_cast<float>(av * 0.1 * 3);
				}
			}

			logger::trace("Got value {} for Perk, total now is {}"sv, value->data, result_);
		}

		return ReturnType::kContinue;
	}

	float perk_visitor::get_result() const { return result_; }

}  // namespace game

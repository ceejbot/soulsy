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
		RE::TESBoundObject* obj  = nullptr;
		RE::ExtraDataList* extra = nullptr;
		auto remaining           = boundObjectForForm(form, player, obj, extra);

		if (!obj || remaining == 0)
		{
			logger::warn("Ammo not found in inventory! name='{}';"sv, form->GetName());
			return;
		}

		if (const auto* current_ammo = player->GetCurrentAmmo(); current_ammo && current_ammo->formID == obj->formID)
		{
			logger::trace("ammo is already equipped; bound formID={}"sv, string_util::int_to_hex(obj->formID));
			return;
		}

		logger::debug("queuing task to equip ammo; name='{}'; bound formID={}"sv,
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

	void toggleArmorByForm(const RE::TESForm* form, RE::PlayerCharacter*& player)
	{
		// This is a toggle in reality. Also, use this as a model for other equip funcs.
		logger::trace("attempting to toggle armor; name='{}';"sv, form->GetName());
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

	void equipArmorByForm(const RE::TESForm* form, RE::PlayerCharacter*& player)
	{
		logger::trace("attempting to equip armor; name='{}';"sv, form->GetName());
		RE::TESBoundObject* obj  = nullptr;
		RE::ExtraDataList* extra = nullptr;
		auto remaining           = boundObjectForForm(form, player, obj, extra);

		if (!obj || remaining == 0)
		{
			logger::warn("could not find armor in player inventory; name='{}';"sv, form->GetName());
			return;
		}

		const auto is_worn = isItemWorn(obj, player);
		if (!is_worn)
		{
			auto* task          = SKSE::GetTaskInterface();
			auto* equip_manager = RE::ActorEquipManager::GetSingleton();
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
			helpers::honk();
			return;
		}

		if (!obj->Is(RE::FormType::AlchemyItem))
		{
			helpers::honk();
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

		logger::trace("queuing task to use consumable; name='{}'; remaining={}; formID={};"sv,
			obj->GetName(),
			remaining,
			string_util::int_to_hex(obj->formID));
		auto* task = SKSE::GetTaskInterface();
		if (task)
		{
			task->AddTask([=]() { RE::ActorEquipManager::GetSingleton()->EquipObject(player, alchemy_item, extra); });
		}
	}

	void poison_weapon(RE::PlayerCharacter*& thePlayer, RE::AlchemyItem*& a_poison, uint32_t a_count)
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
		if (thePlayer->HasPerkEntries(RE::BGSEntryPoint::ENTRY_POINTS::kModPoisonDoseCount))
		{
			auto perk_visit = perk_visitor(thePlayer, static_cast<float>(poison_doses));
			thePlayer->ForEachPerkEntry(RE::BGSEntryPoint::ENTRY_POINTS::kModPoisonDoseCount, perk_visit);
			poison_doses = static_cast<int>(perk_visit.get_result());
		}
		logger::trace("poison doses read from perks; poison_doses={};"sv, poison_doses);

		RE::BGSSoundDescriptor* sound_descriptor = nullptr;
		if (a_poison->data.consumptionSound) { sound_descriptor = a_poison->data.consumptionSound->soundDescriptor; }
		else
		{
			auto* form = RE::TESForm::LookupByID(0x00106614);
			if (form) { sound_descriptor = form->As<RE::BGSSoundDescriptorForm>()->soundDescriptor; }
		}

		auto used             = 0;
		auto* equipped_object = thePlayer->GetEquippedEntryData(false);
		if (equipped_object && equipped_object->object->IsWeapon() && !equipped_object->IsPoisoned() && a_count > 0)
		{
			logger::trace("about to poison right-hand weapon; poison='{}'; weapon='{}';"sv,
				a_poison->GetName(),
				equipped_object->GetDisplayName());
			equipped_object->PoisonObject(a_poison, poison_doses);
			// We only play the sound once, even if we also dose the left-hand item.
			if (sound_descriptor) { player::play_sound(sound_descriptor, thePlayer); }
			used++;
			a_count--;
		}

		logger::trace("now considering left-hand item."sv);
		auto* equipped_object_left = thePlayer->GetEquippedEntryData(true);
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
				[=]() { thePlayer->RemoveItem(a_poison, used, RE::ITEM_REMOVE_REASON::kRemove, nullptr, nullptr); });
		}
	}

	// ---------- potion selection

	const static float MIN_PERFECT = 0.7f;
	const static float MAX_PERFECT = 1.2f;

	void consumeBestOption(RE::ActorValue vitalStat)
	{
		auto* thePlayer = RE::PlayerCharacter::GetSingleton();
		if (!thePlayer) return;

		auto current         = thePlayer->AsActorValueOwner()->GetActorValue(vitalStat);
		auto permanent       = thePlayer->AsActorValueOwner()->GetPermanentActorValue(vitalStat);
		auto temporary       = thePlayer->GetActorValueModifier(RE::ACTOR_VALUE_MODIFIER::kTemporary, vitalStat);
		auto max_actor_value = permanent + temporary;
		auto deficit         = max_actor_value - current;
		auto goalMin         = deficit * MIN_PERFECT;
		auto goalMax         = deficit * MAX_PERFECT;

		if (deficit == 0)
		{
			logger::info("Not drinking a {} potion because you don't need one."sv, vitalStat);
			helpers::honk();
			return;
		}

		logger::debug("goal potion: deficit={}; min={}; max={};"sv,
			fmt::format(FMT_STRING("{:.2f}"), deficit),
			fmt::format(FMT_STRING("{:.2f}"), goalMin),
			fmt::format(FMT_STRING("{:.2f}"), goalMax));

		RE::TESBoundObject* obj = nullptr;
		float prevRating        = -100.0f;

		auto candidates = player::getInventoryForType(thePlayer, RE::FormType::AlchemyItem);
		logger::debug("{} potions in inventory"sv, candidates.size(), vitalStat);
		auto count = 0;
		for (const auto& [item, inv_data] : candidates)
		{
			const auto& [num_items, entry] = inv_data;

			auto* alchemy_item = item->As<RE::AlchemyItem>();
			if (alchemy_item->IsPoison() || alchemy_item->IsFood()) { continue; }
			auto actor_value = equippable::getPotionEffect(item, true);
			if (actor_value == RE::ActorValue::kNone) { continue; }
			if (actor_value != vitalStat) { continue; }

			// this potion might be useful
			count++;
			auto magnitude = alchemy_item->GetCostliestEffectItem()->GetMagnitude();
			auto duration  = alchemy_item->GetCostliestEffectItem()->GetDuration();
			if (duration == 0) { duration = 1; }
			auto max_restored = magnitude * duration;
			auto diff         = std::fabs(max_restored - deficit);
			auto rating       = max_restored > deficit ? diff : -diff;

			if (!obj)
			{
				// any match is better than no match
				obj        = alchemy_item;
				prevRating = rating;
				logger::debug("found at least one {} potion: rating={}; max_restored={}; deficit={};"sv,
					vitalStat,
					rating,
					max_restored,
					deficit);
				if (rating == 0) break;  // this item is perfect already
				continue;
			}

			// We have at least a second candidate. Is it better than our current choice?
			if (std::fabs(rating) < std::fabs(prevRating))
			{
				logger::debug(
					"improved selection: rating={}; max_restored={}; deficit={};"sv, rating, max_restored, deficit);
				obj        = alchemy_item;
				prevRating = rating;
				if (rating == 0) break;  // perfection
				continue;
			}
		}

		if (obj)
		{
			logger::debug("after considering {} candidates, found a potion: rating={}; name='{}';"sv,
				vitalStat,
				prevRating,
				obj->GetName());
			auto* task = SKSE::GetTaskInterface();
			if (task)
			{
				task->AddTask([=]() { RE::ActorEquipManager::GetSingleton()->EquipObject(thePlayer, obj); });
			}
		}
		else
		{
			logger::warn("We couldn't find any {} potions!"sv, vitalStat);
			helpers::honk();
		}
	}

	// ---------- perk visitor, used only by the actor value potion selection

	RE::BSContainer::ForEachResult perk_visitor::Visit(RE::BGSPerkEntry* perk_entry)
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

		return RE::BSContainer::ForEachResult::kContinue;
	}

	float perk_visitor::get_result() const { return result_; }

}  // namespace game

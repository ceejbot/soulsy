#include "utility.h"

#include "RE/A/Actor.h"
#include "constant.h"
#include "equippable.h"
#include "gear.h"
#include "helpers.h"
#include "player.h"

#include "lib.rs.h"

using namespace soulsy;

namespace utility
{
	// ---------- ammo

	void equipAmmoByForm(const RE::TESForm* form, RE::PlayerCharacter*& thePlayer)
	{
		RE::TESBoundObject* obj      = nullptr;
		RE::ExtraDataList* extraData = nullptr;
		auto remaining               = gear::boundObjectForForm(form, obj, extraData);

		if (!obj || remaining == 0)
		{
			rlog::warn("Ammo not found in inventory! name='{}';"sv, helpers::nameAsUtf8(form));
			return;
		}

		if (const auto* current_ammo = thePlayer->GetCurrentAmmo(); current_ammo && current_ammo->formID == obj->formID)
		{
			// rlog::trace("ammo is already equipped; bound formID={:#08x}"sv, obj->formID);
			return;
		}

		rlog::debug(
			"queuing task to equip ammo; name='{}'; bound formID={:#08x};"sv, helpers::nameAsUtf8(obj), obj->formID);
		auto* task = SKSE::GetTaskInterface();
		if (task)
		{
			task->AddTask([=]() { RE::ActorEquipManager::GetSingleton()->EquipObject(thePlayer, obj); });
		}
	}

	void unequipCurrentAmmo()
	{
		auto thePlayer = RE::PlayerCharacter::GetSingleton();

		auto* obj = thePlayer->GetCurrentAmmo();
		if (!obj || !obj->IsAmmo()) { return; }

		auto* ammo = obj->As<RE::TESAmmo>();
		if (ammo->GetRuntimeData().data.flags.all(RE::AMMO_DATA::Flag::kNonBolt) ||
			ammo->GetRuntimeData().data.flags.none(RE::AMMO_DATA::Flag::kNonBolt))
		{
			auto* task = SKSE::GetTaskInterface();
			if (task)
			{
				task->AddTask([=]() { RE::ActorEquipManager::GetSingleton()->UnequipObject(thePlayer, ammo); });
			}
			rlog::debug("ammo unequipped; name='{}'; formID={:#08x};"sv, helpers::nameAsUtf8(ammo), ammo->formID);
		}
	}

	// ---------- armor

	bool unequipArmor(RE::TESBoundObject*& item, RE::PlayerCharacter*& thePlayer, RE::ActorEquipManager*& equipManager)
	{
		const auto isWorn = gear::isItemWorn(item, thePlayer);
		if (isWorn)
		{
			auto* task = SKSE::GetTaskInterface();
			if (task)
			{
				task->AddTask([=]() { equipManager->UnequipObject(thePlayer, item); });
			}
			// rlog::trace("unequipped armor; name='{}';"sv, helpers::nameAsUtf8(item));
		}
		return isWorn;
	}

	void toggleArmorByForm(const RE::TESForm* form, RE::PlayerCharacter*& thePlayer, const std::string& nameToMatch)
	{
		// This is a toggle in reality. Also, use this as a model for other equip funcs.
		// rlog::trace("attempting to toggle armor; name='{}';"sv, helpers::nameAsUtf8(form));
		RE::TESBoundObject* obj      = nullptr;
		RE::ExtraDataList* extraData = nullptr;
		auto remaining               = gear::boundObjectMatchName(form, nameToMatch, obj, extraData);

		if (!obj || remaining == 0)
		{
			rlog::warn("could not find armor in player inventory; name='{}';"sv, nameToMatch);
			return;
		}

		auto* task         = SKSE::GetTaskInterface();
		auto* equipManager = RE::ActorEquipManager::GetSingleton();
		const auto isWorn  = gear::isItemWorn(obj, thePlayer);
		if (isWorn)
		{
			task->AddTask([=]() { equipManager->UnequipObject(thePlayer, obj, extraData); });
		}
		else
		{
			task->AddTask([=]() { equipManager->EquipObject(thePlayer, obj, extraData); });
		}
	}

	void equipArmorByForm(const RE::TESForm* form, RE::PlayerCharacter*& thePlayer, const std::string& nameToMatch)
	{
		// rlog::trace("attempting to equip armor; name='{}';"sv, helpers::nameAsUtf8(form));
		RE::TESBoundObject* obj      = nullptr;
		RE::ExtraDataList* extraData = nullptr;
		auto remaining               = gear::boundObjectMatchName(form, nameToMatch, obj, extraData);

		if (!obj || remaining == 0)
		{
			rlog::warn("could not find armor in player inventory; name='{}';"sv, nameToMatch);
			return;
		}

		if (!gear::isItemWorn(obj, thePlayer))
		{
			auto* task         = SKSE::GetTaskInterface();
			auto* equipManager = RE::ActorEquipManager::GetSingleton();
			task->AddTask([=]() { equipManager->EquipObject(thePlayer, obj, extraData); });
		}
	}

	// ---------- potions

	void consumePotion(const RE::TESForm* potionForm, RE::PlayerCharacter*& thePlayer)
	{
		rlog::trace("consumePotion called; form_id={:#08x}; potion='{}';"sv,
			potionForm->formID,
			helpers::nameAsUtf8(potionForm));

		RE::TESBoundObject* obj      = nullptr;
		RE::ExtraDataList* extraData = nullptr;
		auto remaining               = gear::boundObjectForForm(potionForm, obj, extraData);

		if (!obj || remaining == 0)
		{
			rlog::warn("Couldn't find requested potion in inventory!"sv);
			helpers::honk();
			return;
		}

		if (!obj->Is(RE::FormType::AlchemyItem))
		{
			helpers::honk();
			rlog::warn("bound object is not an alchemy item? name='{}'; formID={:#08x};"sv,
				helpers::nameAsUtf8(obj),
				obj->formID);
			return;
		}

		auto* alchemyItem = obj->As<RE::AlchemyItem>();
		if (alchemyItem->IsPoison())
		{
			poisonWeapon(thePlayer, alchemyItem, remaining, extraData);
			return;
		}

		auto* task = SKSE::GetTaskInterface();
		if (!task) { return; }
		task->AddTask([=]() { RE::ActorEquipManager::GetSingleton()->EquipObject(thePlayer, alchemyItem, extraData); });
	}

	void poisonWeapon(RE::PlayerCharacter*& thePlayer,
		RE::AlchemyItem*& poison,
		uint32_t remaining,
		RE::ExtraDataList* extraData)
	{
		auto* task = SKSE::GetTaskInterface();
		if (!task) { return; }

		auto* right_eq = thePlayer->GetActorRuntimeData().currentProcess->GetEquippedRightHand();
		if (right_eq && right_eq->IsWeapon())
		{
			task->AddTask(
				[=]()
				{
					RE::ActorEquipManager::GetSingleton()->EquipObject(
						thePlayer, poison, extraData, 1, gear::right_hand_equip_slot());
				});
			remaining--;
		}
		auto* left_eq = thePlayer->GetActorRuntimeData().currentProcess->GetEquippedLeftHand();
		if (left_eq && left_eq->IsWeapon() && remaining > 0)
		{
			task->AddTask(
				[=]() {
					RE::ActorEquipManager::GetSingleton()->EquipObject(
						thePlayer, poison, extraData, 1, gear::left_hand_equip_slot());
				});
		}
	}

	// ---------- sounds

	void playSound(RE::BGSSoundDescriptor* soundDescriptor, RE::PlayerCharacter*& thePlayer)
	{
		auto* audio = RE::BSAudioManager::GetSingleton();
		if (audio && soundDescriptor)
		{
			RE::BSSoundHandle soundHandle;
			audio->BuildSoundDataFromDescriptor(soundHandle, soundDescriptor);
			soundHandle.SetObjectToFollow(thePlayer->Get3D());
			soundHandle.SetVolume(1.0);
			soundHandle.Play();
			// rlog::trace("played sound"sv);
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
			rlog::info("Not drinking a {} potion because you don't need one."sv, vitalStat);
			helpers::honk();
			return;
		}

		rlog::debug("goal potion: deficit={}; min={}; max={};"sv,
			fmt::format(FMT_STRING("{:.2f}"), deficit),
			fmt::format(FMT_STRING("{:.2f}"), goalMin),
			fmt::format(FMT_STRING("{:.2f}"), goalMax));

		RE::TESBoundObject* obj = nullptr;
		float prevRating        = -100.0f;

		auto candidates = player::getInventoryForType(thePlayer, RE::FormType::AlchemyItem);
		rlog::debug("{} potions in inventory"sv, candidates.size(), vitalStat);
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
				rlog::debug("found at least one {} potion: rating={}; max_restored={}; deficit={};"sv,
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
				rlog::debug(
					"improved selection: rating={}; max_restored={}; deficit={};"sv, rating, max_restored, deficit);
				obj        = alchemy_item;
				prevRating = rating;
				if (rating == 0) break;  // perfection
				continue;
			}
		}

		if (obj)
		{
			rlog::debug("after considering {} candidates, found a potion: rating={}; name='{}';"sv,
				vitalStat,
				prevRating,
				helpers::nameAsUtf8(obj));
			auto* task = SKSE::GetTaskInterface();
			if (task)
			{
				task->AddTask([=]() { RE::ActorEquipManager::GetSingleton()->EquipObject(thePlayer, obj); });
			}
		}
		else
		{
			rlog::warn("We couldn't find any {} potions!"sv, vitalStat);
			helpers::honk();
		}
	}

	// ---------- perk visitor, used only by the actor value potion selection

	using PerkFuncType     = RE::BGSEntryPointPerkEntry::EntryData::Function;
	using PerkFuncDataType = RE::BGSEntryPointFunctionData::ENTRY_POINT_FUNCTION_DATA;

	RE::BSContainer::ForEachResult perk_visitor::Visit(RE::BGSPerkEntry* perk_entry)
	{
		const auto* entry_point = static_cast<RE::BGSEntryPointPerkEntry*>(perk_entry);
		const auto* perk        = entry_point->perk;

		rlog::trace("perk formID={:#08x}; name='{}';"sv, perk->formID, helpers::nameAsUtf8(perk));

		// This was originally intended to handle many variations of the poison
		// dose perk-- it should calculate the correct value from vanilla,
		// Adamant, Ordinator, and others. It doesn't actually do so. We apply
		// poisons differently up above, by just equipping it like normal.
		if (entry_point->functionData)
		{
			const RE::BGSEntryPointFunctionDataOneValue* value =
				static_cast<RE::BGSEntryPointFunctionDataOneValue*>(entry_point->functionData);
			if (entry_point->entryData.function == PerkFuncType::kSetValue) { result_ = value->data; }
			else if (entry_point->entryData.function == PerkFuncType::kAddValue) { result_ += value->data; }
			else if (entry_point->entryData.function == PerkFuncType::kMultiplyValue) { result_ *= value->data; }
			else if (entry_point->entryData.function == PerkFuncType::kAddActorValueMult)
			{
				if (perk_entry->GetFunction() == RE::BGSPerkEntry::EntryPoint::kModPoisonDoseCount)
				{
					auto av = actor_->AsActorValueOwner()->GetActorValue(RE::ActorValue::kAlchemy);
					result_ += static_cast<float>(av * value->data * 3);
				}
			}

			rlog::trace("Got value {} for Perk, total now is {}"sv, value->data, result_);
		}

		return RE::BSContainer::ForEachResult::kContinue;
	}

	float perk_visitor::get_result() const { return result_; }

}  // namespace game

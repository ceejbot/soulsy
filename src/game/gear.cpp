﻿#include "gear.h"

#include "constant.h"
#include "offset.h"
#include "player.h"

#include "lib.rs.h"

namespace game
{
	EquippableItemData::EquippableItemData()
		: count(0)
		, itemExtraList(nullptr)
		, wornExtraList(nullptr)
		, wornLeftExtraList(nullptr)
		, isWorn(false)
		, isWornLeft(false)
		, isFavorite(false)
	{
	}

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

	bool inventoryEntryDataFor(const RE::TESForm* form, RE::InventoryEntryData*& outentry)
	{
		auto* thePlayer = RE::PlayerCharacter::GetSingleton();
		std::map<RE::TESBoundObject*, std::pair<int, std::unique_ptr<RE::InventoryEntryData>>> candidates =
			player::getInventoryForType(thePlayer, form->GetFormType());
		RE::InventoryEntryData entryData;
		bool found = false;

		for (const auto& [item, inv_data] : candidates)
		{
			const auto& [num_items, entry] = inv_data;
			if (entry->object->formID == form->formID)
			{
				if (item)
				{
					entryData = *entry;
					found     = true;
					break;
				}
			}
		}

		if (found) { outentry = &entryData; }
		return found;
	}

	// The next three functions have some refactoring opportunities, as they
	// say. However, I am first getting them working properly before cleaning up.

	// Returns only matches that are currently equipped, so 0, 1, or 2 are your
	// only possible return values. Heh.
	int boundObjectForWornItem(const RE::TESForm* form,
		WornWhere constraint,
		RE::TESBoundObject*& outobj,
		EquippableItemData*& outEquipData)
	{
		auto* thePlayer                 = RE::PlayerCharacter::GetSingleton();
		RE::TESBoundObject* foundObject = nullptr;
		EquippableItemData equipData    = EquippableItemData();
		std::vector<RE::ExtraDataList*> extraDataCopy;

		std::map<RE::TESBoundObject*, std::pair<int, std::unique_ptr<RE::InventoryEntryData>>> candidates =
			player::getInventoryForType(thePlayer, form->GetFormType());

		bool matchFound = false;
		for (const auto& [item, inventoryData] : candidates)
		{
			const auto& [countHeld, entry] = inventoryData;

			if (entry->object->formID == form->formID)
			{
				EquippableItemData tmpData = EquippableItemData();
				std::vector<RE::ExtraDataList*> tmpExtra;
				// We walk extra data and wait until we have a worn item
				// before we decide we have a match.
				auto simpleList = entry->extraLists;
				if (simpleList)
				{
					for (auto* extraData : *simpleList)
					{
						tmpExtra.push_back(extraData);
						bool isWorn     = extraData->HasType(RE::ExtraDataType::kWorn);
						bool isWornLeft = extraData->HasType(RE::ExtraDataType::kWornLeft);

						if (isWornLeft)
						{
							matchFound = constraint == WornWhere::kLeftOnly || constraint == WornWhere::kAnywhere;
						}
						else if (isWorn)
						{
							matchFound = constraint == WornWhere::kRightOnly || constraint == WornWhere::kAnywhere;
						}

						tmpData.isFavorite |= extraData->HasType(RE::ExtraDataType::kHotkey);
						tmpData.isPoisoned |= extraData->HasType(RE::ExtraDataType::kPoison);

						if (isWorn) { tmpData.isWorn = true; }
						else if (isWornLeft) { tmpData.isWornLeft = true; }
					}
				}
				if (matchFound)
				{
					extraDataCopy = tmpExtra;
					equipData     = tmpData;
					break;
				}
			}  // end of if block
		}      // end of candidates loop

		if (!foundObject) { return 0; }

		rlog::debug("found worn bound object '{}';" rlog::formatAsHex(foundObject->formID));

		if (extraDataCopy.size() > 0)
		{
			if (equipData.isWorn) { equipData.wornExtraList = extraDataCopy.back(); }
			else if (equipData.isWornLeft) { equipData.wornLeftExtraList = extraDataCopy.back(); }
			else { equipData.itemExtraList = extraDataCopy.back(); }
		}

		outobj       = foundObject;
		outEquipData = &equipData;
		return equipData.count;
	}

	// Returns only exact name matches.
	int boundObjectMatchName(const RE::TESForm* form,
		const std::string& nameToMatch,
		RE::TESBoundObject*& outobj,
		EquippableItemData*& outEquipData)
	{
		const auto* baseName = form->GetName();
		// If we don't need to match the name, we don't do that work.
		if (std::string(baseName) == nameToMatch) { return boundObjectForForm(form, outobj, outEquipData); }

		auto* thePlayer                 = RE::PlayerCharacter::GetSingleton();
		RE::TESBoundObject* foundObject = nullptr;
		EquippableItemData equipData    = EquippableItemData();
		std::vector<RE::ExtraDataList*> extraDataCopy;

		std::map<RE::TESBoundObject*, std::pair<int, std::unique_ptr<RE::InventoryEntryData>>> candidates =
			player::getInventoryForType(thePlayer, form->GetFormType());

		for (const auto& [item, inventoryData] : candidates)
		{
			const auto& [countHeld, entry] = inventoryData;
			if (entry->object->formID == form->formID)
			{
				// there are two cases where we know we have a match:
				// first, when countHeld == 1
				// second, when the name matches
				if (countHeld > 1)
				{
					const auto candidateName = std::string(entry->GetDisplayName());
					if (candidateName != nameToMatch) { continue; }
				}
				// we have a match. This next part is probably extractable into a function.
				foundObject     = item;
				equipData.count = 1;

				auto simpleList = entry->extraLists;
				if (simpleList)
				{
					for (auto* extraData : *simpleList)
					{
						extraDataCopy.push_back(extraData);
						bool isWorn     = extraData->HasType(RE::ExtraDataType::kWorn);
						bool isWornLeft = extraData->HasType(RE::ExtraDataType::kWornLeft);
						equipData.isFavorite |= extraData->HasType(RE::ExtraDataType::kHotkey);
						equipData.isPoisoned |= extraData->HasType(RE::ExtraDataType::kPoison);

						if (isWorn) { equipData.isWorn = true; }
						else if (isWornLeft) { equipData.isWornLeft = true; }
					}
				}
				break;
			}  // end of if block
		}      // end of candidates loop

		if (!foundObject) { return 0; }

		rlog::debug(
			"found bound object matching name '{}'; formID={};"sv, nameToMatch, rlog::formatAsHex(foundObject->formID));

		if (extraDataCopy.size() > 0)
		{
			if (equipData.isWorn) { equipData.wornExtraList = extraDataCopy.back(); }
			else if (equipData.isWornLeft) { equipData.wornLeftExtraList = extraDataCopy.back(); }
			else { equipData.itemExtraList = extraDataCopy.back(); }
		}

		outobj       = foundObject;
		outEquipData = &equipData;
		return equipData.count;
	}

	// Returns first found.
	int boundObjectForForm(const RE::TESForm* form, RE::TESBoundObject*& outobj, EquippableItemData*& outEquipData)
	{
		auto* thePlayer                 = RE::PlayerCharacter::GetSingleton();
		RE::TESBoundObject* foundObject = nullptr;
		EquippableItemData equipData    = EquippableItemData();
		std::vector<RE::ExtraDataList*> extraDataCopy;

		std::map<RE::TESBoundObject*, std::pair<int, std::unique_ptr<RE::InventoryEntryData>>> candidates =
			player::getInventoryForType(thePlayer, form->GetFormType());

		auto count = 0;
		for (const auto& [item, inventoryData] : candidates)
		{
			const auto& [num_items, entry] = inventoryData;
			if (entry->object->formID == form->formID)
			{
				foundObject     = item;
				equipData.count = num_items;
				count           = num_items;
				auto simpleList = entry->extraLists;

				if (simpleList)
				{
					// Here we follow the SKSE's example in building a data structure
					// a little easier to use in making decisions about the object.
					// Another approach would be to fix up or make local replacements
					// for the clib functions that walk the extra data every time to
					// answer these questions.
					for (auto* extraData : *simpleList)
					{
						extraDataCopy.push_back(extraData);
						bool isWorn     = extraData->HasType(RE::ExtraDataType::kWorn);
						bool isWornLeft = extraData->HasType(RE::ExtraDataType::kWornLeft);
						equipData.isFavorite |= extraData->HasType(RE::ExtraDataType::kHotkey);
						equipData.isPoisoned |= extraData->HasType(RE::ExtraDataType::kPoison);

						if (isWorn)
						{
							// This bool should only be set if we have a deep name match (see comment above)
							equipData.isWorn = true;
						}
						else if (isWornLeft)
						{
							// This bool should only be set if we have a deep name match (see comment above)
							equipData.isWornLeft = true;
						}
					}
				}
				break;
			}
		}

		if (!foundObject) { return 0; }

		rlog::trace("found {} instance for bound object; name='{}'; formID={};"sv,
			count,
			form->GetName(),
			rlog::formatAsHex(form->formID));

		if (extraDataCopy.size() > 0)
		{
			if (equipData.isWorn) { equipData.wornExtraList = extraDataCopy.back(); }
			else if (equipData.isWornLeft) { equipData.wornLeftExtraList = extraDataCopy.back(); }
			else { equipData.itemExtraList = extraDataCopy.back(); }
		}

		outobj       = foundObject;
		outEquipData = &equipData;
		return count;
	}

	bool isItemWorn(RE::TESBoundObject*& bound_obj, RE::PlayerCharacter*& thePlayer)
	{
		auto worn = false;
		for (const auto& [item, inv_data] : player::getInventoryForType(thePlayer, RE::FormType::Armor))
		{
			const auto& [count, entry] = inv_data;
			if (entry && entry->object && (entry->object->formID == bound_obj->formID) && entry->IsWorn())
			{
				worn = true;
				break;
			}
		}
		return worn;
	}

	bool isItemFavorited(const RE::TESForm* form)
	{
		// TODO I don't think this handles spells
		RE::TESBoundObject* bound_obj = nullptr;
		EquippableItemData* data      = nullptr;
		auto* thePlayer               = RE::PlayerCharacter::GetSingleton();
		rlog::debug("isItemFavorited() calling boundObjectForForm()");
		game::boundObjectForForm(form, bound_obj, data);
		if (data) { return data->isFavorite; }
		return false;
	}

	bool isItemPoisoned(const RE::TESForm* form)
	{
		auto* thePlayer          = RE::PlayerCharacter::GetSingleton();
		RE::TESBoundObject* obj  = nullptr;
		EquippableItemData* data = nullptr;
		// rlog::debug("isItemPoisoned() calling boundObjectForForm()");
		[[maybe_unused]] auto count = boundObjectForForm(form, obj, data);
		if (data) { return data->isPoisoned; }
		return false;
	}

	float itemChargeLevel(const RE::TESForm* form)
	{
		RE::InventoryEntryData* inventoryEntry = nullptr;

		if (!inventoryEntryDataFor(form, inventoryEntry)) { return 0.0f; }
		std::optional<double> charge = inventoryEntry->GetEnchantmentCharge();
		return static_cast<float>(charge.value_or(0.0));
	}

	const char* displayName(const RE::TESForm* form)
	{
		auto* thePlayer = RE::PlayerCharacter::GetSingleton();
		std::map<RE::TESBoundObject*, std::pair<int, std::unique_ptr<RE::InventoryEntryData>>> candidates =
			player::getInventoryForType(thePlayer, form->GetFormType());

		for (const auto& [item, inv_data] : candidates)
		{
			const auto& [num_items, entry] = inv_data;
			if (entry->object->formID == form->formID)
			{
				if (item && entry->extraLists)
				{
					for (auto* datalist : *entry->extraLists)
					{
						auto* extrafox = datalist->GetByType(RE::ExtraDataType::kTextDisplayData);
						if (extrafox)
						{
							auto* extraTxt = static_cast<RE::ExtraTextDisplayData*>(extrafox);
							if (extraTxt->customNameLength > 0) { return extraTxt->displayName.c_str(); }
						}
					}
				}
			}
		}

		return form->GetName();
	}

	void equipItemByFormAndSlot(RE::TESForm* form,
		RE::BGSEquipSlot*& slot,
		RE::PlayerCharacter*& thePlayer,
		const std::string& nameToMatch)
	{
		auto slot_is_left = slot == left_hand_equip_slot();
		rlog::debug("attempting to equip item in slot; name='{}'; is-left='{}'; type={};"sv,
			form->GetName(),
			slot_is_left,
			form->GetFormType());

		if (form->formID == util::unarmed)
		{
			rlog::debug("unequipping this slot by request!"sv);
			unequipLeftOrRightSlot(thePlayer, slot);
			return;
		}
		else if (form->Is(RE::FormType::Spell))
		{
			// We do not want to look for a bound object for spells. Q: why not?
			equipSpellByFormAndSlot(form, slot, thePlayer);
			return;
		}

		RE::TESBoundObject* equipObject = nullptr;
		EquippableItemData* data        = nullptr;
		rlog::debug("equipIemByFormAndSlot() calling boundObjectForForm()");
		auto foundCount = boundObjectMatchName(form, nameToMatch, equipObject, data);
		if (foundCount == 0)
		{
			rlog::debug("unable to find bound object for name='{}'"sv, nameToMatch);
			return;
		}

		const auto* obj_right = thePlayer->GetActorRuntimeData().currentProcess->GetEquippedRightHand();
		const auto* obj_left  = thePlayer->GetActorRuntimeData().currentProcess->GetEquippedLeftHand();

		const auto obj_equipped_left  = obj_left && obj_left->formID == equipObject->formID;
		const auto obj_equipped_right = obj_right && obj_right->formID == equipObject->formID;

		if (slot_is_left && obj_equipped_left)
		{
			rlog::debug("item already equipped in left hand. name='{}'"sv, equipObject->GetName());
			return;
		}

		if (!slot_is_left && obj_equipped_right)
		{
			rlog::debug("item already equipped in right hand. name='{}'"sv, equipObject->GetName());
			return;
		}

		auto equipped_count = 0;
		if (obj_equipped_left) { equipped_count++; }
		if (obj_equipped_right) { equipped_count++; }
		rlog::trace("checking how many '{}' we have available; count={}; equipped_count={}"sv,
			equipObject->GetName(),
			foundCount,
			equipped_count);

		if (foundCount == equipped_count)
		{
			// The game might try to equip something else, according to mlthelama.
			unequipLeftOrRightSlot(thePlayer, slot);
			return;
		}

		rlog::debug("queuing task to equip '{}'; left={}; formID={};"sv,
			form->GetName(),
			slot_is_left,
			rlog::formatAsHex(equipObject->formID));
		auto* task = SKSE::GetTaskInterface();
		if (task)
		{
			task->AddTask(
				[=]() {
					RE::ActorEquipManager::GetSingleton()->EquipObject(
						thePlayer, equipObject, data->itemExtraList, 1, slot);
				});
		}
	}

	void equipSpellByFormAndSlot(RE::TESForm* form, RE::BGSEquipSlot*& slot, RE::PlayerCharacter*& player)
	{
		auto slot_is_left = slot == left_hand_equip_slot();
		rlog::debug("attempting to equip spell in slot; name='{}'; is-left='{}'; type={};"sv,
			form->GetName(),
			slot_is_left,
			form->GetFormType());

		const auto* obj_right = player->GetActorRuntimeData().currentProcess->GetEquippedRightHand();
		const auto* obj_left  = player->GetActorRuntimeData().currentProcess->GetEquippedLeftHand();

		const auto obj_equipped_left  = obj_left && obj_left->formID == form->formID;
		const auto obj_equipped_right = obj_right && obj_right->formID == form->formID;

		if (slot_is_left && obj_equipped_left)
		{
			rlog::debug("spell already equipped in left hand. name='{}'"sv, form->GetName());
			return;
		}

		if (!slot_is_left && obj_equipped_right)
		{
			rlog::debug("spell already equipped in right hand. name='{}'"sv, form->GetName());
			return;
		}

		auto* task = SKSE::GetTaskInterface();
		if (!task) return;

		auto* spell = form->As<RE::SpellItem>();
		if (player->HasSpell(spell))
		{
			task->AddTask([=]() { RE::ActorEquipManager::GetSingleton()->EquipSpell(player, spell, slot); });
		}
		else
		{
			rlog::info("player tried to equip a spell they don't know; upstream bug?"sv);
			return;
		}

		rlog::debug("queued task to equip '{}'; left={}; formID={};"sv,
			form->GetName(),
			slot_is_left,
			rlog::formatAsHex(form->formID));
	}

	void unequipHand(RE::PlayerCharacter*& player, Action which)
	{
		RE::BGSEquipSlot* slot = nullptr;
		if (which == Action::Left) { slot = left_hand_equip_slot(); }
		else if (which == Action::Right) { slot = right_hand_equip_slot(); }
		else
		{
			rlog::debug("somebody called unequipHand() with slot={};"sv, static_cast<uint8_t>(which));
			return;
		}

		unequipLeftOrRightSlot(player, slot);
	}

	void unequipLeftOrRightSlot(RE::PlayerCharacter*& player, RE::BGSEquipSlot*& slot)
	{
		// We're starting with a slot not a hand enum.
		RE::TESForm* equipped = nullptr;
		if (slot == left_hand_equip_slot())
		{
			equipped = player->GetActorRuntimeData().currentProcess->GetEquippedLeftHand();
		}
		else if (slot == right_hand_equip_slot())
		{
			equipped = player->GetActorRuntimeData().currentProcess->GetEquippedRightHand();
		}
		else
		{
			rlog::debug("slot is not left/right!");
			return;
		}
		if (!equipped) { return; }

		auto* equip_manager = RE::ActorEquipManager::GetSingleton();
		auto* task          = SKSE::GetTaskInterface();
		if (!task)
		{
			rlog::warn("Unable to get SKSE task interface! Cannot equip or unequip anything."sv);
			return;
		}

		if (equipped->Is(RE::FormType::Spell))
		{
			// We have to do the dagger proxy trick. If we can't find the dagger because
			// this is the Skyrim engine without the Skyrim assets, then we don't equip
			// it because oh well. At least we didn't crash.
			auto* form = RE::TESForm::LookupByID<RE::TESForm>(0x00020163);
			if (!form) { return; }
			auto* proxy = form->As<RE::TESObjectWEAP>();
			task->AddTask([=]() { equip_manager->EquipObject(player, proxy, nullptr, 1, slot, false, true, false); });
			task->AddTask([=]() { equip_manager->UnequipObject(player, proxy, nullptr, 1, slot, false, true, false); });
			return;
		}

		auto* object = equipped->As<RE::TESBoundObject>();
		if (!object) { return; }
		task->AddTask([=]() { equip_manager->UnequipObject(player, object, nullptr, 1, slot, false, true, false); });
	}

}

#include "gear.h"

#include "RE/E/ExtraDataTypes.h"
#include "constant.h"
#include "helpers.h"
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

		for (const auto& [item, invData] : candidates)
		{
			const auto& [num_items, entry] = invData;
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
		RE::ExtraDataList* outextra)
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

		rlog::debug("boundObjectForWornItem(constraint={}) found formid='{}';",
			static_cast<std::underlying_type_t<WornWhere>>(constraint),
			rlog::formatAsHex(foundObject->formID));

		if (extraDataCopy.size() > 0) { outextra = extraDataCopy.back(); }
		outobj = foundObject;
		return equipData.count;
	}

	// Returns only exact name matches.
	int boundObjectMatchName(const RE::TESForm* form,
		const std::string& nameToMatch,
		RE::TESBoundObject*& outobj,
		RE::ExtraDataList* outextra)
	{
		const auto* baseName = form->GetName();  // this use of GetName() is okay
		// If we don't need to match the name, we don't do that work.
		if (std::string(baseName) == nameToMatch) { return boundObjectForForm(form, outobj, outextra); }

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
					for (auto* extraData : *simpleList) { extraDataCopy.push_back(extraData); }
				}
				break;
			}  // end of if block
		}      // end of candidates loop

		if (!foundObject) { return 0; }

		rlog::debug(
			"boundObjectMatchName '{}'; found formID={};"sv, nameToMatch, rlog::formatAsHex(foundObject->formID));
		if (extraDataCopy.size() > 0) { outextra = extraDataCopy.back(); }
		outobj = foundObject;
		return equipData.count;
	}

	// Returns first found.
	int boundObjectForForm(const RE::TESForm* form, RE::TESBoundObject*& outobj, RE::ExtraDataList* outextra)
	{
		auto* thePlayer                 = RE::PlayerCharacter::GetSingleton();
		RE::TESBoundObject* foundObject = nullptr;
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
				count           = num_items;
				auto simpleList = entry->extraLists;

				if (simpleList)
				{
					for (auto* extraData : *simpleList) { extraDataCopy.push_back(extraData); }
				}
				break;
			}
		}

		if (!foundObject) { return 0; }

		rlog::trace("found {} instance(s) for bound object; name='{}'; formID={};"sv,
			count,
			helpers::nameAsUtf8(form),
			rlog::formatAsHex(form->formID));

		if (extraDataCopy.size() > 0) { outextra = extraDataCopy.back(); }
		outobj = foundObject;
		return count;
	}

	bool isItemWorn(RE::TESBoundObject*& bound_obj, RE::PlayerCharacter*& thePlayer)
	{
		auto worn = false;
		for (const auto& [item, inv_data] : player::getInventoryForType(thePlayer, RE::FormType::Armor))
		{
			const auto& [count, entry] = invData;
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
		RE::ExtraDataList* extraData  = nullptr;
		game::boundObjectForForm(form, bound_obj, extraData);
		if (extraData) { return extraData->HasType(RE::ExtraDataType::kHotkey); }
		return false;
	}

	bool isItemPoisoned(const RE::TESForm* form)
	{
		RE::TESBoundObject* obj      = nullptr;
		RE::ExtraDataList* extraData = nullptr;
		[[maybe_unused]] auto count  = boundObjectForForm(form, obj, extraData);
		if (extraData) { return extraData->HasType(RE::ExtraDataType::kPoison); }
		return false;
	}

	bool itemHasCharge(const RE::TESForm* form)
	{
		if (!form) { return false; }
		if (form->Is(RE::FormType::Shout)) { return true; }
		const auto enchantable = form->As<RE::TESEnchantableForm>();
		if (enchantable && enchantable->formEnchanting) { return true; }

		auto* thePlayer = RE::PlayerCharacter::GetSingleton();
		std::map<RE::TESBoundObject*, std::pair<int, std::unique_ptr<RE::InventoryEntryData>>> candidates =
			player::getInventoryForType(thePlayer, form->GetFormType());

		for (const auto& [item, invData] : candidates)
		{
			const auto& [num_items, entry] = invData;
			if (entry->object->formID == form->formID)
			{
				if (item && entry->extraLists)
				{
					for (auto* datalist : *entry->extraLists)
					{
						auto* extrafox = datalist->GetByType(RE::ExtraDataType::kEnchantment);
						if (extrafox) { return true; }
					}
				}
			}
		}
		return false;
	}

	// This returns a percentage.
	float itemChargeLevel(const RE::TESForm* form)
	{
		if (!form) { return false; }

		if (form->Is(RE::FormType::Shout))
		{
			auto* thePlayer   = RE::PlayerCharacter::GetSingleton();
			auto* playerActor = thePlayer->As<RE::Actor>();
			return static_cast<float>(playerActor->GetCurrentShoutLevel());
		}

		float current = 0.0f;
		float max     = 0.0f;

		const auto enchantable = form->As<RE::TESEnchantableForm>();
		if (enchantable) { max = enchantable->amountofEnchantment; }

		auto* thePlayer = RE::PlayerCharacter::GetSingleton();
		std::map<RE::TESBoundObject*, std::pair<int, std::unique_ptr<RE::InventoryEntryData>>> candidates =
			player::getInventoryForType(thePlayer, form->GetFormType());

		for (const auto& [item, invData] : candidates)
		{
			const auto& [num_items, entry] = invData;
			if (entry->object->formID == form->formID)
			{
				if (item && entry->extraLists)
				{
					for (auto* datalist : *entry->extraLists)
					{
						auto* maybe_charge = datalist->GetByType(RE::ExtraDataType::kCharge);
						if (maybe_charge && current == 0.0f)
						{
							auto* charge = static_cast<RE::ExtraCharge*>(maybe_charge);
							current      = charge->charge;
						}
					}  // end of extra data checking
					if (max > 0.0f && current > 0.0f) { return current * 100.0f / max; }
				}
			}
		}  // end of candidates loop

		RE::InventoryEntryData* inventoryEntry = nullptr;
		return 100.0f;
	}

	const char* displayName(const RE::TESForm* form)
	{
		if (!form) { return "null"; }

		auto* thePlayer = RE::PlayerCharacter::GetSingleton();
		std::map<RE::TESBoundObject*, std::pair<int, std::unique_ptr<RE::InventoryEntryData>>> candidates =
			player::getInventoryForType(thePlayer, form->GetFormType());

		for (const auto& [item, invData] : candidates)
		{
			const auto& [num_items, entry] = invData;
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
		rlog::trace("attempting to equip item in slot; name='{}'; is-left='{}'; type={};"sv,
			helpers::nameAsUtf8(form),
			slot_is_left,
			form->GetFormType());

		if (form->formID == util::unarmed)
		{
			logger::debug("unequipping this slot by request!"sv);
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
		RE::ExtraDataList* extraData    = nullptr;
		auto foundCount                 = boundObjectMatchName(form, nameToMatch, equipObject, extraData);
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
			rlog::debug("item already equipped in left hand. name='{}'"sv, helpers::nameAsUtf8(equipObject));
			return;
		}

		if (!slot_is_left && obj_equipped_right)
		{
			rlog::debug("item already equipped in right hand. name='{}'"sv, helpers::nameAsUtf8(equipObject));
			return;
		}

		auto equipped_count = 0;
		if (obj_equipped_left) { equipped_count++; }
		if (obj_equipped_right) { equipped_count++; }
		rlog::trace("checking how many '{}' we have available; count={}; equipped_count={}"sv,
			helpers::nameAsUtf8(equipObject),
			foundCount,
			equipped_count);

		if (foundCount == equipped_count)
		{
			// The game might try to equip something else, according to mlthelama.
			unequipLeftOrRightSlot(thePlayer, slot);
			return;
		}

		rlog::debug("queuing task to equip '{}'; left={}; formID={};"sv,
			helpers::nameAsUtf8(form),
			slot_is_left,
			rlog::formatAsHex(equipObject->formID));
		auto* task = SKSE::GetTaskInterface();
		if (task)
		{
			task->AddTask([=]()
				{ RE::ActorEquipManager::GetSingleton()->EquipObject(thePlayer, equipObject, extraData, 1, slot); });
		}
	}

	void equipSpellByFormAndSlot(RE::TESForm* form, RE::BGSEquipSlot*& slot, RE::PlayerCharacter*& thePlayer)
	{
		auto slot_is_left = slot == left_hand_equip_slot();
		rlog::trace("attempting to equip spell in slot; name='{}'; is-left='{}'; type={};"sv,
			helpers::nameAsUtf8(form),
			slot_is_left,
			form->GetFormType());

		const auto* obj_right = thePlayer->GetActorRuntimeData().currentProcess->GetEquippedRightHand();
		const auto* obj_left  = thePlayer->GetActorRuntimeData().currentProcess->GetEquippedLeftHand();

		const auto obj_equipped_left  = obj_left && obj_left->formID == form->formID;
		const auto obj_equipped_right = obj_right && obj_right->formID == form->formID;

		if (slot_is_left && obj_equipped_left)
		{
			rlog::debug("spell already equipped in left hand. name='{}'"sv, helpers::nameAsUtf8(form));
			return;
		}

		if (!slot_is_left && obj_equipped_right)
		{
			rlog::debug("spell already equipped in right hand. name='{}'"sv, helpers::nameAsUtf8(form));
			return;
		}

		auto* task = SKSE::GetTaskInterface();
		if (!task) return;

		auto* spell = form->As<RE::SpellItem>();
		if (thePlayer->HasSpell(spell))
		{
			task->AddTask([=]() { RE::ActorEquipManager::GetSingleton()->EquipSpell(thePlayer, spell, slot); });
		}
		else
		{
			rlog::info("player tried to equip a spell they don't know; upstream bug?"sv);
			return;
		}

		rlog::debug("queued task to equip '{}'; left={}; formID={};"sv,
			helpers::nameAsUtf8(form),
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

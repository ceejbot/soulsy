#include "gear.h"

#include "constant.h"
#include "offset.h"
#include "player.h"
#include "string_util.h"

#include "lib.rs.h"

namespace game
{
	EquippableItemData::EquippableItemData()
		: count(0)
		, itemExtraList(NULL)
		, wornExtraList(NULL)
		, wornLeftExtraList(NULL)
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
		auto* the_player = RE::PlayerCharacter::GetSingleton();
		std::map<RE::TESBoundObject*, std::pair<int, std::unique_ptr<RE::InventoryEntryData>>> candidates =
			player::getInventoryForType(the_player, form->GetFormType());
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

	int boundObjectForForm(const RE::TESForm* form,
		RE::PlayerCharacter*& the_player,
		RE::TESBoundObject*& outobj,
		EquippableItemData*& outEquipData)
	{
		RE::TESBoundObject* foundObject = nullptr;
		RE::ExtraDataList* extra        = nullptr;
		EquippableItemData equipData    = EquippableItemData();
		std::vector<RE::ExtraDataList*> extra_vector;

		std::map<RE::TESBoundObject*, std::pair<int, std::unique_ptr<RE::InventoryEntryData>>> candidates =
			player::getInventoryForType(the_player, form->GetFormType());

		auto count = 0;
		for (const auto& [item, inventoryData] : candidates)
		{
			const auto& [num_items, entry] = inventoryData;
			// TODO after base case is working, refactor to allow comparing to the
			// exact item name we want to equip. This will need to get passed down
			// from the equip-item call on the Rust side, because rust has this data.
			if (entry->object->formID == form->formID)
			{
				foundObject     = item;
				equipData.count = num_items;
				count           = num_items;
				auto simpleList = entry->extraLists;

				if (simpleList)
				{
					for (auto* extraData : *simpleList)
					{
						extra           = extraData;
						bool isWorn     = extraData->HasType(RE::ExtraDataType::kWorn);
						bool isWornLeft = extraData->HasType(RE::ExtraDataType::kWornLeft);
						equipData.isFavorite |= extraData->HasType(RE::ExtraDataType::kHotkey);
						equipData.isPoisoned |= extraData->HasType(RE::ExtraDataType::kPoison);

						if (isWorn)
						{
							equipData.isWorn        = isWorn;
							equipData.wornExtraList = extraData;
						}
						else if (isWornLeft)
						{
							equipData.isWornLeft        = true;
							equipData.wornLeftExtraList = extraData;
						}
						else { equipData.itemExtraList = extraData; }
					}
				}
				break;
			}
		}

		if (!foundObject) { return 0; }

		rlog::trace("found {} instance for bound object; name='{}'; formID={};"sv,
			count,
			form->GetName(),
			util::string_util::int_to_hex(form->formID));

		outobj       = foundObject;
		outEquipData = &equipData;
		return count;
	}

	bool isItemWorn(RE::TESBoundObject*& bound_obj, RE::PlayerCharacter*& the_player)
	{
		auto worn = false;
		for (const auto& [item, inv_data] : player::getInventoryForType(the_player, RE::FormType::Armor))
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
		game::boundObjectForForm(form, thePlayer, bound_obj, data);
		if (data) { return data->isFavorite; }
		return false;
	}

	bool isItemPoisoned(const RE::TESForm* form)
	{
		auto* the_player            = RE::PlayerCharacter::GetSingleton();
		RE::TESBoundObject* obj     = nullptr;
		EquippableItemData* data    = nullptr;
		[[maybe_unused]] auto count = boundObjectForForm(form, the_player, obj, data);
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
		auto* the_player = RE::PlayerCharacter::GetSingleton();
		std::map<RE::TESBoundObject*, std::pair<int, std::unique_ptr<RE::InventoryEntryData>>> candidates =
			player::getInventoryForType(the_player, form->GetFormType());

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

	void equipItemByFormAndSlot(RE::TESForm* form, RE::BGSEquipSlot*& slot, RE::PlayerCharacter*& thePlayer)
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
		auto item_count                 = boundObjectForForm(form, thePlayer, equipObject, data);
		if (!equipObject)
		{
			rlog::debug("unable to find bound object for name='{}'"sv, form->GetName());
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
		rlog::debug("checking how many '{}' we have available; count={}; equipped_count={}"sv,
			equipObject->GetName(),
			item_count,
			equipped_count);

		if (item_count == equipped_count)
		{
			// The game might try to equip something else, according to mlthelama.
			unequipLeftOrRightSlot(thePlayer, slot);
			return;
		}

		rlog::debug("queuing task to equip '{}'; left={}; formID={};"sv,
			form->GetName(),
			slot_is_left,
			util::string_util::int_to_hex(equipObject->formID));
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
		if (form->Is(RE::FormType::Scroll))
		{
			equipItemByFormAndSlot(form, slot, player);
			return;
		}

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
			util::string_util::int_to_hex(form->formID));
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

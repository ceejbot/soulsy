#include "gear.h"

#include "constant.h"
#include "offset.h"
#include "player.h"
#include "string_util.h"
#include "weapons.h"

#include "lib.rs.h"

namespace game
{
	int boundObjectForForm(const RE::TESForm* form,
		RE::PlayerCharacter*& the_player,
		RE::TESBoundObject*& outobj,
		RE::ExtraDataList*& outextra)
	{
		RE::TESBoundObject* bound_obj = nullptr;
		RE::ExtraDataList* extra      = nullptr;
		std::vector<RE::ExtraDataList*> extra_vector;
		std::map<RE::TESBoundObject*, std::pair<int, std::unique_ptr<RE::InventoryEntryData>>> candidates =
			player::getInventoryForType(the_player, form->GetFormType());

		logger::trace("found count={} candidates for name='{}';"sv, candidates.size(), form->GetName());

		auto item_count = 0;
		for (const auto& [item, inv_data] : candidates)
		{
			if (const auto& [num_items, entry] = inv_data; entry->object->formID == form->formID)
			{
				bound_obj                   = item;
				item_count                  = num_items;
				auto simple_extra_data_list = entry->extraLists;
				if (simple_extra_data_list)
				{
					for (auto* extra_data : *simple_extra_data_list)
					{
						extra           = extra_data;
						auto worn_right = extra_data->HasType(RE::ExtraDataType::kWorn);
						auto worn_left  = extra_data->HasType(RE::ExtraDataType::kWornLeft);
						logger::trace("extra data {}, worn right {}, worn left {}"sv,
							extra_data->GetCount(),
							worn_right,
							worn_left);
						if (!worn_right && !worn_left) { extra_vector.push_back(extra_data); }
					}
				}
				break;
			}
		}

		if (!bound_obj)
		{
			logger::info("unable to find any bound objects for item; bailing. name='{}'; "sv, form->GetName());
			return 0;
		}

		logger::info("found {} instance for bound object; name='{}'; formid=0x{};"sv,
			item_count,
			form->GetName(),
			util::string_util::int_to_hex(form->formID));

		if (!extra_vector.empty()) { outextra = extra_vector.back(); }
		outobj = bound_obj;
		return item_count;
	}

	bool isItemWorn(RE::TESBoundObject*& bound_obj, RE::PlayerCharacter*& player)
	{
		auto worn = false;
		for (const auto& [item, inv_data] : player::getInventoryForType(player, RE::FormType::Armor))
		{
			if (const auto& [count, entry] = inv_data; entry->object->formID == bound_obj->formID && entry->IsWorn())
			{
				worn = true;
				break;
			}
		}
		return worn;
	}

	void equipItemByFormAndSlot(const RE::TESForm* form, RE::BGSEquipSlot*& slot, RE::PlayerCharacter*& player)
	{
		auto slot_is_left = slot == left_hand_equip_slot();
		logger::trace("attempting to equip item in slot; name='{}'; is-left='{}'; type={};"sv,
			form->GetName(),
			slot_is_left,
			form->GetFormType());

		if (form->formID == util::unarmed)
		{
			logger::trace("this slot should be unarmed; unequipping slot"sv);
			unequipLeftOrRightSlot(slot, player);
			return;
		}

		RE::TESBoundObject* bound_obj = nullptr;
		RE::ExtraDataList* extra      = nullptr;
		auto item_count               = boundObjectForForm(form, player, bound_obj, extra);

		if (!bound_obj)
		{
			logger::info("unable to find bound object for name='{}'"sv, form->GetName());
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
			unequipLeftOrRightSlot(slot, player);
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
}

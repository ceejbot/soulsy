#include "gear.h"

#include "constant.h"
#include "magic.h"
#include "offset.h"
#include "player.h"
#include "string_util.h"

#include "lib.rs.h"

namespace game
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

	int boundObjectForForm(const RE::TESForm* form,
		RE::PlayerCharacter*& thePlayer,
		RE::TESBoundObject*& outobj,
		RE::ExtraDataList*& outextra)
	{
		RE::TESBoundObject* bound_obj = nullptr;
		RE::ExtraDataList* extra      = nullptr;
		std::vector<RE::ExtraDataList*> extra_vector;

		std::map<RE::TESBoundObject*, std::pair<int, std::unique_ptr<RE::InventoryEntryData>>> candidates =
			player::getInventoryForType(thePlayer, form->GetFormType());

		auto item_count = 0;
		for (const auto& [item, invData] : candidates)
		{
			if (const auto& [num_items, entry] = invData; entry->object->formID == form->formID)
			{
				bound_obj                   = item;
				item_count                  = num_items;
				auto simple_extra_data_list = entry->extraLists;

				if (simple_extra_data_list)
				{
					for (auto* extra_data : *simple_extra_data_list)
					{
						extra = extra_data;
						extra_vector.push_back(extra_data);
					}
				}
				break;
			}
		}

		if (!bound_obj) { return 0; }

		rlog::trace("found {} instance for bound object; name='{}'; formID={};"sv,
			item_count,
			form->GetName(),
			util::string_util::int_to_hex(form->formID));

		if (!extra_vector.empty()) { outextra = extra_vector.back(); }
		outobj = bound_obj;
		return item_count;
	}

	bool isItemWorn(RE::TESBoundObject*& bound_obj, RE::PlayerCharacter*& thePlayer)
	{
		auto worn = false;
		for (const auto& [item, invData] : player::getInventoryForType(thePlayer, RE::FormType::Armor))
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
		// TODO this does not handle spells
		RE::TESBoundObject* boundObj = nullptr;
		RE::ExtraDataList* extra     = nullptr;
		auto* thePlayer              = RE::PlayerCharacter::GetSingleton();
		game::boundObjectForForm(form, thePlayer, boundObj, extra);
		if (extra) { return extra->HasType(RE::ExtraDataType::kHotkey); }
		return false;
	}

	bool isItemPoisoned(const RE::TESForm* form)
	{
		if (!form) { return false; }
		auto* thePlayer             = RE::PlayerCharacter::GetSingleton();
		RE::TESBoundObject* obj     = nullptr;
		RE::ExtraDataList* extra    = nullptr;
		[[maybe_unused]] auto count = boundObjectForForm(form, thePlayer, obj, extra);
		if (extra) { return extra->HasType(RE::ExtraDataType::kPoison); }
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

	void equipItemByFormAndSlot(RE::TESForm* form, RE::BGSEquipSlot*& slot, RE::PlayerCharacter*& thePlayer)
	{
		auto slot_is_left = slot == left_hand_equip_slot();
		rlog::debug("attempting to equip item in slot; name='{}'; is-left='{}'; type={};"sv,
			form->GetName(),
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
			// We do not want to look for a bound object for spells.
			equipSpellByFormAndSlot(form, slot, thePlayer);
			return;
		}

		RE::TESBoundObject* bound_obj = nullptr;
		RE::ExtraDataList* extra      = nullptr;
		auto item_count               = boundObjectForForm(form, thePlayer, bound_obj, extra);
		if (!bound_obj)
		{
			rlog::debug("unable to find bound object for name='{}'"sv, form->GetName());
			return;
		}

		const auto* obj_right = thePlayer->GetActorRuntimeData().currentProcess->GetEquippedRightHand();
		const auto* obj_left  = thePlayer->GetActorRuntimeData().currentProcess->GetEquippedLeftHand();

		const auto obj_equipped_left  = obj_left && obj_left->formID == bound_obj->formID;
		const auto obj_equipped_right = obj_right && obj_right->formID == bound_obj->formID;

		if (slot_is_left && obj_equipped_left)
		{
			rlog::debug("item already equipped in left hand. name='{}'"sv, bound_obj->GetName());
			return;
		}

		if (!slot_is_left && obj_equipped_right)
		{
			rlog::debug("item already equipped in right hand. name='{}'"sv, bound_obj->GetName());
			return;
		}

		auto equipped_count = 0;
		if (obj_equipped_left) { equipped_count++; }
		if (obj_equipped_right) { equipped_count++; }
		rlog::debug("checking how many '{}' we have available; count={}; equipped_count={}"sv,
			bound_obj->GetName(),
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
			util::string_util::int_to_hex(bound_obj->formID));
		auto* task = SKSE::GetTaskInterface();
		if (task)
		{
			task->AddTask(
				[=]() { RE::ActorEquipManager::GetSingleton()->EquipObject(thePlayer, bound_obj, nullptr, 1, slot); });
		}
	}

	void equipSpellByFormAndSlot(RE::TESForm* form, RE::BGSEquipSlot*& slot, RE::PlayerCharacter*& thePlayer)
	{
		if (form->Is(RE::FormType::Scroll))
		{
			equipItemByFormAndSlot(form, slot, thePlayer);
			return;
		}

		auto slot_is_left = slot == left_hand_equip_slot();
		rlog::debug("attempting to equip spell in slot; name='{}'; is-left='{}'; type={};"sv,
			form->GetName(),
			slot_is_left,
			form->GetFormType());

		const auto* obj_right = thePlayer->GetActorRuntimeData().currentProcess->GetEquippedRightHand();
		const auto* obj_left  = thePlayer->GetActorRuntimeData().currentProcess->GetEquippedLeftHand();

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

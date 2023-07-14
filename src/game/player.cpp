#include "player.h"

#include "enums.h"
#include "equippable.h"
#include "handle/ammo_handle.h"
#include "helpers.h"
#include "offset.h"
#include "string_util.h"
#include "user_settings.h"

#include "lib.rs.h"

namespace player
{
	using string_util = util::string_util;
	using slot_type   = enums::slot_type;
	using action_type = enums::action_type;
	using data_helper = helpers::data_helper;

	rust::Box<CycleEntry> equipped_left_hand()
	{
		auto* player   = RE::PlayerCharacter::GetSingleton();
		const auto obj = player->GetActorRuntimeData().currentProcess->GetEquippedLeftHand();
		if (!obj)
			return default_cycle_entry();
		auto* item_form = RE::TESForm::LookupByID(obj->formID);
		if (!item_form)
			return default_cycle_entry();
		return equippable::cycle_entry_from_form(item_form);
	}

	rust::Box<CycleEntry> equipped_right_hand()
	{
		auto* player = RE::PlayerCharacter::GetSingleton();

		const auto obj = player->GetActorRuntimeData().currentProcess->GetEquippedRightHand();
		if (!obj)
			return default_cycle_entry();
		auto* item_form = RE::TESForm::LookupByID(obj->formID);
		if (!item_form)
			return default_cycle_entry();
		return equippable::cycle_entry_from_form(item_form);
	}

	rust::Box<CycleEntry> equipped_power()
	{
		auto* player    = RE::PlayerCharacter::GetSingleton();
		const auto* obj = player->GetActorRuntimeData().selectedPower;
		if (!obj)
			return default_cycle_entry();
		auto* item_form = RE::TESForm::LookupByID(obj->formID);
		if (!item_form)
			return default_cycle_entry();
		return equippable::cycle_entry_from_form(item_form);
	}

	rust::Box<CycleEntry> equipped_ammo()
	{
		const auto* ammo_handle = handle::ammo_handle::get_singleton();
		auto* current_ammo      = ammo_handle->get_current();

		if (!current_ammo || !current_ammo->form)
		{
			return default_cycle_entry();
		}

		const auto formspec       = helpers::get_form_spec(current_ammo->form);
		const std::string tmpname = current_ammo->form->GetName();

		return create_cycle_entry(EntryKind::Arrow,
			false,
			true,
			current_ammo->item_count,
			current_ammo->form->GetName(),
			formspec);
	}

	std::map<RE::TESBoundObject*, std::pair<int, std::unique_ptr<RE::InventoryEntryData>>>
		get_inventory(RE::PlayerCharacter*& a_player, RE::FormType a_type)
	{
		return a_player->GetInventory([a_type](const RE::TESBoundObject& a_object) { return a_object.Is(a_type); });
	}

	uint32_t get_inventory_count(const RE::TESForm* a_form)
	{
		uint32_t count = 0;
		if (!a_form)
		{
			return count;
		}

		auto* player = RE::PlayerCharacter::GetSingleton();
		if (a_form->IsWeapon())
		{
			count = get_inventory_count(a_form, RE::FormType::Weapon, player);
		}
		else if (a_form->IsArmor())
		{
			count = get_inventory_count(a_form, RE::FormType::Armor, player);
		}

		logger::trace("got {} in inventory for item {}"sv, count, a_form->GetName());

		return count;
	}

	std::vector<data_helper*> get_hand_assignment(bool a_two_handed)
	{
		std::vector<data_helper*> data;
		const auto* player = RE::PlayerCharacter::GetSingleton();
		auto right_obj     = player->GetActorRuntimeData().currentProcess->GetEquippedRightHand();
		auto left_obj      = player->GetActorRuntimeData().currentProcess->GetEquippedLeftHand();

		const auto empty_handle = config::mcm_setting::get_empty_hand_setting();

		const auto item   = new data_helper();
		item->form        = nullptr;
		item->left        = false;
		item->type        = slot_type::empty;
		item->action_type = empty_handle ? action_type::un_equip : action_type::default_action;
		data.push_back(item);

		const auto item2   = new data_helper();
		item2->form        = nullptr;
		item2->left        = true;
		item2->type        = slot_type::empty;
		item2->action_type = empty_handle ? action_type::un_equip : action_type::default_action;
		data.push_back(item2);

		if (!a_two_handed)
		{
			a_two_handed = right_obj && equippable::is_two_handed(right_obj);
		}

		logger::trace("got form {}, name {} on both/right hand"sv,
			right_obj ? string_util::int_to_hex(right_obj->GetFormID()) : "null",
			right_obj ? right_obj->GetName() : "null");

		logger::trace("got form {}, name {} on left hand"sv,
			left_obj ? string_util::int_to_hex(left_obj->GetFormID()) : "null",
			left_obj ? left_obj->GetName() : "null");

		if (a_two_handed && right_obj && left_obj && right_obj->formID == left_obj->formID)
		{
			data[0]->form        = right_obj;
			data[0]->left        = false;
			data[0]->type        = equippable::get_type(right_obj);
			data[0]->action_type = action_type::default_action;
			data.erase(data.begin() + 1);
		}

		if (right_obj)
		{
			data[0]->form        = right_obj;
			data[0]->left        = false;
			data[0]->type        = equippable::get_type(right_obj);
			data[0]->action_type = action_type::default_action;
		}

		if (left_obj)
		{
			data[1]->form        = left_obj;
			data[1]->left        = true;
			data[1]->type        = equippable::get_type(left_obj);
			data[1]->action_type = action_type::default_action;
		}

		logger::trace("got {} items in List now. return."sv, data.size());
		return data;
	}

	bool has_item_or_spell(RE::TESForm* a_form)
	{
		auto has_it = false;
		if (!a_form)
		{
			return has_it;
		}

		//add option to skip check for Items
		auto* player = RE::PlayerCharacter::GetSingleton();
		if (a_form->IsWeapon())
		{
			has_it = get_inventory_count(a_form, RE::FormType::Weapon, player) > 0;
		}
		else if (a_form->IsArmor())
		{
			has_it = get_inventory_count(a_form, RE::FormType::Armor, player) > 0;
		}
		else if (a_form->Is(RE::FormType::Light))
		{
			has_it = get_inventory_count(a_form, RE::FormType::Light, player) > 0;
		}
		else if (a_form->Is(RE::FormType::Spell) || a_form->Is(RE::FormType::LeveledSpell))
		{
			auto* spell = a_form->As<RE::SpellItem>();
			has_it      = player->HasSpell(spell);
		}
		else if (a_form->Is(RE::FormType::AlchemyItem))
		{
			has_it = get_inventory_count(a_form, RE::FormType::AlchemyItem, player) > 0;
		}
		else if (a_form->Is(RE::FormType::Scroll))
		{
			has_it = get_inventory_count(a_form, RE::FormType::Scroll, player) > 0;
		}
		else if (a_form->Is(RE::FormType::Shout))
		{
			const auto shout = a_form->As<RE::TESShout>();
			has_it           = has_shout(player, shout);
		}

		logger::trace("Player has item/spell/shout {}, name {}, form {} "sv,
			has_it,
			a_form->GetName(),
			util::string_util::int_to_hex(a_form->formID));

		return has_it;
	}

	uint32_t get_inventory_count(const RE::TESForm* a_form, RE::FormType a_type, RE::PlayerCharacter*& a_player)
	{
		auto count     = 0;
		auto inventory = get_inventory(a_player, a_type);
		for (const auto& [item, inv_data] : inventory)
		{
			if (const auto& [num_items, entry] = inv_data; entry->object->formID == a_form->formID)
			{
				count = num_items;
				break;
			}
		}
		return count;
	}

	bool has_shout(RE::Actor* a_actor, RE::TESShout* a_shout)
	{
		using func_t = decltype(&has_shout);
		REL::Relocation<func_t> func{ offset::has_shout };
		return func(a_actor, a_shout);
	}

	void play_sound(RE::BGSSoundDescriptor* a_sound_descriptor, RE::PlayerCharacter*& a_player)
	{
		auto* audio_manager = RE::BSAudioManager::GetSingleton();
		if (audio_manager && a_sound_descriptor)
		{
			RE::BSSoundHandle sound_handle;
			audio_manager->BuildSoundDataFromDescriptor(sound_handle, a_sound_descriptor);
			sound_handle.SetObjectToFollow(a_player->Get3D());
			sound_handle.SetVolume(1.0);
			sound_handle.Play();
			logger::trace("played sound"sv);
		}
	}
}  // util

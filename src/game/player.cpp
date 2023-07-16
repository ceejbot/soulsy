#include "player.h"

#include "enums.h"
#include "equippable.h"
#include "gear.h"
#include "magic.h"
#include "utility_items.h"

#include "helpers.h"
#include "offset.h"
#include "string_util.h"

#include "lib.rs.h"

namespace player
{
	using string_util = util::string_util;
	using slot_type   = enums::slot_type;
	using action_type = enums::action_type;

	rust::Box<TesItemData> equippedLeftHand()
	{
		auto* player   = RE::PlayerCharacter::GetSingleton();
		const auto obj = player->GetActorRuntimeData().currentProcess->GetEquippedLeftHand();
		if (!obj)
			return default_cycle_entry();
		auto* item_form = RE::TESForm::LookupByID(obj->formID);
		if (!item_form)
			return default_cycle_entry();
		return equippable::makeTESItemDataFromForm(item_form);
	}

	rust::Box<TesItemData> equippedRightHand()
	{
		auto* player = RE::PlayerCharacter::GetSingleton();

		const auto obj = player->GetActorRuntimeData().currentProcess->GetEquippedRightHand();
		if (!obj)
			return default_cycle_entry();
		auto* item_form = RE::TESForm::LookupByID(obj->formID);
		if (!item_form)
			return default_cycle_entry();
		return equippable::makeTESItemDataFromForm(item_form);
	}

	rust::Box<TesItemData> equippedPower()
	{
		auto* player    = RE::PlayerCharacter::GetSingleton();
		const auto* obj = player->GetActorRuntimeData().selectedPower;
		if (!obj)
			return default_cycle_entry();
		auto* item_form = RE::TESForm::LookupByID(obj->formID);
		if (!item_form)
			return default_cycle_entry();
		return equippable::makeTESItemDataFromForm(item_form);
	}

	rust::Box<TesItemData> equippedAmmo()
	{
		auto player        = RE::PlayerCharacter::GetSingleton();
		auto* current_ammo = player->GetCurrentAmmo();
		if (!current_ammo || !current_ammo->IsAmmo())
		{
			return default_cycle_entry();
		}

		const auto formspec = helpers::makeFormSpecString(current_ammo);
		auto count          = inventoryCount(current_ammo, RE::FormType::Ammo, player);
		return create_tesitem_shim(EntryKind::Arrow, false, true, count, current_ammo->GetName(), formspec);
	}

	void unequipSlot(Action which)
	{
		auto* player = RE::PlayerCharacter::GetSingleton();

		if (which == Action::Power)
		{
			equip::unequipShoutSlot(player);
		}
		else if (which == Action::Right || which == Action::Left)
		{
			equip::unequipHand(player, which);
		}
		else
		{
			logger::debug("somebody called unequipSlot() with slot={};"sv, static_cast<uint8_t>(which));
		}
	}

	void unequipShout()
	{
		auto* player = RE::PlayerCharacter::GetSingleton();
		equip::unequipShoutSlot(player);
	}

	void equipShout(const std::string& form_spec)
	{
		auto* shout_form = helpers::formSpecToFormItem(form_spec);
		if (!shout_form)
		{
			return;
		}
		auto* player = RE::PlayerCharacter::GetSingleton();
		equip::equipShoutByForm(shout_form, player);
	}

	void equipMagic(const std::string& form_spec, Action slot)
	{
		auto* form = helpers::formSpecToFormItem(form_spec);
		if (!form)
		{
			return;
		}
		auto* player     = RE::PlayerCharacter::GetSingleton();
		auto* equip_slot = (slot == Action::Left ? equip::left_hand_equip_slot() : equip::right_hand_equip_slot());
		equip::equip_item(form, equip_slot, player);
	}

	void equipWeapon(const std::string& form_spec, Action slot)
	{
		auto* form = helpers::formSpecToFormItem(form_spec);
		if (!form)
		{
			return;
		}
		auto* player     = RE::PlayerCharacter::GetSingleton();
		auto* equip_slot = (slot == Action::Left ? equip::left_hand_equip_slot() : equip::right_hand_equip_slot());
		equip::equip_item(form, equip_slot, player);
	}

	void equipArmor(const std::string& form_spec)
	{
		auto* form = helpers::formSpecToFormItem(form_spec);
		if (!form)
		{
			return;
		}
		auto* player = RE::PlayerCharacter::GetSingleton();
		equip::equipArmorByForm(form, player);
	}

	void equipAmmo(const std::string& form_spec)
	{
		auto* form = helpers::formSpecToFormItem(form_spec);
		if (!form)
		{
			return;
		}
		auto* player = RE::PlayerCharacter::GetSingleton();
		equip::equip_ammo(form, player);
	}

	void consumePotion(const std::string& form_spec)
	{
		auto* form = helpers::formSpecToFormItem(form_spec);
		if (!form)
		{
			return;
		}
		auto* player = RE::PlayerCharacter::GetSingleton();
		equip::consume_potion(form, player);
	}

	std::map<RE::TESBoundObject*, std::pair<int, std::unique_ptr<RE::InventoryEntryData>>>
		get_inventory(RE::PlayerCharacter*& a_player, RE::FormType a_type)
	{
		return a_player->GetInventory([a_type](const RE::TESBoundObject& a_object) { return a_object.Is(a_type); });
	}

	uint32_t getInventoryCountByForm(const RE::TESForm* form)
	{
		uint32_t count = 0;
		if (!form)
		{
			return count;
		}

		auto* player = RE::PlayerCharacter::GetSingleton();
		count        = inventoryCount(form, form->GetFormType(), player);
		logger::trace("item='{}'; count={};"sv, form->GetName(), count);

		return count;
	}

	bool playerHasItemOrSpell(const std::string& form_spec)
	{
		auto* form = helpers::formSpecToFormItem(form_spec);
		if (!form)
		{
			return false;
		}

		auto* player = RE::PlayerCharacter::GetSingleton();
		auto has_it  = false;
		if (form->IsWeapon())
		{
			has_it = inventoryCount(form, RE::FormType::Weapon, player) > 0;
		}
		else if (form->IsArmor())
		{
			has_it = inventoryCount(form, RE::FormType::Armor, player) > 0;
		}
		else if (form->Is(RE::FormType::Light))
		{
			has_it = inventoryCount(form, RE::FormType::Light, player) > 0;
		}
		else if (form->Is(RE::FormType::Spell) || form->Is(RE::FormType::LeveledSpell))
		{
			auto* spell = form->As<RE::SpellItem>();
			has_it      = player->HasSpell(spell);
		}
		else if (form->Is(RE::FormType::AlchemyItem))
		{
			has_it = inventoryCount(form, RE::FormType::AlchemyItem, player) > 0;
		}
		else if (form->Is(RE::FormType::Scroll))
		{
			has_it = inventoryCount(form, RE::FormType::Scroll, player) > 0;
		}
		else if (form->Is(RE::FormType::Shout))
		{
			const auto shout = form->As<RE::TESShout>();
			has_it           = has_shout(player, shout);
		}

		logger::trace("player has: {}; name='{}'; formid=0x{};"sv,
			has_it,
			form->GetName(),
			util::string_util::int_to_hex(form->formID));

		return has_it;
	}

	void reequipLeftHand(const std::string& form_spec)
	{
		auto* form = helpers::formSpecToFormItem(form_spec);
		if (!form)
		{
			return;
		}

		auto* player    = RE::PlayerCharacter::GetSingleton();

		RE::TESBoundObject* bound_obj = nullptr;
		equip::boundObjectForForm(form, player, bound_obj);
		if (!bound_obj)
		{
			return;
		}

		logger::info("re-equipping item in left hand; name=''; formid=0x{}"sv,
			form->GetName(),
			util::string_util::int_to_hex(form->formID));
		// auto* left_slot = equip::left_hand_equip_slot();
		// equip::unequip_this_slot(left_slot, player);
		auto* task = SKSE::GetTaskInterface();
		if (task)
		{
			task->AddTask([=]() { RE::ActorEquipManager::GetSingleton()->EquipObject(player, bound_obj); });
		}
	}

	uint32_t inventoryCount(const RE::TESForm* a_form, RE::FormType a_type, RE::PlayerCharacter*& a_player)
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

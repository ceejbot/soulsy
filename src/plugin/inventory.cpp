#include "inventory.h"

#include "equippable.h"
#include "gear.h"
#include "hooks.h"
#include "string_util.h"

#include "lib.rs.h"

void install_hooks()
{
	PlayerHook::install();
	MenuHook::install();
}

inline const std::set<RE::FormType> RELEVANT_FORMTYPES_INVENTORY{
	RE::FormType::AlchemyItem,
	RE::FormType::Ammo,
	RE::FormType::Armor,
	RE::FormType::Light,
	RE::FormType::Scroll,
	RE::FormType::Weapon,
};

inline const std::set<RE::FormType> RELEVANT_FORMTYPES_ALL{
	RE::FormType::AlchemyItem,
	RE::FormType::Ammo,
	RE::FormType::Armor,
	RE::FormType::Light,
	RE::FormType::Scroll,
	RE::FormType::Shout,
	RE::FormType::Spell,
	RE::FormType::Weapon,
};

// ---------- PlayerHook

void PlayerHook::install()
{
	logger::info("Hooking player so we get equip events plus inventory changes..."sv);

	REL::Relocation<std::uintptr_t> player_character_vtbl{ RE::VTABLE_PlayerCharacter[0] };
	add_object_to_container_ = player_character_vtbl.write_vfunc(0x5A, add_object_to_container);
	pick_up_object_          = player_character_vtbl.write_vfunc(0xCC, pick_up_object);
	remove_item_             = player_character_vtbl.write_vfunc(0x56, remove_item);

	auto& trampoline = SKSE::GetTrampoline();
	REL::Relocation<std::uintptr_t> add_item_functor_hook{ RELOCATION_ID(55946, 56490) };
	add_item_functor_ = trampoline.write_call<5>(add_item_functor_hook.address() + 0x15D, add_item_functor);
}

void PlayerHook::add_object_to_container(RE::Actor* a_this,
	RE::TESBoundObject* object,
	RE::ExtraDataList* extraDataList,
	int32_t delta,
	RE::TESObjectREFR* a_from_refr)
{
	add_object_to_container_(a_this, object, extraDataList, delta, a_from_refr);
	if (object->IsInventoryObject())
	{
		auto item_form = RE::TESForm::LookupByID(object->formID);
		notifyInventoryChanged(item_form);
	}
}

void PlayerHook::pick_up_object(RE::Actor* actor,
	RE::TESObjectREFR* object,
	uint32_t delta,
	bool a_arg3,
	bool a_play_sound)
{
	pick_up_object_(actor, object, delta, a_arg3, a_play_sound);
	if (object->GetBaseObject()->IsInventoryObject())
	{
		auto lookup = object->formID;
		if (lookup == 0) { lookup = object->GetBaseObject()->formID; }
		auto item_form = RE::TESForm::LookupByID(lookup);
		notifyInventoryChanged(item_form);
	}
}

RE::ObjectRefHandle PlayerHook::remove_item(RE::Actor* actor,
	RE::TESBoundObject* object,
	std::int32_t delta,
	RE::ITEM_REMOVE_REASON a_reason,
	RE::ExtraDataList* extraDataList,
	RE::TESObjectREFR* a_move_to_ref,
	const RE::NiPoint3* a_drop_loc,
	const RE::NiPoint3* a_rotate)
{
	auto retval = remove_item_(actor, object, delta, a_reason, extraDataList, a_move_to_ref, a_drop_loc, a_rotate);
	if (object->IsInventoryObject())
	{
		auto* item_form = RE::TESForm::LookupByID(object->formID);
		notifyInventoryChanged(item_form);
	}
	return retval;
}

void PlayerHook::add_item_functor(RE::TESObjectREFR* a_this, RE::TESObjectREFR* object, int32_t delta, bool a4, bool a5)
{
	add_item_functor_(a_this, object, delta, a4, a5);
	auto item_form = RE::TESForm::LookupByID(object->GetBaseObject()->formID);
	notifyInventoryChanged(item_form);
}

void PlayerHook::notifyInventoryChanged(RE::TESForm* item_form)
{
	if (!item_form) { return; }

	// We do not pass along all inventory changes to the HUD, only changes
	// for the kinds of items the HUD is used to show.
	const auto formtype = item_form->GetFormType();
	if (!RELEVANT_FORMTYPES_INVENTORY.contains(formtype)) { return; }

	auto count              = player::getInventoryCountByForm(item_form);
	std::string form_string = helpers::makeFormSpecString(item_form);
	handle_inventory_changed(form_string, count);
}

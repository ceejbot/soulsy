#include "hooks.h"
#include "equippable.h"

#include "lib.rs.h"

namespace hooks
{
	void install_hooks()
	{
		PlayerHook::install();
		MenuHook::install();
	}

	// ---------- MenuHook

	void MenuHook::install()
	{
		logger::info("Hooking menu..."sv);

		REL::Relocation<std::uintptr_t> menu_controls_vtbl{ RE::VTABLE_MenuControls[0] };
		process_event_ = menu_controls_vtbl.write_vfunc(0x1, &MenuHook::process_event);

		logger::info("Menu hooked."sv);
		didControlMapDump = false;
	}

	RE::BSEventNotifyControl MenuHook::process_event(RE::InputEvent** eventPtr,
		RE::BSTEventSource<RE::InputEvent*>* eventSource)
	{
		auto* ui                        = RE::UI::GetSingleton();
		rust::Box<UserSettings> hotkeys = user_settings();
		auto relevantMenuOpen           = helpers::relevantMenuOpen();

		if (ui->IsMenuOpen("LootMenu") || !relevantMenuOpen) { return process_event_(this, eventPtr, eventSource); }

		if (!didControlMapDump)
		{
			auto* ctrlMaps = RE::ControlMap::GetSingleton();
			for (auto i = 0; i < RE::UserEvents::INPUT_CONTEXT_ID::InputContextID::kTotal; i++)
			{
				const auto* ctx = ctrlMaps->controlMap[i];
				if (!ctx) continue;
				// BSTArray<UserEventMapping> deviceMappings[INPUT_DEVICES::kTotal];
				for (auto j = 0; j < RE::INPUT_DEVICE::INPUT_DEVICES::kTotal; j++)
				{
					const auto* deviceMappings = ctx->deviceMappings[j];
					if (!deviceMappings) continue;
					for (auto& mapping : deviceMappings)
					{
						logger::info("{}  {}  eventid={}; key={};"sv i, j, mapping.eventID, mapping.inputKey);
					}
				}
			}
			didControlMapDump = true;
		}

		// auto* mapping = RE::ControlMap::GetSingleton();
		// auto favconst = RE::UserEvents::GetSingleton()->toggleFavorite;
		// auto favkey = mapping->GetMappedKey(favconst, RE::INPUT_DEVICE::kKeyboard, RE::UserEvents::INPUT_CONTEXT_ID::kInventory);
		// auto favbutton = mapping->GetMappedKey(favconst, RE::INPUT_DEVICE::kGamepad, RE::UserEvents::INPUT_CONTEXT_ID::kInventory);

		if (eventPtr && *eventPtr)
		{
			for (auto* event = *eventPtr; event; event = event->next)
			{
				if (event->eventType != RE::INPUT_EVENT_TYPE::kButton || !event->HasIDCode()) { continue; }

				auto* button = static_cast<RE::ButtonEvent*>(event);
				if (button->idCode == keycodes::k_invalid) { continue; }

				auto key = keycodes::get_key_id(button);
				/*
				if (button->IsUp()) {
					auto underlying = button->GetDevice();
					auto maybe = mapping->GetMappedKey(favconst, underlying, RE::UserEvents::INPUT_CONTEXT_ID::kInventory);
					logger::info("is this a {} key? (points at butterfly) {}; favkey={} maybe={}"sv, favconst, key, favkey, maybe);
				}
				*/

				// we send all key events to this handler because it needs to track modifiers
				auto do_toggle = handle_menu_event(key, *button);

				if (do_toggle)
				{
					auto menu_form = helpers::getSelectedFormFromMenu(ui);
					if (!menu_form) continue;

					auto* item_form = RE::TESForm::LookupByID(menu_form);
					if (!item_form) continue;

					auto entry = equippable::makeItemDataFromForm(item_form);
					toggle_item(key, std::move(entry));
				}
			}
		}

		return process_event_(this, eventPtr, eventSource);
	}

	// ---------- PlayerHook

	void PlayerHook::install()
	{
		logger::info("Hooking player..."sv);

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
		int32_t count,
		RE::TESObjectREFR* a_from_refr)
	{
		// TODO update counts for consumables if we need to, or otherwise update the controller
		add_object_to_container_(a_this, object, extraDataList, count, a_from_refr);

		if (object->IsInventoryObject())
		{
			auto item_form = RE::TESForm::LookupByID(object->formID);
			if (item_form)
			{
				auto shim = equippable::makeItemDataFromForm(item_form);
				handle_inventory_changed(std::move(shim), count);
			}
		}
	}

	void PlayerHook::pick_up_object(RE::Actor* actor,
		RE::TESObjectREFR* object,
		uint32_t count,
		bool a_arg3,
		bool a_play_sound)
	{
		pick_up_object_(actor, object, count, a_arg3, a_play_sound);
		if (object->GetBaseObject()->IsInventoryObject())
		{
			auto item_form = RE::TESForm::LookupByID(object->formID);
			if (!item_form) { return; }
			auto shim = equippable::makeItemDataFromForm(item_form);
			handle_inventory_changed(std::move(shim), count);
		}
	}

	RE::ObjectRefHandle PlayerHook::remove_item(RE::Actor* actor,
		RE::TESBoundObject* object,
		std::int32_t count,
		RE::ITEM_REMOVE_REASON a_reason,
		RE::ExtraDataList* extraDataList,
		RE::TESObjectREFR* a_move_to_ref,
		const RE::NiPoint3* a_drop_loc,
		const RE::NiPoint3* a_rotate)
	{
		if (object->IsInventoryObject())
		{
			auto* item_form = RE::TESForm::LookupByID(object->formID);
			if (item_form)
			{
				auto shim = equippable::makeItemDataFromForm(item_form);
				handle_inventory_changed(std::move(shim), -count);
			}
		}

		return remove_item_(actor, object, count, a_reason, extraDataList, a_move_to_ref, a_drop_loc, a_rotate);
	}

	void PlayerHook::add_item_functor(RE::TESObjectREFR* a_this,
		RE::TESObjectREFR* object,
		int32_t count,
		bool a4,
		bool a5)
	{
		add_item_functor_(a_this, object, count, a4, a5);

		if (object->GetBaseObject()->IsInventoryObject())
		{
			auto item_form = RE::TESForm::LookupByID(object->GetBaseObject()->formID);
			if (item_form)
			{
				auto item = equippable::makeItemDataFromForm(item_form);
				handle_inventory_changed(std::move(item), count);
			}
		}
	}
}

#include "hooks.h"
#include "equippable.h"
#include "gear.h"
#include "keycodes.h"
#include "string_util.h"

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
		logger::info("Hooking menus to get keystrokes..."sv);

		REL::Relocation<std::uintptr_t> menu_controls_vtbl{ RE::VTABLE_MenuControls[0] };
		process_event_ = menu_controls_vtbl.write_vfunc(0x1, &MenuHook::process_event);

		logger::info("Menus hooked."sv);
	}

	RE::BSEventNotifyControl MenuHook::process_event(RE::InputEvent** eventPtr,
		RE::BSTEventSource<RE::InputEvent*>* eventSource)
	{
		auto* ui = RE::UI::GetSingleton();
		if (!ui) return process_event_(this, eventPtr, eventSource);

		rust::Box<UserSettings> settings = user_settings();
		bool check_favorites             = settings->link_to_favorites();
		auto relevantMenuOpen            = helpers::relevantMenuOpen();

		auto* controlMap = RE::ControlMap::GetSingleton();
		auto* userEvents = RE::UserEvents::GetSingleton();
		if (!controlMap || !userEvents) return process_event_(this, eventPtr, eventSource);
		auto favconst  = userEvents->togglePOV;       // m&k shortcut
		auto favtoggle = userEvents->jump;  // controller shortcut

		if (ui->IsMenuOpen("LootMenu") || !relevantMenuOpen) { return process_event_(this, eventPtr, eventSource); }

		if (eventPtr && *eventPtr)
		{
			for (auto* event = *eventPtr; event; event = event->next)
			{
				if (event->eventType != RE::INPUT_EVENT_TYPE::kButton || !event->HasIDCode()) { continue; }

				auto* button = static_cast<RE::ButtonEvent*>(event);
				if (button->idCode == keycodes::k_invalid) { continue; }
				auto key = keycodes::get_key_id(button);

				if (button->IsUp() && check_favorites)
				{
					if (buttonMatchesEvent(controlMap, favconst, button) ||
						buttonMatchesEvent(controlMap, favtoggle, button))
					{
						helpers::MenuSelection* selection = nullptr;
						auto menu_form                    = helpers::MenuSelection::getSelectionFromMenu(ui, selection);
						if (!menu_form) continue;

						logger::debug("Got toggled favorite: form_id={}; is-favorited={}"sv,
							util::string_util::int_to_hex(selection->form_id),
							selection->favorite);

						if (selection->bound_obj)
						{
							auto entry = equippable::makeItemDataFromForm(selection->bound_obj);
							logger::trace("got bound object; name='{}'; kind={};"sv,
								selection->bound_obj->GetName(),
								static_cast<uint8_t>(entry->kind()));
							handle_favorite_event(*button, selection->favorite, std::move(entry));
						}
						else if (selection->form)
						{
							auto entry = equippable::makeItemDataFromForm(selection->form);
							logger::trace("got form; name='{}'; kind={}"sv,
								selection->form->GetName(),
								static_cast<uint8_t>(entry->kind()));
							handle_favorite_event(*button, selection->favorite, std::move(entry));
						}
						continue;
					}
				}

				// we send all key events to this handler because it needs to track modifiers
				auto do_toggle = handle_menu_event(key, *button);

				if (do_toggle)
				{
					helpers::MenuSelection* selection = nullptr;
					auto menu_form                    = helpers::MenuSelection::getSelectionFromMenu(ui, selection);
					if (!menu_form) continue;

					auto* item_form = selection->form;
					if (!item_form) continue;

					auto entry = equippable::makeItemDataFromForm(item_form);
					toggle_item(key, std::move(entry));
				}
			}
		}

		return process_event_(this, eventPtr, eventSource);
	}

	using INPUT_CONTEXT_ID = RE::UserEvents::INPUT_CONTEXT_IDS::INPUT_CONTEXT_ID;

	bool MenuHook::buttonMatchesEvent(RE::ControlMap* controlMap, RE::BSFixedString eventID, RE::ButtonEvent* button)
	{
		auto the_device  = button->GetDevice();
		auto key = controlMap->GetMappedKey(eventID, the_device, static_cast<RE::ControlMap::InputContextID>(0));
		return key == button->GetIDCode();
	}

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

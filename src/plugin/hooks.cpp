#include "include/hooks.h"
#include "include/user_settings.h"

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
	}

	RE::BSEventNotifyControl MenuHook::process_event(RE::InputEvent** eventPtr,
		RE::BSTEventSource<RE::InputEvent*>* eventSource)
	{
		auto* ui          = RE::UI::GetSingleton();
		rust::Box<UserSettings> hotkeys     = user_settings();
		auto* user_event  = RE::UserEvents::GetSingleton();
		auto* control_map = RE::ControlMap::GetSingleton();

		if (eventPtr && *eventPtr && processing::game_menu_setting::relevant_menu_open(ui))
		{
			for (auto* event = *eventPtr; event; event = event->next)
			{
				if (event->eventType != RE::INPUT_EVENT_TYPE::kButton)
				{
					continue;
				}

				if (event->HasIDCode())
				{
					auto* button = static_cast<RE::ButtonEvent*>(event);
					if (button->idCode == keycodes::k_invalid)
					{
						continue;
					}

					auto key = keycodes::get_key_id(button);


					if (button->IsUp())
					{
						// If this is a slot cycle button, start the equip timer now.
						// TODO
					}

					// Early return after we're finished processing button-up events.
					if (!button->IsDown())
					{
						continue;
					}

					if (button->IsPressed() && hotkeys->is_cycle_button(key))
					{

						auto menu_form = processing::game_menu_setting::get_selected_form(ui);
						if (menu_form)
						{
							// TOOD: okay! Time to figure out how to send form info over to Rust.
							auto* tes_form_menu = RE::TESForm::LookupByID(menu_form);
							if (!tes_form_menu)
							{
								// I don't like null pointer exceptions, I guess.
								continue;
							}
							
							rust::Box<CycleEntry> entry = create_cycle_entry(kind
																			 : EntryIcon, two_handed
																			 : bool, has_count
																			 : bool, count
																			 : usize, form_string
																			 : &str);
							MenuEventResponse response = handle_menu_event(key, entry);
							logger::info("got result code {} from menu event for {}"sv, response, key);
							// TODO vary this response notification based on the result
							// write_notification(fmt::format("Added Item {}", a_form ? a_form->GetName() : "null"));
							// here the old code wrote the config out; we've already done that in rust
						}
					}
				}
			}
		}
		return process_event_(this, eventPtr, eventSource);
	}

	bool MenuHook::need_to_overwrite(RE::ButtonEvent*& a_button,
		RE::UserEvents*& a_user_event,
		RE::ControlMap*& a_control_map) const
	{
		auto button_event = a_button->userEvent;
		if (button_event == a_user_event->up || button_event == a_user_event->right ||
			button_event == a_user_event->down || button_event == a_user_event->left ||
			button_event == a_user_event->strafeRight || button_event == a_user_event->strafeLeft ||
			button_event == a_user_event->forward || button_event == a_user_event->back ||
			button_event == a_user_event->pageUp || button_event == a_user_event->nextPage ||
			button_event == a_user_event->pageDown || button_event == a_user_event->prevPage)
		{
			return true;
		}

		auto device = a_button->device.get();
		auto key    = keycodes::get_key_id(a_button);

		if (key == a_control_map->GetMappedKey(a_user_event->up, device) ||
			key == a_control_map->GetMappedKey(a_user_event->right, device) ||
			key == a_control_map->GetMappedKey(a_user_event->down, device) ||
			key == a_control_map->GetMappedKey(a_user_event->left, device) ||
			key == a_control_map->GetMappedKey(a_user_event->strafeRight, device) ||
			key == a_control_map->GetMappedKey(a_user_event->strafeLeft, device) ||
			key == a_control_map->GetMappedKey(a_user_event->forward, device) ||
			key == a_control_map->GetMappedKey(a_user_event->back, device) ||
			key == a_control_map->GetMappedKey(a_user_event->pageUp, device) ||
			key == a_control_map->GetMappedKey(a_user_event->nextPage, device) ||
			key == a_control_map->GetMappedKey(a_user_event->pageDown, device) ||
			key == a_control_map->GetMappedKey(a_user_event->prevPage, device))
		{
			return true;
		}
		return false;
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

		logger::info("Hooked."sv);
	}

	void PlayerHook::add_object_to_container(RE::Actor* a_this,
		RE::TESBoundObject* object,
		RE::ExtraDataList* extraDataList,
		int32_t count,
		RE::TESObjectREFR* a_from_refr)
	{
		add_object_to_container_(a_this, object, extraDataList, count, a_from_refr);

		if (object->IsInventoryObject())
		{
			processing::set_setting_data::set_new_item_count_if_needed(object, count);
		}
	}

	void PlayerHook::pick_up_object(RE::Actor* a_this,
		RE::TESObjectREFR* object,
		uint32_t count,
		bool a_arg3,
		bool a_play_sound)
	{
		pick_up_object_(a_this, object, count, a_arg3, a_play_sound);

		if (object->GetBaseObject()->IsInventoryObject())
		{
			processing::set_setting_data::set_new_item_count_if_needed(object->GetBaseObject(),
				static_cast<int32_t>(count));
		}
	}

	RE::ObjectRefHandle PlayerHook::remove_item(RE::Actor* a_this,
		RE::TESBoundObject* a_item,
		std::int32_t count,
		RE::ITEM_REMOVE_REASON a_reason,
		RE::ExtraDataList* extraDataList,
		RE::TESObjectREFR* a_move_to_ref,
		const RE::NiPoint3* a_drop_loc,
		const RE::NiPoint3* a_rotate)
	{
		if (a_item->IsInventoryObject())
		{
			processing::set_setting_data::set_new_item_count_if_needed(a_item, -count);
		}

		return remove_item_(a_this, a_item, count, a_reason, extraDataList, a_move_to_ref, a_drop_loc, a_rotate);
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
			processing::set_setting_data::set_new_item_count_if_needed(object->GetBaseObject(),
				static_cast<int32_t>(count));
		}
	}

}

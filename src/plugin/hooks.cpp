#include "hooks.h"

#include "handle/key_position_handle.h"
#include "processing/game_menu_setting.h"
#include "processing/set_setting_data.h"
#include "setting/mcm_setting.h"

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

	RE::BSEventNotifyControl MenuHook::process_event(RE::InputEvent** a_event,
		RE::BSTEventSource<RE::InputEvent*>* a_source)
	{
		auto* ui          = RE::UI::GetSingleton();
		auto* binding     = control::binding::get_singleton();
		auto edit_key     = config::mcm_setting::get_key_press_to_enter_edit();
		auto* user_event  = RE::UserEvents::GetSingleton();
		auto* control_map = RE::ControlMap::GetSingleton();

		if (a_event && *a_event && processing::game_menu_setting::relevant_menu_open(ui))
		{
			for (auto* event = *a_event; event; event = event->next)
			{
				if (event->eventType != RE::INPUT_EVENT_TYPE::kButton)
				{
					continue;
				}

				if (event->HasIDCode())
				{
					auto* button = static_cast<RE::ButtonEvent*>(event);

					key_ = button->idCode;
					if (key_ == common::k_invalid)
					{
						continue;
					}

					common::get_key_id(button, key_);

					if (button->IsUp())
					{
						if (binding->get_is_edit_down() &&
							((edit_key && common::is_key_valid_and_matches(key_, binding->get_edit_key())) ||
								common::is_key_valid_and_matches(key_, binding->get_bottom_execute_or_toggle_action())))
						{
							binding->set_is_edit_down(false);
						}

						if (binding->get_is_edit_left_down() &&
							common::is_key_valid_and_matches(key_, binding->get_edit_key_left_or_overwrite()))
						{
							binding->set_is_edit_left_down(false);
						}

						if (binding->get_is_remove_down() &&
							common::is_key_valid_and_matches(key_, binding->get_remove_key()))
						{
							binding->set_is_remove_down(false);
						}
					}

					if (button->IsDown())
					{
						if (common::is_key_valid_and_matches(key_, binding->get_bottom_execute_or_toggle_action()) ||
							(config::mcm_setting::get_key_press_to_enter_edit() &&
								common::is_key_valid_and_matches(key_, binding->get_edit_key())))
						{
							binding->set_is_edit_down(true);
						}

						if (common::is_key_valid_and_matches(key_, binding->get_edit_key_left_or_overwrite()))
						{
							binding->set_is_edit_left_down(true);
						}

						if (common::is_key_valid_and_matches(key_, binding->get_remove_key()))
						{
							binding->set_is_remove_down(true);
						}
					}

					if (need_to_overwrite(button, user_event, control_map) &&
						(binding->get_is_edit_down() || binding->get_is_edit_left_down() ||
							binding->get_is_remove_down()))
					{
						button->idCode    = common::k_invalid;
						button->userEvent = "";
					}

					if (!button->IsDown())
					{
						continue;
					}

					if (!binding->get_is_edit_down() && !binding->get_is_edit_left_down() &&
						!binding->get_is_remove_down())
					{
						continue;
					}

					if (button->IsPressed() && binding->is_position_button(key_))
					{
						auto menu_form = processing::game_menu_setting::get_selected_form(ui);
						if (menu_form)
						{
							auto* tes_form_menu = RE::TESForm::LookupByID(menu_form);
							auto key_position =
								handle::key_position_handle::get_singleton()->get_position_for_key(key_);
							if (binding->get_is_remove_down())
							{
								logger::trace("doing remove for form"sv);
								processing::set_setting_data::default_remove(tes_form_menu);
							}
							else
							{
								logger::trace("doing add or place for form."sv);
								if (config::mcm_setting::get_elden_demon_souls())
								{
									processing::game_menu_setting::elden_souls_config(tes_form_menu,
										key_position,
										binding->get_is_edit_left_down());
								}
								else
								{
									processing::game_menu_setting::default_config(tes_form_menu,
										key_position,
										binding->get_is_edit_left_down());
								}
							}
						}
					}
				}
			}
		}
		return process_event_(this, a_event, a_source);
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
		if (key_ == a_control_map->GetMappedKey(a_user_event->up, device) ||
			key_ == a_control_map->GetMappedKey(a_user_event->right, device) ||
			key_ == a_control_map->GetMappedKey(a_user_event->down, device) ||
			key_ == a_control_map->GetMappedKey(a_user_event->left, device) ||
			key_ == a_control_map->GetMappedKey(a_user_event->strafeRight, device) ||
			key_ == a_control_map->GetMappedKey(a_user_event->strafeLeft, device) ||
			key_ == a_control_map->GetMappedKey(a_user_event->forward, device) ||
			key_ == a_control_map->GetMappedKey(a_user_event->back, device) ||
			key_ == a_control_map->GetMappedKey(a_user_event->pageUp, device) ||
			key_ == a_control_map->GetMappedKey(a_user_event->nextPage, device) ||
			key_ == a_control_map->GetMappedKey(a_user_event->pageDown, device) ||
			key_ == a_control_map->GetMappedKey(a_user_event->prevPage, device))
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
		RE::TESBoundObject* a_object,
		RE::ExtraDataList* a_extra_list,
		int32_t a_count,
		RE::TESObjectREFR* a_from_refr)
	{
		add_object_to_container_(a_this, a_object, a_extra_list, a_count, a_from_refr);

		if (a_object->IsInventoryObject())
		{
			processing::set_setting_data::set_new_item_count_if_needed(a_object, a_count);
		}
	}

	void PlayerHook::pick_up_object(RE::Actor* a_this,
		RE::TESObjectREFR* a_object,
		uint32_t a_count,
		bool a_arg3,
		bool a_play_sound)
	{
		pick_up_object_(a_this, a_object, a_count, a_arg3, a_play_sound);

		if (a_object->GetBaseObject()->IsInventoryObject())
		{
			processing::set_setting_data::set_new_item_count_if_needed(a_object->GetBaseObject(),
				static_cast<int32_t>(a_count));
		}
	}

	RE::ObjectRefHandle PlayerHook::remove_item(RE::Actor* a_this,
		RE::TESBoundObject* a_item,
		std::int32_t a_count,
		RE::ITEM_REMOVE_REASON a_reason,
		RE::ExtraDataList* a_extra_list,
		RE::TESObjectREFR* a_move_to_ref,
		const RE::NiPoint3* a_drop_loc,
		const RE::NiPoint3* a_rotate)
	{
		if (a_item->IsInventoryObject())
		{
			processing::set_setting_data::set_new_item_count_if_needed(a_item, -a_count);
		}

		return remove_item_(a_this, a_item, a_count, a_reason, a_extra_list, a_move_to_ref, a_drop_loc, a_rotate);
	}

	void PlayerHook::add_item_functor(RE::TESObjectREFR* a_this,
		RE::TESObjectREFR* a_object,
		int32_t a_count,
		bool a4,
		bool a5)
	{
		add_item_functor_(a_this, a_object, a_count, a4, a5);

		if (a_object->GetBaseObject()->IsInventoryObject())
		{
			processing::set_setting_data::set_new_item_count_if_needed(a_object->GetBaseObject(),
				static_cast<int32_t>(a_count));
		}
	}

}

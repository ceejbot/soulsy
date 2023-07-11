#include "helper.h"
#include "equippable.h"
#include "keycodes.h"
#include "player.h"
#include "sinks.h"
#include "ui_renderer.h"
#include "user_settings.h"

#include "handle/extra_data_holder.h"
#include "handle/name_handle.h"
#include "processing/set_setting_data.h"
#include "processing/setting_execute.h"

#include "lib.rs.h"

// Handle equipment change events. We need to update our UI when this happens.

// Where all == both.
void register_all_sinks()
{
	EquipEventSink::register_sink();
	KeyEventSink::register_sink();
}

EquipEventSink* EquipEventSink::get_singleton()
{
	static EquipEventSink singleton;
	return std::addressof(singleton);
}

void EquipEventSink::register_sink() { RE::ScriptEventSourceHolder::GetSingleton()->AddEventSink(get_singleton()); }

EquipEventSink::event_result EquipEventSink::ProcessEvent(const RE::TESEquipEvent* event,
	[[maybe_unused]] RE::BSTEventSource<RE::TESEquipEvent>* source)
{
	if (!event || !event->actor || !event->actor->IsPlayerRef())
	{
		return event_result::kContinue;
	}

	auto* form = RE::TESForm::LookupByID(event->baseObject);
	if (!form)
	{
		return event_result::kContinue;
	}

	// Here we want to turn over the processing to the Rust side.
	// which does not yet exist

	if (config::mcm_setting::get_draw_current_items_text() &&
		(form->IsWeapon() || form->Is(RE::FormType::Spell) || form->IsAmmo() || form->Is(RE::FormType::Light)))
	{
		handle::name_handle::get_singleton()->init_names(player::get_hand_assignment());
	}

	if (config::mcm_setting::get_draw_current_shout_text() && form->Is(RE::FormType::Shout) ||
		form->Is(RE::FormType::Spell))
	{
		// call function there and check selected power, spell trigger and spells as well but that is ok for now
		handle::name_handle::get_singleton()->init_voice_name(
			RE::PlayerCharacter::GetSingleton()->GetActorRuntimeData().selectedPower);
	}

	// add check if we need to block left
	if (!RE::UI::GetSingleton()->GameIsPaused() && equippable::is_two_handed(form))
	{
		processing::set_setting_data::check_if_location_needs_block(form, event->equipped);
	}

	return event_result::kContinue;
}

// Handle key press events. Do we need to act on the keypress in any way?

using event_result    = RE::BSEventNotifyControl;
using position_type   = enums::position_type;
using setting_execute = processing::setting_execute;

KeyEventSink* KeyEventSink::get_singleton()
{
	static KeyEventSink singleton;
	return std::addressof(singleton);
}

void KeyEventSink::register_sink()
{
	RE::BSInputDeviceManager::GetSingleton()->AddEventSink(get_singleton());
	logger::info("start listening for input events."sv);
}

event_result KeyEventSink::ProcessEvent(RE::InputEvent* const* event_list,
	[[maybe_unused]] RE::BSTEventSource<RE::InputEvent*>* source)
{
	// We start by figuring out if we need to ddo anything at all.
	if (!event_list)
	{
		return event_result::kContinue;
	}

	// If we can't ask questions about the state of the UI, we bail.
	auto* ui = RE::UI::GetSingleton();
	if (!ui)
	{
		return event_result::kContinue;
	}

	// We do nothing if the console, the inventory menu, the magic menu, or the favorites
	// menu are open.
	const auto* interface_strings = RE::InterfaceStrings::GetSingleton();
	if (ui->IsMenuOpen(interface_strings->console))
	{
		return event_result::kContinue;
	}

	if (ui->IsMenuOpen(RE::InventoryMenu::MENU_NAME) || ui->IsMenuOpen(RE::MagicMenu::MENU_NAME) ||
		ui->IsMenuOpen(RE::FavoritesMenu::MENU_NAME))
	{
		return event_result::kContinue;
	}

	// TODO remove this. clears extra data (what is extra data?)
	handle::extra_data_holder::get_singleton()->reset_data();

	// We might get a list of events to handle.
	for (auto* event = *event_list; event; event = event->next)
	{
		if (event->eventType != RE::INPUT_EVENT_TYPE::kButton)
		{
			continue;
		}

		/*if the game is not paused with the menu, it triggers the menu always in the background*/
		if (ui->GameIsPaused() || !ui->IsCursorHiddenWhenTopmost() || !ui->IsShowingMenus() ||
			!ui->GetMenu<RE::HUDMenu>())
		{
			continue;
		}

		// consider not acting when loot menu is open

		// If we're not in control of the player character or otherwise not in gameplay, move on.
		const auto* control_map = RE::ControlMap::GetSingleton();
		if (!control_map || !control_map->IsMovementControlsEnabled() ||
			control_map->contextPriorityStack.back() != RE::UserEvents::INPUT_CONTEXT_ID::kGameplay)
		{
			continue;
		}

		// this stays static_cast
		const auto* button =
			static_cast<RE::ButtonEvent*>(event);  // NOLINT(cppcoreguidelines-pro-type-static-cast-downcast)

		// This offsets the button by an amount that varies based on what originated the
		// event. This appears to be so that we can directly compare it to the hotkey numbers
		// we have snagged from the MCM settings.
		const uint32_t key = keycodes::get_key_id(button);
		if (key == -1)
		{
			continue;
		}

		const rust::Box<UserSettings> settings = user_settings();  // rust
		const bool is_cycle_button = settings->is_cycle_button(key);

		// we hand off to rust to act.
		const KeyEventResponse response = handle_key_event(key, *button);
		if (!response.handled)
		{
			continue;
		}

		if (response.stop_timer != Action::Irrelevant)
		{
			// TODO stop the matching timer if we have one
		}

		if (response.start_timer != Action::Irrelevant)
		{
			// TODO start a matching timer and record it somewhere
		}

		/*
this needs to be in its own function so Rust can call it with CycleEntry data when timers fire
		if (button->IsUp() && is_position_button)
		{
			logger::debug("configured Key ({}) is up"sv, key);
			// set slot back to normal color
			//  Look up the current thing-we-would-do for this keypress, then do it.
			//  E.g., equip the next item in the cycle.
			auto* position_setting = setting_execute::get_position_setting_for_key(key);
			if (!position_setting)
			{
				logger::warn("setting for key {} is null. break."sv, key);
				continue;
			}
			position_setting->button_press_modify = ui::draw_full;
			if (position_setting->position == position_type::left)
			{
				if (auto* current_ammo = handle::ammo_handle::get_singleton()->get_current())
				{
					current_ammo->button_press_modify = ui::draw_full;
				}
			}
		}
		*/
	}  // end event handling for loop

	return event_result::kContinue;
}

/*
// TODO turn this function into the timer-fired handler. It has most of the required logic in it already.
void handleCycleSlotKey(uint32_t a_key, control::binding*& a_binding)
{
	logger::debug("configured Key ({}) pressed"sv, a_key);
	auto* position_setting = setting_execute::get_position_setting_for_key(a_key);

	const auto* key_handler  = handle::key_position_handle::get_singleton();
	const auto* page_handler = handle::page_handle::get_singleton();

	// Is this position locked? If so, we just check our ammo and exit.
	if (key_handler->is_position_locked(position_setting->position))
	{
		logger::trace("position {} is locked, skip"sv, static_cast<uint32_t>(position_setting->position));
		// check ammo is set, might be a bow or crossbow present
		const auto* ammo_handle = handle::ammo_handle::get_singleton();
		if (const auto next_ammo = ammo_handle->get_next_ammo())
		{
			setting_execute::execute_ammo(next_ammo);
			handle::ammo_handle::get_singleton()->get_current()->highlight_slot = true;
		}
		return;
	}

	// Advance our cycle one step. Why aren't we returning the settings from this call?
	page_handler->set_active_page_position(
		page_handler->get_next_non_empty_setting_for_position(position_setting->position),
		position_setting->position);

	// Get the new position setting. If we get nothing here, we are in a state where
	// we can't do anything useful.
	auto* new_position = setting_execute::get_position_setting_for_key(a_key);

	if (!new_position)
	{
		logger::warn("setting for key {} is null. break."sv, a_key);
		return;
	}

	new_position->highlight_slot = true;
	if (!scroll_position(a_key, a_binding))
	{
		setting_execute::activate(new_position->slot_settings);
	}
	else if (new_position->position == position_type::top)
	{
		setting_execute::activate(new_position->slot_settings, true);
	}
}
*/

/*
bool KeyEventSink::scroll_position(const uint32_t a_key, control::binding*& a_binding)
{
	if (a_key == a_binding->get_bottom_action() || a_key == a_binding->get_top_action())
	{
		return true;
	}
	return false;
}

void KeyEventSink::do_button_down(handle::position_setting*& a_position_setting) const
{
	if (!a_position_setting)
	{
		return;
	}
	if (!handle::key_position_handle::get_singleton()->is_position_locked(a_position_setting->position))
	{
		a_position_setting->button_press_modify = button_press_modify_;
	}
	else
	{
		if (a_position_setting->position == position_type::left)
		{
			if (auto* current_ammo = handle::ammo_handle::get_singleton()->get_current())
			{
				current_ammo->button_press_modify = button_press_modify_;
			}
		}
	}
}
*/

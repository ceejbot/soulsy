#include "sinks.h"

#include "equippable.h"
#include "helpers.h"
#include "keycodes.h"
#include "player.h"
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

	auto item = equippable::cycle_entry_from_form(form);
	auto changed = handle_item_equipped(std::move(item));
	logger::info("handled inventory change; changed={}; item='{}';"sv, changed, form->GetName());
	// TODO trigger UI redraw? or just wait for next tick?

	return event_result::kContinue;
}

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
		// we have snagged from the MCM settings. ??
		const uint32_t key = keycodes::get_key_id(button);
		logger::info("handling button event; after offset key={}"sv, key);
		if (key == -1)
		{
			continue;
		}

		const KeyEventResponse response = handle_key_event(key, *button);
		logger::info("controller responded to button event; key={}; handled={}; start={}; stop={}"sv,
			key,
			response.handled,
			static_cast<uint8_t>(response.start_timer),
			static_cast<uint8_t>(response.stop_timer));
		if (!response.handled)
		{
			continue;
		}

		if (response.stop_timer != Action::Irrelevant)
		{
			logger::info("hysteresis timer STOP; slot={}"sv, static_cast<uint8_t>(response.stop_timer));
			ui::ui_renderer::stopTimer(response.stop_timer);
		}

		if (response.start_timer != Action::Irrelevant)
		{
			logger::info("hysteresis timer START; slot={}"sv, static_cast<uint8_t>(response.start_timer));
			ui::ui_renderer::startTimer(response.start_timer);
		}

	}  // end event handling for loop

	return event_result::kContinue;
}

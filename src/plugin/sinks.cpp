#include "sinks.h"

#include "equippable.h"
#include "helpers.h"
#include "keycodes.h"
#include "player.h"
#include "ui_renderer.h"

#include "lib.rs.h"

// Handle equipment change events. We need to update our UI when this happens.

using event_result = RE::BSEventNotifyControl;

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
	if (!event || !event->actor || !event->actor->IsPlayerRef()) { return event_result::kContinue; }

	auto* form = RE::TESForm::LookupByID(event->baseObject);
	if (!form) { return event_result::kContinue; }

	auto item = equippable::makeItemDataFromForm(form);
	handle_item_equipped(event->equipped, std::move(item));

	return event_result::kContinue;
}

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
	// We start by figuring out if we need to do anything at all.
	if (!event_list) { return event_result::kContinue; }

	if (helpers::ignoreKeyEvents()) { return event_result::kContinue; }

	// We might get a list of events to handle.
	for (auto* event = *event_list; event; event = event->next)
	{
		if (event->eventType != RE::INPUT_EVENT_TYPE::kButton) { continue; }

		const auto* button =
			static_cast<RE::ButtonEvent*>(event);  // NOLINT(cppcoreguidelines-pro-type-static-cast-downcast)

		// This offsets the button by an amount that varies based on what originated the
		// event. This appears to be so that we can directly compare it to the hotkey numbers
		// we have snagged from the MCM settings. ??
		const uint32_t key = keycodes::get_key_id(button);
		if (key == -1) { continue; }

		// We need to be a little bit stateful to handle modifier keys, because we don't
		// get chording events, so all the logic is now in the controller.
		const KeyEventResponse response = handle_key_event(key, *button);
		if (!response.handled) { continue; }

		if (response.stop_timer != Action::None)
		{
			logger::debug("hysteresis timer STOP; slot={}"sv, static_cast<uint8_t>(response.stop_timer));
			ui::ui_renderer::stopTimer(response.stop_timer);
		}

		if (response.start_timer != Action::None)
		{
			logger::debug("hysteresis timer START; slot={}"sv, static_cast<uint8_t>(response.start_timer));
			ui::ui_renderer::startTimer(response.start_timer);
		}

	}  // end event handling for loop

	return event_result::kContinue;
}

#include "sinks.h"

#include "equippable.h"
#include "gear.h"
#include "helpers.h"
#include "inventory.h"
#include "keycodes.h"
#include "player.h"
#include "ui_renderer.h"

#include "lib.rs.h"

using event_result = RE::BSEventNotifyControl;

void registerAllListeners()
{
	rlog::info("Registering listeners for game events.");
	EquipEventListener::registerListener();
	KeyEventListener::registerListener();
	AnimGraphListener::registerListener();
	// MagicEffectListener::registerListener();
}

EquipEventListener* EquipEventListener::get_singleton()
{
	static EquipEventListener singleton;
	return std::addressof(singleton);
}

void EquipEventListener::registerListener()
{
	RE::ScriptEventSourceHolder::GetSingleton()->AddEventSink(get_singleton());
	rlog::info("    Listening for equipment change events.");
}

// Handle equipment change events. We need to update our UI when this happens.
EquipEventListener::event_result EquipEventListener::ProcessEvent(const RE::TESEquipEvent* event,
	[[maybe_unused]] RE::BSTEventSource<RE::TESEquipEvent>* source)
{
	if (!event || !event->actor || !event->actor->IsPlayerRef()) { return event_result::kContinue; }
	auto* form = RE::TESForm::LookupByID(event->baseObject);
	if (!form) { return event_result::kContinue; }

	auto* player   = RE::PlayerCharacter::GetSingleton();
	auto* left_eq  = player->GetActorRuntimeData().currentProcess->GetEquippedLeftHand();
	auto* right_eq = player->GetActorRuntimeData().currentProcess->GetEquippedRightHand();

	if (form->IsAmmo() && !event->equipped)
	{
		// double-check that we really unequipped it and it's not just a count change.
		auto* current_ammo = player->GetCurrentAmmo();
		if (current_ammo && current_ammo->GetFormID() == form->GetFormID()) { return event_result::kContinue; }
	}

	const auto formtype = form->GetFormType();
	const auto name     = helpers::displayNameAsUtf8(form);
	if (event->equipped) { rlog::debug("equip event: {} '{}' equipped", RE::FormTypeToString(formtype), name); }
	else { rlog::debug("equip event: {} '{}' removed", RE::FormTypeToString(formtype), name); }

	std::string worn_right = helpers::makeFormSpecString(right_eq);
	std::string worn_left  = helpers::makeFormSpecString(left_eq);
	std::string form_spec  = helpers::makeFormSpecString(form);
	handle_item_equipped(event->equipped, form_spec, worn_right, worn_left);

	return event_result::kContinue;
}

KeyEventListener* KeyEventListener::get_singleton()
{
	static KeyEventListener singleton;
	return std::addressof(singleton);
}

void KeyEventListener::registerListener()
{
	RE::BSInputDeviceManager::GetSingleton()->AddEventSink(get_singleton());
	rlog::info("    Listening for player input events."sv);
}

event_result KeyEventListener::ProcessEvent(RE::InputEvent* const* event_list,
	[[maybe_unused]] RE::BSTEventSource<RE::InputEvent*>* source)
{
	// We start by figuring out if we need to do anything at all.
	if (!event_list) { return event_result::kContinue; }

	if (helpers::ignoreKeyEvents()) { return event_result::kContinue; }

	// We might get a list of events to handle.
	for (auto* event = *event_list; event; event = event->next)
	{
		if (event->eventType != RE::INPUT_EVENT_TYPE::kButton) { continue; }

		auto* button = static_cast<RE::ButtonEvent*>(event);  // NOLINT(cppcoreguidelines-pro-type-static-cast-downcast)

		// This offsets the button by an amount that varies based on what originated the
		// event. This appears to be so that we can directly compare it to the hotkey numbers
		// we have snagged from the MCM settings. ??
		const uint32_t key = keycodes::keyID(button);
		if (key == -1) { continue; }

		// We need to be a little bit stateful to handle modifier keys, because we don't
		// get chording events, so all the logic is now in the controller.
		const KeyEventResponse response = handle_key_event(key, *button);
		if (!response.handled) { continue; }
		//rlog::info("mod handled key: {}", key);

		if (response.stop_timer != Action::None)
		{
			// rlog::trace("hysteresis timer STOP; slot={}"sv, static_cast<uint8_t>(response.stop_timer));
			ui::stopTimer(response.stop_timer);
		}

		if (response.start_timer != Action::None)
		{
			// rlog::trace("hysteresis timer START; slot={}"sv, static_cast<uint8_t>(response.start_timer));
			auto settings = user_settings();
			auto duration = settings->equip_delay_ms();
			ui::startTimer(response.start_timer, duration);
		}

		// Now wipe out the event data so nothing else acts on it.
		// Is there a way to respond with                                                                                                                                                                                     `kStop` for just one event in the list?
		button->idCode    = keycodes::kInvalid;
		button->userEvent = "";
	}  // end event handling for loop

	return event_result::kContinue;
}


// ---------- animation graph events
// Here we watch for anim graph events ONLY to catch CGO's grip switch variable change.

AnimGraphListener* AnimGraphListener::get_singleton()
{
	static AnimGraphListener singleton;
	return std::addressof(singleton);
}

void AnimGraphListener::registerListener()
{
	auto* player = RE::PlayerCharacter::GetSingleton();
	auto okay    = player->AddAnimationGraphEventSink(AnimGraphListener::get_singleton());
	if (okay) { rlog::info("    Listening for animation graph events to get grip changes."); }
	// else { rlog::warn("Surprising: failed to add an event listener for animation graph events."); }
}

RE::BSEventNotifyControl AnimGraphListener::ProcessEvent(const RE::BSAnimationGraphEvent* event,
	[[maybe_unused]] RE::BSTEventSource<RE::BSAnimationGraphEvent>* source)
{
	if (event->tag == "GripChangeEvent")
	{
		bool useAltGrip = false;
		RE::PlayerCharacter::GetSingleton()->GetGraphVariableBool("bUseAltGrip", useAltGrip);
		handle_grip_change(useAltGrip);
	}

	return event_result::kContinue;
}

// ----------- MagicEffectListener
// This only gets notifications of magic effects arriving.

MagicEffectListener* MagicEffectListener::get_singleton()
{
	static MagicEffectListener singleton;
	return std::addressof(singleton);
}

void MagicEffectListener::registerListener()
{
	auto* scriptEvents = RE::ScriptEventSourceHolder::GetSingleton();
	scriptEvents->AddEventSink(MagicEffectListener::get_singleton());
	rlog::info("    Listening for magic effect events."sv);
}

// TODO I also need a listener for TESActiveEffectApplyRemoveEvent

RE::BSEventNotifyControl MagicEffectListener::ProcessEvent(const RE::TESMagicEffectApplyEvent* event,
	[[maybe_unused]] RE::BSTEventSource<RE::TESMagicEffectApplyEvent>* source)
{
	// TODO
	auto caster     = event->caster->GetBaseObject();
	auto casterName = helpers::displayNameAsUtf8(caster);

	auto* magicEffect = RE::TESForm::LookupByID(event->magicEffect);
	auto effectName   = helpers::displayNameAsUtf8(magicEffect);

	auto target     = event->target->GetBaseObject();
	auto targetName = helpers::displayNameAsUtf8(target);

	rlog::info("Effect status change: '{}' put \"{}\" ({}) on '{}'",
		casterName.length() > 0 ? casterName : rlog::formatAsHex(event->caster->GetFormID()),
		effectName,
		rlog::formatAsHex(event->magicEffect),
		targetName.length() > 0 ? targetName : rlog::formatAsHex(event->target->GetFormID()));

	return event_result::kContinue;
}

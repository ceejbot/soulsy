#include "sinks.h"

#include "RE/P/PlayerCharacter.h"
#include "equippable.h"
#include "gear.h"
#include "helpers.h"
#include "keycodes.h"
#include "player.h"
#include "ui_renderer.h"

#include "lib.rs.h"

void registerAllListeners()
{
	rlog::info("Registering listeners for game events:");
	auto listener                = TheListener::singleton();
	auto scriptEventSourceHolder = RE::ScriptEventSourceHolder::GetSingleton();

	scriptEventSourceHolder->GetEventSource<RE::TESEquipEvent>()->AddEventSink(listener);
	rlog::info("    equipment change events: {}", typeid(RE::TESEquipEvent).name());

	// scriptEventSourceHolder->GetEventSource<RE::TESHitEvent>()->AddEventSink(listener);
	// rlog::info("    hit events: {}"sv, typeid(RE::TESHitEvent).name());

	// RE::UI::GetSingleton()->AddEventSink<RE::MenuOpenCloseEvent>(listener);
	//rlog::info("    menu open/close events: {}"sv, typeid(RE::MenuOpenCloseEvent).name());

	RE::BSInputDeviceManager::GetSingleton()->AddEventSink(listener);
	rlog::info("    player input events."sv);

	auto* player = RE::PlayerCharacter::GetSingleton();
	auto okay    = player->AddAnimationGraphEventSink(listener);
	if (okay) { rlog::info("    animation graph events to get grip changes."); }

	//scriptEventSourceHolder->GetEventSource<RE::TESMagicEffectApplyEvent>()->AddEventSink(listener);
	//scriptEventSourceHolder->GetEventSource<RE::TESActiveEffectApplyRemoveEvent>()->AddEventSink(listener);
	//rlog::info("    magic effects come and go, talking of Michelangelo."sv);
}

TheListener* TheListener::singleton()
{
	static TheListener singleton;
	return std::addressof(singleton);
}

RE::BSEventNotifyControl TheListener::ProcessEvent(const RE::TESEquipEvent* event,
	[[maybe_unused]] RE::BSTEventSource<RE::TESEquipEvent>* source)
{
	if (!event || !event->actor || !event->actor->IsPlayerRef()) { return RE::BSEventNotifyControl::kContinue; }
	auto* form = RE::TESForm::LookupByID(event->baseObject);
	if (!form) { return RE::BSEventNotifyControl::kContinue; }

	auto* player   = RE::PlayerCharacter::GetSingleton();
	auto* left_eq  = player->GetActorRuntimeData().currentProcess->GetEquippedLeftHand();
	auto* right_eq = player->GetActorRuntimeData().currentProcess->GetEquippedRightHand();

	if (form->IsAmmo() && !event->equipped)
	{
		// double-check that we really unequipped it and it's not just a count change.
		auto* current_ammo = player->GetCurrentAmmo();
		if (current_ammo && current_ammo->GetFormID() == form->GetFormID())
		{
			return RE::BSEventNotifyControl::kContinue;
		}
	}

	const auto formtype = form->GetFormType();
	const auto name     = helpers::displayNameAsUtf8(form);
	if (event->equipped) { rlog::debug("equip event: {} '{}' equipped", RE::FormTypeToString(formtype), name); }
	else { rlog::debug("equip event: {} '{}' removed", RE::FormTypeToString(formtype), name); }

	if (formtype == RE::FormType::Enchantment) { return RE::BSEventNotifyControl::kContinue; }

	std::string worn_right = helpers::makeFormSpecString(right_eq);
	std::string worn_left  = helpers::makeFormSpecString(left_eq);
	std::string form_spec  = helpers::makeFormSpecString(form);
	handle_item_equipped(event->equipped, form_spec, worn_right, worn_left);

	return RE::BSEventNotifyControl::kContinue;
}

RE::BSEventNotifyControl TheListener::ProcessEvent(const RE::TESHitEvent* event,
	[[maybe_unused]] RE::BSTEventSource<RE::TESHitEvent>* source)
{
	// TODO; just logging for now
	auto* sourceForm = RE::TESForm::LookupByID(event->source);
	auto sourceName  = helpers::displayNameAsUtf8(sourceForm);

	auto target     = event->target->GetBaseObject();
	auto targetName = helpers::displayNameAsUtf8(target);

	rlog::info("hit event: '{}' 🗡️ {}",
		sourceName.length() > 0 ? sourceName : rlog::formatAsHex(event->source),
		targetName.length() > 0 ? targetName : rlog::formatAsHex(event->target->GetFormID()));

	return RE::BSEventNotifyControl::kContinue;
}

RE::BSEventNotifyControl TheListener::ProcessEvent(const RE::MenuOpenCloseEvent* event,
	[[maybe_unused]] RE::BSTEventSource<RE::MenuOpenCloseEvent>* source)
{
	// TODO; just logging for now
	rlog::info("menu event: '{}' {}", event->menuName, event->opening ? "opened" : "closed");
	return RE::BSEventNotifyControl::kContinue;
}

RE::BSEventNotifyControl TheListener::ProcessEvent(RE::InputEvent* const* event_list,
	[[maybe_unused]] RE::BSTEventSource<RE::InputEvent*>* source)
{
	// We start by figuring out if we need to do anything at all.
	if (!event_list) { return RE::BSEventNotifyControl::kContinue; }

	if (helpers::ignoreKeyEvents()) { return RE::BSEventNotifyControl::kContinue; }

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
		// Is there a way to respond with `kStop` for just one event in the list?
		button->idCode    = keycodes::kInvalid;
		button->userEvent = "";
	}  // end event handling for loop

	return RE::BSEventNotifyControl::kContinue;
}

// Here we watch for anim graph events ONLY to catch CGO's grip switch variable change.
RE::BSEventNotifyControl TheListener::ProcessEvent(const RE::BSAnimationGraphEvent* event,
	[[maybe_unused]] RE::BSTEventSource<RE::BSAnimationGraphEvent>* source)
{
	if (event->tag == "GripChangeEvent")
	{
		bool useAltGrip = false;
		RE::PlayerCharacter::GetSingleton()->GetGraphVariableBool("bUseAltGrip", useAltGrip);
		handle_grip_change(useAltGrip);
	}

	return RE::BSEventNotifyControl::kContinue;
}

// ----------- MagicEffectListener
// This only gets notifications of magic effects arriving. ?
RE::BSEventNotifyControl TheListener::ProcessEvent(const RE::TESMagicEffectApplyEvent* event,
	[[maybe_unused]] RE::BSTEventSource<RE::TESMagicEffectApplyEvent>* source)
{
	// TODO; just logging for now
	std::string casterName = std::string("<none>");
	if (event->caster)
	{
		auto caster = event->caster->GetBaseObject();
		casterName  = helpers::displayNameAsUtf8(caster);
	}

	auto* magicEffect = RE::TESForm::LookupByID(event->magicEffect);
	auto effectName   = helpers::displayNameAsUtf8(magicEffect);
	if (effectName == "XPMSE Weapon Apply Effect") { return RE::BSEventNotifyControl::kContinue; }

	std::string targetName = std::string("<none>");
	if (event->caster)
	{
		auto target = event->target->GetBaseObject();
		targetName  = helpers::displayNameAsUtf8(target);
	}

	if (event->caster && event->caster->GetFormID() != 0x00000014 && event->target &&
		event->target->GetFormID() != 0x00000014)
	{
		return RE::BSEventNotifyControl::kContinue;
	}

	rlog::info("Effect status change: '{}' {} put \"{}\" ({}) on '{}' {}",
		casterName,
		event->caster ? rlog::formatAsHex(event->caster->GetFormID()) : "<none>",
		effectName,
		rlog::formatAsHex(event->magicEffect),
		targetName,
		event->target ? rlog::formatAsHex(event->target->GetFormID()) : "<none>");

	return RE::BSEventNotifyControl::kContinue;
}

RE::BSEventNotifyControl TheListener::ProcessEvent(const RE::TESActiveEffectApplyRemoveEvent* event,
	[[maybe_unused]] RE::BSTEventSource<RE::TESActiveEffectApplyRemoveEvent>* source)
{
	// TODO; just logging for now
	auto caster     = event->caster ? event->caster->GetBaseObject() : nullptr;
	auto casterName = caster ? helpers::displayNameAsUtf8(caster) : "<unknown>";

	auto target     = event->target ? event->target->GetBaseObject() : nullptr;
	auto targetName = target ? helpers::displayNameAsUtf8(target) : "<unknown>";

	rlog::info(
		"effect unique id={:#04x}; verb: {}", event->activeEffectUniqueID, event->isApplied ? "applied" : "removed");

	if (!caster || !target) { return RE::BSEventNotifyControl::kContinue; }

	const auto playerID = RE::PlayerCharacter::GetSingleton()->GetFormID();
	if (caster->GetFormID() != playerID && target->GetFormID() != playerID)
	{
		return RE::BSEventNotifyControl::kContinue;
	}

	rlog::info("Effect status change: '{}' -> {} effect id {} -> '{}'",
		casterName.length() > 0 ? casterName : rlog::formatAsHex(event->caster->GetFormID()),
		event->isApplied ? "applied" : "removed",
		rlog::formatAsHex(event->activeEffectUniqueID),
		targetName.length() > 0 ? targetName : rlog::formatAsHex(event->target->GetFormID()));

	return RE::BSEventNotifyControl::kContinue;
}

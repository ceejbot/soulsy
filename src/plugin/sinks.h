#pragma once

// This file relies on the force-injected precompiled header.
// It contains all of our event sinks. We register all of these
// with CommonLibSSE's events and do initial processing in the callbacks.
// All heavy application-level logic happens on the Rust side.

void registerAllListeners();


class TheListener final
	: public RE::BSTEventSink<RE::BSAnimationGraphEvent*>
	, public RE::BSTEventSink<RE::InputEvent*>
	, public RE::BSTEventSink<RE::MenuOpenCloseEvent*>
	, public RE::BSTEventSink<RE::TESEquipEvent>
	, public RE::BSTEventSink<RE::TESHitEvent>
	, public RE::BSTEventSink<RE::TESMagicEffectApplyEvent*>
	, public RE::BSTEventSink<RE::TESActiveEffectApplyRemoveEvent*>
{
	using event_result = RE::BSEventNotifyControl;

public:
	static TheListener* singleton(void);

	// It's a programmer error to have more than one.
	TheListener(const TheListener&) = delete;
	TheListener(TheListener&&)      = delete;

	TheListener& operator=(const TheListener&) = delete;
	TheListener& operator=(TheListener&&)      = delete;

protected:
	RE::BSEventNotifyControl ProcessEvent(const RE::TESEquipEvent* event,
		[[maybe_unused]] RE::BSTEventSource<RE::TESEquipEvent>* source) override;
	RE::BSEventNotifyControl ProcessEvent(RE::InputEvent* const* a_event,
		[[maybe_unused]] RE::BSTEventSource<RE::InputEvent*>* a_event_source) override;
	RE::BSEventNotifyControl ProcessEvent(const RE::BSAnimationGraphEvent* event,
		RE::BSTEventSource<RE::BSAnimationGraphEvent>* source) override;
	RE::BSEventNotifyControl ProcessEvent(const RE::TESMagicEffectApplyEvent* event,
		RE::BSTEventSource<RE::TESMagicEffectApplyEvent>* source) override;

private:
	TheListener()           = default;
	~TheListener() override = default;
};

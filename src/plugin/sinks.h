#pragma once

// This file relies on the force-injected precompiled header.
// It contains all of our event sinks. We register all of these
// with CommonLibSSE's events and do initial processing in the callbacks.
// All heavy application-level logic happens on the Rust side.

void registerAllListeners();


class EquipEventListener final : public RE::BSTEventSink<RE::TESEquipEvent>
{
	using event_result = RE::BSEventNotifyControl;

public:
	static EquipEventListener* get_singleton(void);
	static void registerListener(void);

	// It's a programmer error to have more than one.
	EquipEventListener(const EquipEventListener&) = delete;
	EquipEventListener(EquipEventListener&&)      = delete;

	EquipEventListener& operator=(const EquipEventListener&) = delete;
	EquipEventListener& operator=(EquipEventListener&&)      = delete;

protected:
	RE::BSEventNotifyControl ProcessEvent(const RE::TESEquipEvent* event,
		[[maybe_unused]] RE::BSTEventSource<RE::TESEquipEvent>* source) override;

private:
	EquipEventListener()           = default;
	~EquipEventListener() override = default;
};

class KeyEventListener final : public RE::BSTEventSink<RE::InputEvent*>
{
	using event_result = RE::BSEventNotifyControl;

public:
	static KeyEventListener* get_singleton();
	static void registerListener();

	KeyEventListener(const KeyEventListener&) = delete;
	KeyEventListener(KeyEventListener&&)      = delete;

	KeyEventListener& operator=(const KeyEventListener&) = delete;
	KeyEventListener& operator=(KeyEventListener&&)      = delete;

protected:
	RE::BSEventNotifyControl ProcessEvent(RE::InputEvent* const* a_event,
		[[maybe_unused]] RE::BSTEventSource<RE::InputEvent*>* a_event_source) override;

private:
	KeyEventListener()           = default;
	~KeyEventListener() override = default;
};

class AnimGraphListener final : public RE::BSTEventSink<RE::BSAnimationGraphEvent>
{
	using event_result = RE::BSEventNotifyControl;

public:
	static AnimGraphListener* get_singleton();
	static void registerListener();

	AnimGraphListener(const AnimGraphListener&) = delete;
	AnimGraphListener(AnimGraphListener&&)      = delete;

	AnimGraphListener& operator=(const AnimGraphListener&) = delete;
	AnimGraphListener& operator=(AnimGraphListener&&)      = delete;

protected:
	RE::BSEventNotifyControl ProcessEvent(const RE::BSAnimationGraphEvent* event,
		RE::BSTEventSource<RE::BSAnimationGraphEvent>* source) override;

private:
	AnimGraphListener()           = default;
	~AnimGraphListener() override = default;
};

class ControlStateListener final : public RE::BSTEventSink<RE::UserEventEnabled>
{
public:
	static ControlStateListener* get_singleton();
	static void registerListener();

	ControlStateListener(const ControlStateListener&) = delete;
	ControlStateListener(ControlStateListener&&)      = delete;

	ControlStateListener& operator=(const ControlStateListener&) = delete;
	ControlStateListener& operator=(ControlStateListener&&)      = delete;

	static std::string controlStateDisplay(const SKSE::stl::enumeration<RE::ControlMap::UEFlag, uint32_t> state);

protected:
	RE::BSEventNotifyControl ProcessEvent(const RE::UserEventEnabled* event,
		RE::BSTEventSource<RE::UserEventEnabled>* source) override;

private:
	ControlStateListener()           = default;
	~ControlStateListener() override = default;
};

class MagicEffectListener final : public RE::BSTEventSink<RE::TESMagicEffectApplyEvent>
{
public:
	static MagicEffectListener* get_singleton();
	static void registerListener();

	MagicEffectListener(const MagicEffectListener&) = delete;
	MagicEffectListener(MagicEffectListener&&)      = delete;

	MagicEffectListener& operator=(const MagicEffectListener&) = delete;
	MagicEffectListener& operator=(MagicEffectListener&&)      = delete;

protected:
	RE::BSEventNotifyControl ProcessEvent(const RE::TESMagicEffectApplyEvent* event,
		RE::BSTEventSource<RE::TESMagicEffectApplyEvent>* source) override;

private:
	MagicEffectListener()           = default;
	~MagicEffectListener() override = default;
};

#pragma once

// This file relies on the force-injected precompiled header.
// It contains all of our event sinks. We register all of these
// with CommonLibSSE's events and do initial processing in the callbacks.
// All heavy application-level logic happens on the Rust side.

void register_all_sinks();

class EquipEventSink final : public RE::BSTEventSink<RE::TESEquipEvent>
{
public:
    using event_result = RE::BSEventNotifyControl;

    static EquipEventSink *get_singleton();
    static void register_sink();

    // It's a programmer error to have more than one.
    EquipEventSink(const EquipEventSink &) = delete;
    EquipEventSink(EquipEventSink &&) = delete;

    EquipEventSink &operator=(const EquipEventSink &) = delete;
    EquipEventSink &operator=(EquipEventSink &&) = delete;

protected:
    event_result ProcessEvent(const RE::TESEquipEvent *event,
                              [[maybe_unused]] RE::BSTEventSource<RE::TESEquipEvent> *source) override;

private:
    EquipEventSink() = default;
    ~EquipEventSink() override = default;
};

class KeyEventSink final : public RE::BSTEventSink<RE::InputEvent *>
{
    using event_result = RE::BSEventNotifyControl;

public:
    static KeyEventSink *get_singleton();
    static void sink();

    KeyEventSink(const KeyEventSink &) = delete;
    KeyEventSink(KeyEventSink &&) = delete;

    KeyEventSink &operator=(const KeyEventSink &) = delete;
    KeyEventSink &operator=(KeyEventSink &&) = delete;

protected:
    event_result ProcessEvent(RE::InputEvent *const *a_event,
                              [[maybe_unused]] RE::BSTEventSource<RE::InputEvent *> *a_event_source) override;

private:
    KeyEventSink() = default;
    ~KeyEventSink() override = default;

    // I think this means highlight relevant slot on button down.
    uint32_t button_press_modify_ = ui::draw_full;

    void handleCycleSlotKey(uint32_t a_key, control::binding *&a_binding) const;
    [[nodiscard]] static bool scroll_position(uint32_t a_key, control::binding *&a_binding);
    void do_button_down(handle::position_setting *&a_position_setting) const;
};

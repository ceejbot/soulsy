#pragma once
#include "./data/page/slot_setting.h"

class data_helper {
public:
    RE::TESForm* form = nullptr;
    handle::slot_setting::slot_type type = handle::slot_setting::slot_type::empty;
    handle::slot_setting::action_type action_type = handle::slot_setting::action_type::default_action;
    bool left = false;
    bool two_handed = false;
    RE::ActorValue actor_value = RE::ActorValue::kNone;
};

#pragma once

namespace handle {
    class slot_setting {
    public:
        //un equip just makes sense with form == nullptr
        enum class action_type : std::uint32_t { default_action = 0, instant = 1, un_equip = 2 };

        enum class hand_equip : std::uint32_t { single = 0, both = 1, total = 2 };

        enum class slot_type : std::uint32_t {
            weapon = 0,
            magic = 1,
            shield = 2,
            shout = 3,
            power = 4,
            consumable = 5,
            armor = 6,
            scroll = 7,
            empty = 8,
            misc = 9,
            light = 10,
            lantern = 11,
            mask = 12
        };


        RE::TESForm* form = nullptr;
        slot_type type = slot_type::empty;
        action_type action = action_type::default_action;
        hand_equip equip = hand_equip::total;
        RE::BGSEquipSlot* equip_slot = nullptr;
        int32_t item_count = 0;
        RE::ActorValue actor_value = RE::ActorValue::kNone;
        bool display_item_count = false;
    };
}

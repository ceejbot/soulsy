use std::collections::VecDeque;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::settings::*;

/*

class slot_setting {
public:
    //un equip just makes sense with form == nullptr
    enum class action_type : std::uint32_t { default_action = 0, instant = 1, un_equip = 2 };

    enum class hand_equip : std::uint32_t { single = 0, both = 1, total = 2 };


    RE::TESForm* form = nullptr;
    slot_type type = slot_type::empty;
    action_type action = action_type::default_action;
    hand_equip equip = hand_equip::total;
    RE::BGSEquipSlot* equip_slot = nullptr;
    int32_t item_count = 0;
    RE::ActorValue actor_value = RE::ActorValue::kNone;
    bool display_item_count = false;
};
 */

 #[derive(Debug, Clone)]
 enum ItemKind {
    Empty,
    Weapon,
    Magic,
    Shield,
    Shout,
    Power,
    Consumable,
    Armor,
    Scroll,
    Misc,
    Light,
    Lantern,
    Mask,
}

impl Default for ItemKind {
    fn default() -> Self {
        ItemKind::Empty
    }
}

// Haven't yet figured out how to serialize this to toml or anything yet.
// Still working on what data I want to track.
#[derive(Debug, Clone, Default)]
pub struct CycleEntry {
    spec: String,
    kind: ItemKind,
    // form: &TESForm,
    // slot: &BGSEquipSlot
}

#[derive(Debug, Clone, Default)]
pub struct CycleData {
    left: VecDeque<CycleEntry>,
    right: VecDeque<CycleEntry>,
    power: VecDeque<CycleEntry>,
    utility: VecDeque<CycleEntry>,
}

impl CycleData {
    pub fn toggle_power(&self, item: String) -> bool {
        false
    }

    pub fn toggle_left(&self, item: String) -> bool {
        false
    }

    pub fn toggle_right(&self, item: String) -> bool {
        false
    }

    pub fn toggle_utility(&self, item: String) -> bool {
        false
    }
}

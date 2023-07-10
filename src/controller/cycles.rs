use num_derive::{FromPrimitive, ToPrimitive};

use crate::controller::control::Action;
use crate::plugin::{KeyEventResponse, TESForm};

// TODO: This has moved to settings.
static MAX_CYCLE_LEN: usize = 10;

// This is in the same order as the slot_type enum on the C++ side.
// I haven't yet decided I like shared enums. This is a pain, tho.
#[derive(Debug, Clone, PartialEq, Eq, Default, FromPrimitive, ToPrimitive)]
pub enum ItemKind {
    Weapon,
    Magic,
    Shield,
    Shout,
    Power,
    Consumable,
    Armor,
    Scroll,
    #[default]
    Empty,
    Misc,
    Light,
    Lantern,
    Mask,
}

fn kind_from_slot(slot: u32) -> ItemKind {
    num_traits::FromPrimitive::from_u32(slot).unwrap_or_default()
}

fn left_hand_ok(kind: &ItemKind) -> bool {
    matches!(
        kind,
        ItemKind::Weapon
            | ItemKind::Magic
            | ItemKind::Shield
            | ItemKind::Scroll
            | ItemKind::Light
            | ItemKind::Lantern
    )
}

fn right_hand_ok(kind: &ItemKind) -> bool {
    matches!(kind, ItemKind::Weapon | ItemKind::Magic | ItemKind::Scroll)
}

fn is_power(kind: &ItemKind) -> bool {
    matches!(kind, ItemKind::Shout | ItemKind::Power)
}

fn is_utility(kind: &ItemKind) -> bool {
    matches!(
        kind,
        ItemKind::Consumable | ItemKind::Armor | ItemKind::Misc | ItemKind::Mask
    )
}

// Haven't yet figured out how to serialize this to toml or anything yet.
// Still working on what data I want to track.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CycleEntry {
    form_string: String,
    /// An enum classifying this item for fast question-answering. Equiv to `type` from TESForm.
    kind: ItemKind,
    /// True if this item requires both hands to use.
    two_handed: bool,
    /// True if this item should be displayed with count data.
    has_count: bool,
    icon: String,
    // form: &TESForm,
    // slot: &BGSEquipSlot
}

impl CycleEntry {
    /// This is called from C++ when handing us a new item.
    pub fn new(slot: u32, two_handed: bool, has_count: bool, _form: &TESForm) -> Self {
        // todo more
        CycleEntry {
            kind: kind_from_slot(slot),
            two_handed,
            has_count,
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct CycleData {
    left: Vec<CycleEntry>,
    right: Vec<CycleEntry>,
    power: Vec<CycleEntry>,
    utility: Vec<CycleEntry>,
}

impl CycleData {
    pub fn advance(&mut self, which: Action, amount: usize) -> Option<CycleEntry> {
        let cycle = match which {
            Action::Power => &mut self.power,
            Action::Left => &mut self.left,
            Action::Right => &mut self.right,
            Action::Utility => &mut self.utility,
            _ => {
                log::warn!("It is a programmer error to call advance() with {which:?}");
                return None;
            }
        };
        if cycle.is_empty() {
            return None;
        }
        cycle.rotate_left(amount);
        cycle.first().cloned()
    }

    pub fn toggle(&mut self, which: Action, item: CycleEntry) -> KeyEventResponse {
        let cycle = match which {
            Action::Power => {
                if !is_power(&item.kind) {
                    return KeyEventResponse::ItemInappropriate;
                }
                &mut self.power
            }
            Action::Left => {
                if !left_hand_ok(&item.kind) {
                    return KeyEventResponse::ItemInappropriate;
                }
                &mut self.left
            }
            Action::Right => {
                if !right_hand_ok(&item.kind) {
                    return KeyEventResponse::ItemInappropriate;
                }
                &mut self.right
            }
            Action::Utility => {
                if !is_utility(&item.kind) {
                    return KeyEventResponse::ItemInappropriate;
                }
                &mut self.utility
            }
            _ => {
                log::warn!("It is a programmer error to call toggle() with {which:?}");
                return KeyEventResponse::ItemInappropriate;
            }
        };

        // We have at most 10 items, so we can do this with a linear search.
        if let Some(idx) = cycle.iter().position(|xs| *xs == item) {
            cycle.remove(idx);
            KeyEventResponse::ItemRemoved
        } else if cycle.len() >= MAX_CYCLE_LEN {
            return KeyEventResponse::TooManyItems;
        } else {
            cycle.push(item);
            KeyEventResponse::ItemAdded
        }
    }
}

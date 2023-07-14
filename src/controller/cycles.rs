use std::path::PathBuf;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::user_settings;
use crate::plugin::{Action, EntryKind, MenuEventResponse};

// Functions exposed to C++.

/// Given an entry kind, return the filename of the icon to use for it.
pub fn get_icon_file(kind: &EntryKind) -> String {
    kind.icon_file()
}

// CycleEntry, which is also exposed to C++.
#[derive(Deserialize, Serialize, Debug, Clone, Default, Eq)]
pub struct CycleEntry {
    /// Player-visible name.
    name: String,
    /// A string that can be turned back into form data; for serializing.
    form_string: String,
    /// An enum classifying this item for fast question-answering as well as icon selection.
    kind: EntryKind,
    /// True if this item requires both hands to use.
    two_handed: bool,
    /// True if this item should be displayed with count data.
    has_count: bool,
    /// Cached count from inventory data. Relies on hooks to be updated.
    count: usize,
    /// is currently highlighted for some reason
    highlighted: bool,
}

// Testing the formstring is sufficient for our needs, which is figuring out if
// this form item is in a cycle already.
impl PartialEq for CycleEntry {
    fn eq(&self, other: &Self) -> bool {
        self.form_string == other.form_string
    }
}

pub fn create_cycle_entry(
    icon_kind: EntryKind,
    two_handed: bool,
    has_count: bool,
    count: usize,
    name: &str,
    form_string: &str,
) -> Box<CycleEntry> {
    Box::new(CycleEntry::new(
        icon_kind,
        two_handed,
        has_count,
        count,
        name,
        form_string,
    ))
}

pub fn default_cycle_entry() -> Box<CycleEntry> {
    Box::<CycleEntry>::default()
}

impl CycleEntry {
    /// This is called from C++ when handing us a new item.
    pub fn new(
        icon_kind: EntryKind,
        two_handed: bool,
        has_count: bool,
        count: usize,
        name: &str,
        form_string: &str,
    ) -> Self {
        CycleEntry {
            name: name.to_string(),
            form_string: form_string.to_string(),
            kind: icon_kind,
            two_handed,
            has_count,
            count,
            highlighted: false,
        }
    }

    /// Get the name of the item. Cloned string. Might be empty string.
    pub fn name(&self) -> String {
        self.name.clone()
    }

    /// Check if this item must be equipped with both hands.
    pub fn two_handed(&self) -> bool {
        self.two_handed
    }

    pub fn has_count(&self) -> bool {
        self.has_count
    }

    /// If this item has a count, e.g., is arrows, return how many the player has.
    pub fn count(&self) -> usize {
        self.count
    }

    // TODO remove this; we only ever want to change this via local control
    /// If the player's inventory changes, we update the item count.
    pub fn set_count(&mut self, v: usize) {
        self.count = v;
    }

    /// Get this item's form string, which encodes mod esp and formid.
    /// Should be stable across game loads.
    pub fn form_string(&self) -> String {
        self.form_string.clone()
    }

    /// Get the enum representing this entry's kind (1-handed sword, dagger, health potion, etc.)
    pub fn kind(&self) -> EntryKind {
        self.kind
    }

    /// True if this entry should be drawn with a highlight.
    pub fn highlighted(&self) -> bool {
        self.highlighted
    }
}

/// Manage the player's configured item cycles. Track changes, persist data in
/// files, and advance the cycle when the player presses a cycle button.
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct CycleData {
    left: Vec<CycleEntry>,
    right: Vec<CycleEntry>,
    power: Vec<CycleEntry>,
    utility: Vec<CycleEntry>,
}

// where to persist
static CYCLE_PATH: &str = "./data/SKSE/Plugins/SoulsyHUD_Cycles.toml";

impl CycleData {
    /// Write the cycle data to its file. This is *not yet* managed by character
    /// in any way, so it might be nonsense for one save vs another. It has the same
    /// but with rollbacks.
    pub fn write(&self) -> Result<()> {
        let buf = toml::to_string(self)?;
        std::fs::write(CYCLE_PATH, buf)?;
        Ok(())
    }

    /// Read cycle data from its cache file
    pub fn read() -> Result<Self> {
        let buf = std::fs::read_to_string(PathBuf::from(CYCLE_PATH))?;
        let layout = toml::from_str::<CycleData>(&buf)?;
        Ok(layout)
    }

    /// Advance the given cycle by one. Returns a copy of the newly-top item.
    ///
    /// Called when the player presses a hotkey bound to one of the cycle slots.
    /// This does not equip or try to use the item in any way. It's pure management.
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
        if let Some(previous) = cycle.first_mut() {
            previous.highlighted = false;
        }
        cycle.rotate_left(amount);
        cycle.first().cloned()
    }

    /// Attempt to set the current item in a cycle to the given form spec (mod.esp|formid).
    ///
    /// Responds with the entry for the item that ends up being the current for that
    /// cycle, and None if the cycle is empty. If the item is not found, we do not
    /// change the state of the cycle in any way.
    pub fn set_top(&mut self, _which: Action, _form_spec: String) -> Option<CycleEntry> {
        // TODO do I need this at all?
        todo!()
    }

    // the programmer error is annoying, but it's a shared struct...
    pub fn get_top(&self, which: Action) -> Option<CycleEntry> {
        let cycle = match which {
            Action::Power => &self.power,
            Action::Left => &self.left,
            Action::Right => &self.right,
            Action::Utility => &self.utility,
            _ => {
                log::warn!("It is a programmer error to call get_top() with {which:?}");
                return None;
            }
        };

        cycle.first().cloned()
    }

    /// Toggle the presence of the given item in the given cycle.
    ///
    /// Called from menu views when the player presses a hotkey matching a cycle.
    /// If the item is in the cycle, it's removed. If it's not present, it is added,
    /// providing the cycle has room. Returns an enum saying what it did, so calling
    /// layers can do whatever notification they find appropriate.
    ///
    /// Does not change the current item in the cycle, unless the current item is
    /// the one removed. Adds at the end.
    pub fn toggle(&mut self, which: Action, item: CycleEntry) -> MenuEventResponse {
        let cycle = match which {
            Action::Power => {
                if !item.kind.is_power() {
                    return MenuEventResponse::ItemInappropriate;
                }
                &mut self.power
            }
            Action::Left => {
                if !item.kind.left_hand_ok() {
                    return MenuEventResponse::ItemInappropriate;
                }
                &mut self.left
            }
            Action::Right => {
                if !item.kind.right_hand_ok() {
                    return MenuEventResponse::ItemInappropriate;
                }
                &mut self.right
            }
            Action::Utility => {
                if !item.kind.is_utility() {
                    return MenuEventResponse::ItemInappropriate;
                }
                &mut self.utility
            }
            _ => {
                log::warn!("It is a programmer error to call toggle() with {which:?}");
                return MenuEventResponse::ItemInappropriate;
            }
        };

        // We have at most 15 items, so we can do this with a linear search.
        let settings = user_settings();
        if let Some(idx) = cycle.iter().position(|xs| *xs == item) {
            cycle.remove(idx);
            MenuEventResponse::ItemRemoved
        } else if cycle.len() >= settings.maxlen() as usize {
            return MenuEventResponse::TooManyItems;
        } else {
            cycle.push(item);
            MenuEventResponse::ItemAdded
        }
    }
}

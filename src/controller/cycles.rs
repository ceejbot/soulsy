//! Management of the cycle data: serialization and mutation.
//!
use std::fmt::Display;
use std::path::PathBuf;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::control::MenuEventResponse;
use super::user_settings;
use crate::plugin::{hasItemOrSpell, notifyPlayer, playerName, Action, TesItemKind};

/// Given an entry kind, return the filename of the icon to use for it.
/// Exposed to C++.
pub fn get_icon_file(kind: &TesItemKind) -> String {
    kind.icon_file()
}

/// TesItemData, exposed to C++ as an opaque type.
#[derive(Deserialize, Serialize, Debug, Clone, Default, Eq)]
pub struct TesItemData {
    /// Player-visible name.
    name: String,
    /// A string that can be turned back into form data; for serializing.
    form_string: String,
    /// An enum classifying this item for fast question-answering as well as icon selection.
    kind: TesItemKind,
    /// True if this item requires both hands to use.
    two_handed: bool,
    /// True if this item should be displayed with count data.
    has_count: bool,
    /// Cached count from inventory data. Relies on hooks to be updated.
    count: u32,
    /// is currently highlighted for some reason
    highlighted: bool,
}

// Testing the formstring is sufficient for our needs, which is figuring out if
// this form item is in a cycle already.
impl PartialEq for TesItemData {
    fn eq(&self, other: &Self) -> bool {
        self.form_string == other.form_string
    }
}

/// Make a TesItemData struct from the given data.
pub fn make_tesitem(
    icon_kind: TesItemKind,
    two_handed: bool,
    has_count: bool,
    count: u32,
    name: &str,
    form_string: &str,
) -> Box<TesItemData> {
    Box::new(TesItemData::new(
        icon_kind,
        two_handed,
        has_count,
        count,
        name,
        form_string,
    ))
}

pub fn hand_to_hand_item() -> Box<TesItemData> {
    Box::new(TesItemData::new(
        TesItemKind::HandToHand,
        false,
        false,
        1,
        "Unarmed",
        "",
    ))
}

/// Construct a default TesItemData struct, which is displayed as
/// an empty spot on the HUD.
pub fn default_tes_item() -> Box<TesItemData> {
    Box::<TesItemData>::default()
}

impl TesItemData {
    /// This is called from C++ when handing us a new item.
    pub fn new(
        icon_kind: TesItemKind,
        two_handed: bool,
        has_count: bool,
        count: u32,
        name: &str,
        form_string: &str,
    ) -> Self {
        TesItemData {
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
    pub fn count(&self) -> u32 {
        self.count
    }

    /// Update the count following an inventory-count-changed event.
    pub fn set_count(&mut self, v: u32) {
        self.count = v;
    }

    /// Get this item's form string, which encodes mod esp and formid.
    /// Should be stable across game loads.
    pub fn form_string(&self) -> String {
        self.form_string.clone()
    }

    /// Get the enum representing this entry's kind (1-handed sword, dagger, health potion, etc.)
    pub fn kind(&self) -> TesItemKind {
        self.kind
    }

    /// True if this entry should be drawn with a highlight.
    pub fn highlighted(&self) -> bool {
        self.highlighted
    }
}

/// Manage the player's configured item cycles. Track changes, persist data in
/// files, and advance the cycle when the player presses a cycle button. This
/// struct now holds all data we need to persist across game starts.
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct CycleData {
    // I really want these to be maps, but toml can't serialize that.
    // Guess I could write to json instead.
    left: Vec<TesItemData>,
    right: Vec<TesItemData>,
    power: Vec<TesItemData>,
    utility: Vec<TesItemData>,
    #[serde(default)]
    hud_visible: bool,
}

// where to persist
static CYCLE_PATH: &str = "./data/SKSE/Plugins";

impl CycleData {
    /// Write the cycle data to its file.
    fn cycle_storage() -> PathBuf {
        let name = playerName()
            .trim()
            .replace(' ', "_")
            .replace(['\\', '\''], "");
        PathBuf::from(CYCLE_PATH).join(format!("SoulsyHUD_{}_Cycles.toml", name))
    }

    /// Write serialized toml to the cycle storage file for this character.
    pub fn write(&self) -> Result<()> {
        let buf = toml::to_string(self)?;
        std::fs::write(CycleData::cycle_storage(), buf)?;
        log::trace!(
            "wrote cycle data to {}",
            CycleData::cycle_storage().display()
        );
        Ok(())
    }

    /// Read cycle data from the serialization file for this character.
    pub fn read() -> Result<Self> {
        let buf = std::fs::read_to_string(CycleData::cycle_storage())?;
        log::debug!(
            "read cycle data from {}",
            CycleData::cycle_storage().display()
        );
        let data = toml::from_str::<CycleData>(&buf)?;
        Ok(data)
    }

    /// Advance the given cycle by one. Returns a copy of the newly-top item.
    ///
    /// Called when the player presses a hotkey bound to one of the cycle slots.
    /// This does not equip or try to use the item in any way. It's pure management.
    pub fn advance(&mut self, which: Action, amount: usize) -> Option<TesItemData> {
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

    pub fn advance_skipping(&mut self, which: Action, skip: TesItemData) -> Option<TesItemData> {
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
        cycle.rotate_left(1);
        let candidate = cycle
            .iter()
            .find(|xs| xs.form_string() != skip.form_string());
        if candidate.is_some() {
            log::info!("found {candidate:?}");
            let result = candidate.cloned();
            self.set_top(which, &result.as_ref().unwrap());
            result
        } else {
            log::info!("advance skip found nothing?????");
            None
        }
    }

    /// Get the length of the given cycle.
    pub fn cycle_len(&self, which: Action) -> usize {
        match which {
            Action::Power => self.power.len(),
            Action::Left => self.left.len(),
            Action::Right => self.right.len(),
            Action::Utility => self.utility.len(),
            _ => 0,
        }
    }

    /// Truncate a cycle to the passed-in length if necessary. Notifies on change.
    pub fn truncate_if_needed(&mut self, newlen: usize) {
        if self.power.len() > newlen {
            self.power.truncate(newlen);
            cxx::let_cxx_string!(msg = format!("Power cycle shortened to {} items.", newlen));
            notifyPlayer(&msg);
        }
        if self.utility.len() > newlen {
            self.utility.truncate(newlen);
            cxx::let_cxx_string!(msg = format!("Utility cycle shortened to {} items.", newlen));
            notifyPlayer(&msg);
        }
        if self.left.len() > newlen {
            self.left.truncate(newlen);
            cxx::let_cxx_string!(msg = format!("Left-hand cycle shortened to {} items.", newlen));
            notifyPlayer(&msg);
        }
        if self.right.len() > newlen {
            self.right.truncate(newlen);
            cxx::let_cxx_string!(msg = format!("Right-hand cycle shortened to {} items.", newlen));
            notifyPlayer(&msg);
        }
    }

    /// Remove any items that have vanished from the game or from the player's
    /// inventory.
    pub fn validate(&mut self) {
        // This is looking special-case-y. Find an abstraction maybe?
        self.power.retain(|xs| {
            cxx::let_cxx_string!(form_spec = xs.form_string());
            hasItemOrSpell(&form_spec)
        });
        self.utility.retain(|xs| {
            cxx::let_cxx_string!(form_spec = xs.form_string());
            hasItemOrSpell(&form_spec)
        });
        self.left.retain(|xs| {
            if xs.kind() == TesItemKind::HandToHand {
                return true;
            }
            cxx::let_cxx_string!(form_spec = xs.form_string());
            hasItemOrSpell(&form_spec)
        });
        self.right.retain(|xs| {
            if xs.kind() == TesItemKind::HandToHand {
                return true;
            }
            cxx::let_cxx_string!(form_spec = xs.form_string());
            hasItemOrSpell(&form_spec)
        });
    }

    /// Attempt to set the current item in a cycle to the given form spec (mod.esp|formid).
    ///
    /// Responds with the entry for the item that ends up being the current for that
    /// cycle, and None if the cycle is empty. If the item is not found, we do not
    /// change the state of the cycle in any way.
    pub fn set_top(&mut self, which: Action, item: &TesItemData) {
        let cycle = match which {
            Action::Power => &mut self.power,
            Action::Left => &mut self.left,
            Action::Right => &mut self.right,
            Action::Utility => &mut self.utility,
            _ => {
                return;
            }
        };

        if let Some(idx) = cycle.iter().position(|xs| xs == item) {
            cycle.rotate_left(idx);
        }
    }

    // the programmer error is annoying, but it's a shared struct...
    pub fn get_top(&self, which: Action) -> Option<TesItemData> {
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
    pub fn toggle(&mut self, which: Action, item: TesItemData) -> MenuEventResponse {
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

        // We have at most 15 items, so we do this blithely.
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

    pub fn update_count(&mut self, item: TesItemData, count: u32) -> bool {
        if item.kind().is_utility() {
            if let Some(candidate) = self.utility.iter_mut().find(|xs| **xs == item) {
                log::debug!(
                    "updating count for tracked item; formID={}; count={count}",
                    item.form_string()
                );
                candidate.set_count(count);
            }
            if count == 0 {
                self.utility.retain(|xs| xs.count() > 0);
                return true;
            }
        }
        false
    }

    pub fn include_item(&mut self, which: Action, item: TesItemData) {
        let cycle = match which {
            Action::Power => &mut self.power,
            Action::Left => &mut self.left,
            Action::Right => &mut self.right,
            Action::Utility => &mut self.utility,
            _ => {
                return;
            }
        };
        if !cycle
            .iter()
            .any(|xs| xs.kind() == item.kind() || xs.form_string() == item.form_string())
        {
            cycle.push(item);
        }
    }

    pub fn filter_kind(&mut self, which: Action, kind: TesItemKind) {
        let cycle = match which {
            Action::Power => &mut self.power,
            Action::Left => &mut self.left,
            Action::Right => &mut self.right,
            Action::Utility => &mut self.utility,
            _ => {
                return;
            }
        };
        cycle.retain(|xs| xs.kind() != kind);
    }

    pub fn set_hud_visible(&mut self, visible: bool) {
        if visible != self.hud_visible {
            self.hud_visible = visible;
            match self.write() {
                Ok(_) => {}
                Err(e) => {
                    log::warn!("failed to persist cycle data on visibility change; {e:?}");
                }
            }
        }
    }

    pub fn toggle_hud(&mut self) {
        self.set_hud_visible(!self.hud_visible);
    }

    pub fn hud_visible(&mut self) -> bool {
        self.hud_visible
    }
}

impl Display for CycleData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\npower: {};\nutility: {};\nleft: {};\nright: {};",
            vec_to_debug_string(&self.power),
            vec_to_debug_string(&self.utility),
            vec_to_debug_string(&self.left),
            vec_to_debug_string(&self.right)
        )
    }
}

fn vec_to_debug_string(input: &[TesItemData]) -> String {
    format!(
        "[{}]",
        input
            .iter()
            .map(|xs| xs.name())
            .collect::<Vec<String>>()
            .join(", ")
    )
}

impl Display for TesItemData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

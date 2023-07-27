//! Management of the cycle data: serialization and mutation.
//!
use std::fmt::Display;
use std::path::PathBuf;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::control::MenuEventResponse;
use super::itemdata::*;
use super::user_settings;
use crate::plugin::{fadeToAlpha, hasItemOrSpell, playerName, Action, ItemKind};

/// Manage the player's configured item cycles. Track changes, persist data in
/// files, and advance the cycle when the player presses a cycle button. This
/// struct now holds all data we need to persist across game starts.
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct CycleData {
    // I really want these to be maps, but toml can't serialize that.
    // Guess I could write to json instead.
    left: Vec<ItemData>,
    right: Vec<ItemData>,
    power: Vec<ItemData>,
    utility: Vec<ItemData>,
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
    pub fn advance(&mut self, which: Action, amount: usize) -> Option<ItemData> {
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
            previous.set_highlighted(false);
        }
        cycle.rotate_left(amount);
        cycle.first().cloned()
    }

    pub fn advance_skipping(&mut self, which: Action, skip: ItemData) -> Option<ItemData> {
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
            previous.set_highlighted(false);
        }
        cycle.rotate_left(1);
        let candidate = cycle
            .iter()
            .find(|xs| xs.form_string() != skip.form_string());
        if let Some(v) = candidate {
            let result = v.clone();
            self.set_top(which, &result);
            Some(result)
        } else {
            log::trace!("advance skip found nothing?????");
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

    /// Remove any items that have vanished from the game or from the player's
    /// inventory.
    pub fn validate(&mut self) {
        let to_check = vec![
            (Action::Power, "power"),
            (Action::Utility, "utility"),
            (Action::Left, "left"),
            (Action::Right, "right"),
        ];
        to_check.iter().for_each(|xs| {
            let action = xs.0;
            let name = xs.1;
            let cycle = match action {
                Action::Power => &self.power,
                Action::Utility => &self.utility,
                Action::Left => &self.left,
                Action::Right => &self.right,
                _ => &self.power, // I hate non-exhaustive matching
            };
            log::info!("validating {name} cycle");
            cycle.iter().for_each(|item| {
                cxx::let_cxx_string!(form_spec = item.form_string());
                let hasit = hasItemOrSpell(&form_spec);
                log::info!(
                    "    {}: name='{}'; form={}; player has={};",
                    name,
                    item.name(),
                    item.form_string(),
                    hasit
                );
            });
        });

        /*
        self.power.retain(|xs| {
            cxx::let_cxx_string!(form_spec = xs.form_string());
            hasItemOrSpell(&form_spec)
        });
        self.utility.retain(|xs| {
            if xs.kind().is_ammo() {
                return true;
            }
            cxx::let_cxx_string!(form_spec = xs.form_string());
            hasItemOrSpell(&form_spec)
        });
        self.left.retain(|xs| {
            if xs.kind() == ItemKind::HandToHand {
                return true;
            }
            cxx::let_cxx_string!(form_spec = xs.form_string());
            hasItemOrSpell(&form_spec)
        });
        self.right.retain(|xs| {
            if xs.kind() == ItemKind::HandToHand {
                return true;
            }
            cxx::let_cxx_string!(form_spec = xs.form_string());
            hasItemOrSpell(&form_spec)
        });
        */
    }

    /// Attempt to set the current item in a cycle to the given form spec (mod.esp|formid).
    ///
    /// Responds with the entry for the item that ends up being the current for that
    /// cycle, and None if the cycle is empty. If the item is not found, we do not
    /// change the state of the cycle in any way.
    pub fn set_top(&mut self, which: Action, item: &ItemData) {
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
    pub fn get_top(&self, which: Action) -> Option<ItemData> {
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

    pub fn peek_next(&self, which: Action) -> Option<ItemData> {
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
        cycle.get(1).cloned()
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
    pub fn toggle(&mut self, which: Action, item: ItemData) -> MenuEventResponse {
        let cycle = match which {
            Action::Power => {
                if !item.kind().is_power() {
                    return MenuEventResponse::ItemInappropriate;
                }
                &mut self.power
            }
            Action::Left => {
                if item.two_handed() {
                    return MenuEventResponse::ItemInappropriate;
                }
                &mut self.left
            }
            Action::Right => {
                if !item.kind().right_hand_ok() {
                    return MenuEventResponse::ItemInappropriate;
                }
                &mut self.right
            }
            Action::Utility => {
                if !item.kind().is_utility() {
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

    pub fn update_count(&mut self, item: ItemData, count: u32) -> bool {
        if item.kind().is_utility() {
            if let Some(candidate) = self.utility.iter_mut().find(|xs| **xs == item) {
                log::trace!(
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

    pub fn includes(&self, which: Action, item: &ItemData) -> bool {
        let cycle = match which {
            Action::Power => &self.power,
            Action::Left => &self.left,
            Action::Right => &self.right,
            Action::Utility => &self.utility,
            _ => {
                return false;
            }
        };
        cycle
            .iter()
            .any(|xs| xs.form_string() == item.form_string())
    }

    pub fn include_item(&mut self, which: Action, item: ItemData) {
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

    pub fn filter_kind(&mut self, which: Action, kind: ItemKind) {
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
            if visible {
                fadeToAlpha(true, 1.0);
            } else {
                fadeToAlpha(false, 0.0);
            }
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

fn vec_to_debug_string(input: &[ItemData]) -> String {
    format!(
        "[{}]",
        input
            .iter()
            .map(|xs| xs.name())
            .collect::<Vec<String>>()
            .join(", ")
    )
}

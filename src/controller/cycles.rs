//! Management of the cycle data: serialization and mutation.

use std::fmt::Display;
use std::path::PathBuf;

use anyhow::Result;
use cxx::CxxVector;
use serde::{Deserialize, Serialize};

use super::control::MenuEventResponse;
use super::itemdata::*;
use super::keys::CycleSlot;
use super::user_settings;
use crate::plugin::{hasItemOrSpell, playerName, startAlphaTransition, ItemKind};

/// Manage the player's configured item cycles. Track changes, persist data in
/// files, and advance the cycle when the player presses a cycle button. This
/// struct now holds all data we need to persist across game starts.
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct CycleData {
    left: Vec<ItemData>,
    right: Vec<ItemData>,
    power: Vec<ItemData>,
    utility: Vec<ItemData>,
    #[serde(default)]
    hud_visible: bool,
    #[serde(default)]
    pub loaded: bool,
}

// where to persist
static CYCLE_PATH: &str = "./data/SKSE/Plugins";

impl CycleData {
    /// Advance the given cycle by one. Returns a copy of the newly-top item.
    ///
    /// Called when the player presses a hotkey bound to one of the cycle slots.
    /// This does not equip or try to use the item in any way. It's pure management.
    pub fn advance(&mut self, which: &CycleSlot, amount: usize) -> Option<ItemData> {
        let cycle = match which {
            CycleSlot::Power => &mut self.power,
            CycleSlot::Left => &mut self.left,
            CycleSlot::Right => &mut self.right,
            CycleSlot::Utility => &mut self.utility,
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

    pub fn advance_skipping(&mut self, which: &CycleSlot, skip: ItemData) -> Option<ItemData> {
        let cycle = match which {
            CycleSlot::Power => &mut self.power,
            CycleSlot::Left => &mut self.left,
            CycleSlot::Right => &mut self.right,
            CycleSlot::Utility => &mut self.utility,
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
    pub fn cycle_len(&self, which: &CycleSlot) -> usize {
        match which {
            CycleSlot::Power => self.power.len(),
            CycleSlot::Left => self.left.len(),
            CycleSlot::Right => self.right.len(),
            CycleSlot::Utility => self.utility.len(),
        }
    }

    /// Attempt to set the current item in a cycle to the given form spec (mod.esp|formid).
    ///
    /// Responds with the entry for the item that ends up being the current for that
    /// cycle, and None if the cycle is empty. If the item is not found, we do not
    /// change the state of the cycle in any way.
    pub fn set_top(&mut self, which: &CycleSlot, item: &ItemData) {
        let cycle = match which {
            CycleSlot::Power => &mut self.power,
            CycleSlot::Left => &mut self.left,
            CycleSlot::Right => &mut self.right,
            CycleSlot::Utility => &mut self.utility,
        };

        if let Some(idx) = cycle.iter().position(|xs| xs == item) {
            cycle.rotate_left(idx);
        }
    }

    // the programmer error is annoying, but it's a shared struct...
    pub fn get_top(&self, which: &CycleSlot) -> Option<ItemData> {
        let cycle = match which {
            CycleSlot::Power => &self.power,
            CycleSlot::Left => &self.left,
            CycleSlot::Right => &self.right,
            CycleSlot::Utility => &self.utility,
        };

        cycle.first().cloned()
    }

    pub fn peek_next(&self, which: &CycleSlot) -> Option<ItemData> {
        let cycle = match which {
            CycleSlot::Power => &self.power,
            CycleSlot::Left => &self.left,
            CycleSlot::Right => &self.right,
            CycleSlot::Utility => &self.utility,
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
    pub fn toggle(&mut self, which: &CycleSlot, item: ItemData) -> MenuEventResponse {
        let cycle = match which {
            CycleSlot::Power => {
                if !item.kind().is_power() {
                    return MenuEventResponse::ItemInappropriate;
                }
                &mut self.power
            }
            CycleSlot::Left => {
                if item.two_handed() {
                    return MenuEventResponse::ItemInappropriate;
                }
                &mut self.left
            }
            CycleSlot::Right => {
                if !item.kind().right_hand_ok() {
                    return MenuEventResponse::ItemInappropriate;
                }
                &mut self.right
            }
            CycleSlot::Utility => {
                if !item.kind().is_utility() {
                    return MenuEventResponse::ItemInappropriate;
                }
                &mut self.utility
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
        // TODO This does not update counts for other types!!!!!
        false
    }

    pub fn includes(&self, which: &CycleSlot, item: &ItemData) -> bool {
        let cycle = match which {
            CycleSlot::Power => &self.power,
            CycleSlot::Left => &self.left,
            CycleSlot::Right => &self.right,
            CycleSlot::Utility => &self.utility,
        };
        cycle
            .iter()
            .any(|xs| xs.form_string() == item.form_string())
    }

    pub fn include_item(&mut self, which: CycleSlot, item: ItemData) {
        let cycle = match which {
            CycleSlot::Power => &mut self.power,
            CycleSlot::Left => &mut self.left,
            CycleSlot::Right => &mut self.right,
            CycleSlot::Utility => &mut self.utility,
        };
        if !cycle
            .iter()
            .any(|xs| xs.kind() == item.kind() || xs.form_string() == item.form_string())
        {
            cycle.push(item);
        }
    }

    pub fn filter_kind(&mut self, which: &CycleSlot, kind: ItemKind) {
        let cycle = match which {
            CycleSlot::Power => &mut self.power,
            CycleSlot::Left => &mut self.left,
            CycleSlot::Right => &mut self.right,
            CycleSlot::Utility => &mut self.utility,
        };
        cycle.retain(|xs| xs.kind() != kind);
    }

    pub fn set_hud_visible(&mut self, visible: bool) {
        if visible != self.hud_visible {
            self.hud_visible = visible;
            if visible {
                startAlphaTransition(true, 1.0);
            } else {
                startAlphaTransition(false, 0.0);
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

    // ---------- validation

    /// Remove any items that have vanished from the game or from the player's
    /// inventory.
    pub fn validate(&mut self) {
        let to_check = vec![
            (CycleSlot::Power, "power"),
            (CycleSlot::Utility, "utility"),
            (CycleSlot::Left, "left"),
            (CycleSlot::Right, "right"),
        ];
        to_check.iter().for_each(|xs| {
            let name = xs.1;
            let cycle = match &xs.0 {
                CycleSlot::Power => &self.power,
                CycleSlot::Utility => &self.utility,
                CycleSlot::Left => &self.left,
                CycleSlot::Right => &self.right,
            };
            log::info!("validating {name} cycle");
            cycle.iter().for_each(|item| {
                cxx::let_cxx_string!(form_spec = item.form_string());
                let hasit = item.kind().is_ammo() || hasItemOrSpell(&form_spec);
                log::info!(
                    "    {}: name='{}'; kind={:?}; form={}; player has={};",
                    name,
                    item.name(),
                    item.kind(),
                    item.form_string(),
                    hasit
                );
            });
        });
        log::info!("hud_visible: {}", self.hud_visible);
        log::info!("Informational only. No changes made to cycle data. Have a nice day and remember to put on a cloak if it starts snowing.");
    }

    // ---------- TOML serialization

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
        if self.power.is_empty()
            && self.utility.is_empty()
            && self.left.is_empty()
            && self.right.is_empty()
        {
            log::debug!("Declining to write empty/default cycles.");
            return Ok(());
        }

        log::debug!(
            "writing cycles to disk; lengths are: powers={}; utilities={}; left={}; right={};",
            self.power.len(),
            self.utility.len(),
            self.left.len(),
            self.right.len()
        );
        let fname = CycleData::cycle_storage();
        let backup = format!("{}.bak", fname.display());
        std::fs::copy(fname, backup)?;
        let buf = toml::to_string(self)?;
        std::fs::write(CycleData::cycle_storage(), buf)?;
        Ok(())
    }

    /// Read cycle data from the serialization file for this character.
    pub fn read() -> Result<Self> {
        let buf = std::fs::read_to_string(CycleData::cycle_storage())?;
        let data = toml::from_str::<CycleData>(&buf)?;
        log::info!(
            "read cycle data from {}; initial cycle lengths are:",
            CycleData::cycle_storage().display()
        );
        log::info!(
            "powers={}; utilities={}; left={}; right={};",
            data.power.len(),
            data.utility.len(),
            data.left.len(),
            data.right.len()
        );
        Ok(data)
    }

    // rkyv serialization to cosave

    pub fn serialize(&self) -> Vec<u8> {
        archive::serialize(self)
    }

    pub fn deserialize(bytes: &CxxVector<u8>) -> Option<CycleData> {
        archive::deserialize(bytes)
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

mod archive {
    use bincode::{Decode, Encode};
    use cxx::CxxVector;

    use super::{CycleData, ItemData};

    // cosave implemention starts with a very packed set of bytes
    // I'm implementing this to the side of the toml for safety.

    pub fn serialize(cycle: &CycleData) -> Vec<u8> {
        let value = CycleSerialized::from(cycle);
        let config = bincode::config::standard();
        let bytes: Vec<u8> = bincode::encode_to_vec(value, config).unwrap_or_default();
        bytes
    }

    pub fn deserialize(bytes: &CxxVector<u8>) -> Option<CycleData> {
        let bytes: Vec<u8> = bytes.iter().copied().collect();
        let config = bincode::config::standard();
        match bincode::decode_from_slice::<CycleSerialized, _>(&bytes[..], config) {
            Ok((value, _len)) => {
                log::info!("Cycles successfully read from cosave data.");
                Some(value.into())
            }
            Err(e) => {
                log::error!("Bincode cannot decode the cosave data. len={}", bytes.len());
                log::error!("{e:?}");
                None
            }
        }
    }

    #[derive(Decode, Encode, Hash, Debug, Clone, PartialEq, Eq)]
    pub struct CycleSerialized {
        left: Vec<ItemSerialized>,
        right: Vec<ItemSerialized>,
        power: Vec<ItemSerialized>,
        utility: Vec<ItemSerialized>,
        hud_visible: bool,
    }

    #[derive(Decode, Encode, Hash, Debug, Clone, Default, PartialEq, Eq)]
    pub struct ItemSerialized {
        name: String,
        form_string: String,
        kind: u8,
        two_handed: bool,
        has_count: bool,
        count: u32,
    }

    impl From<&CycleData> for CycleSerialized {
        fn from(value: &CycleData) -> Self {
            Self {
                left: value.left.iter().map(|xs| xs.into()).collect(),
                right: value.right.iter().map(|xs| xs.into()).collect(),
                power: value.power.iter().map(|xs| xs.into()).collect(),
                utility: value.utility.iter().map(|xs| xs.into()).collect(),
                hud_visible: value.hud_visible,
            }
        }
    }

    impl From<CycleSerialized> for CycleData {
        fn from(value: CycleSerialized) -> Self {
            Self {
                left: value.left.iter().map(|xs| xs.into()).collect(),
                right: value.right.iter().map(|xs| xs.into()).collect(),
                power: value.power.iter().map(|xs| xs.into()).collect(),
                utility: value.utility.iter().map(|xs| xs.into()).collect(),
                hud_visible: value.hud_visible,
                loaded: true,
            }
        }
    }

    impl From<&ItemData> for ItemSerialized {
        fn from(value: &ItemData) -> Self {
            Self {
                name: value.name(),
                form_string: value.form_string(),
                kind: value.kind().repr,
                two_handed: value.two_handed(),
                has_count: value.has_count(),
                count: value.count(),
            }
        }
    }

    impl From<&ItemSerialized> for ItemData {
        fn from(value: &ItemSerialized) -> ItemData {
            ItemData::new(
                value.kind.into(),
                value.two_handed,
                value.has_count,
                value.count,
                &value.name,
                &value.form_string,
            )
        }
    }
}

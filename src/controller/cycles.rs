//! Management of the cycle data: serialization and mutation.

use std::fmt::Display;

use cxx::CxxVector;
use serde::{Deserialize, Serialize};

use super::control::MenuEventResponse;
use super::keys::CycleSlot;
use super::user_settings;
use crate::data::{BaseType, HudItem, IsHudItem};
#[cfg(target_os = "windows")]
use crate::plugin::playerName;
use crate::plugin::{
    hasItemOrSpell, healthPotionCount, itemCount, magickaPotionCount, staminaPotionCount,
    startAlphaTransition,
};

/// Manage the player's configured item cycles. Track changes, persist data in
/// files, and advance the cycle when the player presses a cycle button. This
/// struct now holds all data we need to persist across game starts.
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct CycleData {
    left: Vec<HudItem>,
    right: Vec<HudItem>,
    power: Vec<HudItem>,
    utility: Vec<HudItem>,
    #[serde(default)]
    hud_visible: bool,
    #[serde(default)]
    pub loaded: bool,
}

impl CycleData {
    /// Advance the given cycle by one. Returns a copy of the newly-top item.
    ///
    /// Called when the player presses a hotkey bound to one of the cycle slots.
    /// This does not equip or try to use the item in any way. It's pure management.
    pub fn advance(&mut self, which: &CycleSlot, amount: usize) -> Option<HudItem> {
        let cycle = match which {
            CycleSlot::Power => &mut self.power,
            CycleSlot::Left => &mut self.left,
            CycleSlot::Right => &mut self.right,
            CycleSlot::Utility => &mut self.utility,
        };
        if cycle.is_empty() {
            return None;
        }
        cycle.rotate_left(amount);
        cycle.first().cloned()
    }

    pub fn advance_skipping(&mut self, which: &CycleSlot, skip: HudItem) -> Option<HudItem> {
        let cycle = match which {
            CycleSlot::Power => &mut self.power,
            CycleSlot::Left => &mut self.left,
            CycleSlot::Right => &mut self.right,
            CycleSlot::Utility => &mut self.utility,
        };
        if cycle.is_empty() {
            return None;
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

    pub fn advance_skipping_twohanders(&mut self) -> Option<HudItem> {
        // This can only be called for the right hand.
        if self.right.is_empty() {
            return None;
        }

        self.right.rotate_left(1);
        let candidate = self.right.iter().find(|xs| !xs.two_handed());
        if let Some(v) = candidate {
            let result = v.clone();
            self.set_top(&CycleSlot::Right, &result);
            Some(result)
        } else {
            log::trace!("no one-handers in the right cycle");
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
    pub fn set_top(&mut self, which: &CycleSlot, item: &HudItem) {
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
    pub fn get_top(&self, which: &CycleSlot) -> Option<HudItem> {
        let cycle = match which {
            CycleSlot::Power => &self.power,
            CycleSlot::Left => &self.left,
            CycleSlot::Right => &self.right,
            CycleSlot::Utility => &self.utility,
        };

        cycle.first().cloned()
    }

    pub fn peek_next(&self, which: &CycleSlot) -> Option<HudItem> {
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
    pub fn toggle(&mut self, which: &CycleSlot, item: HudItem) -> MenuEventResponse {
        let cycle = match which {
            CycleSlot::Power => {
                if !matches!(item.kind(), BaseType::Power) {
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

        // We have at most 20 items, so we do this blithely.
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

    pub fn update_count_by_formid(&mut self, form_id: String, kind: &BaseType, count: u32) {
        // If count is zero, remove from any cycles it's in.
        // If count is zero and item is equipped, advance the relevant cycle.
        if kind.is_utility() {
            if let Some(found) = self
                .utility
                .iter_mut()
                .find(|xs| *xs.form_string() == form_id)
            {
                log::trace!("updating count for utility cycle item; count={count}; item: {found}");
                found.set_count(count);
                if count == 0 {
                    self.utility.retain(|xs| xs.count() > 0);
                }
            }
        };

        if kind.left_hand_ok() {
            if let Some(found) = self.left.iter_mut().find(|xs| *xs.form_string() == form_id) {
                log::trace!("updating count for left cycle item; count={count}; item: {found}");
                found.set_count(count);
                if count == 0 {
                    self.left.retain(|xs| xs.count() > 0);
                }
            }
        }

        if kind.right_hand_ok() {
            if let Some(found) = self
                .right
                .iter_mut()
                .find(|xs| *xs.form_string() == form_id)
            {
                log::trace!("updating count for right cycle item; count={count}; item: {found}");
                found.set_count(count);
                if count == 0 {
                    self.right.retain(|xs| xs.count() > 0);
                }
            }
        }
    }

    pub fn update_count(&mut self, item: HudItem, count: u32) {
        self.update_count_by_formid(item.form_string(), item.kind(), count);
    }

    pub fn includes(&self, which: &CycleSlot, item: &HudItem) -> bool {
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

    pub fn include_item(&mut self, which: CycleSlot, item: &HudItem) -> bool {
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
            cycle.push(item.clone());
            true
        } else {
            false
        }
    }

    pub fn add_item(&mut self, which: CycleSlot, item: &HudItem) -> bool {
        let cycle = match which {
            CycleSlot::Power => &mut self.power,
            CycleSlot::Left => &mut self.left,
            CycleSlot::Right => &mut self.right,
            CycleSlot::Utility => &mut self.utility,
        };
        if !cycle
            .iter()
            .any(|xs| xs.form_string() == item.form_string())
        {
            cycle.push(item.clone());
            true
        } else {
            false
        }
    }

    pub fn remove_item(&mut self, which: CycleSlot, item: &HudItem) -> bool {
        let cycle = match which {
            CycleSlot::Power => &mut self.power,
            CycleSlot::Left => &mut self.left,
            CycleSlot::Right => &mut self.right,
            CycleSlot::Utility => &mut self.utility,
        };

        let orig_len = cycle.len();
        cycle.retain(|xs| xs.form_string() != item.form_string());
        orig_len != cycle.len()
    }

    pub fn filter_kind(&mut self, which: &CycleSlot, unwanted: BaseType) {
        let cycle = match which {
            CycleSlot::Power => &mut self.power,
            CycleSlot::Left => &mut self.left,
            CycleSlot::Right => &mut self.right,
            CycleSlot::Utility => &mut self.utility,
        };
        cycle.retain(|xs| !matches!(xs.kind(), unwanted));
    }

    pub fn set_hud_visible(&mut self, visible: bool) {
        if visible != self.hud_visible {
            self.hud_visible = visible;
            if visible {
                startAlphaTransition(true, 1.0);
            } else {
                startAlphaTransition(false, 0.0);
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
                CycleSlot::Power => &mut self.power,
                CycleSlot::Utility => &mut self.utility,
                CycleSlot::Left => &mut self.left,
                CycleSlot::Right => &mut self.right,
            };
            log::info!("validating {name} cycle");
            let filtered: Vec<_> = cycle
                .iter_mut()
                .filter_map(|incoming| {
                    let mut item = incoming.clone();
                    let spec = item.form_string();
                    if spec.is_empty() {
                        return Some(item);
                    }

                    cxx::let_cxx_string!(form_spec = spec.clone());
                    if hasItemOrSpell(&form_spec) {
                        log::info!("    {incoming}");
                        return Some(item);
                    }

                    let count = match spec.as_str() {
                        "health_proxy" => healthPotionCount(),
                        "magicka_proxy" => magickaPotionCount(),
                        "stamina_proxy" => staminaPotionCount(),
                        _ => itemCount(&form_spec),
                    };
                    if count > 0 {
                        item.set_count(count);
                        log::info!("    {incoming}");
                        Some(item)
                    } else {
                        None
                    }
                })
                .collect();

            match &xs.0 {
                CycleSlot::Power => {
                    self.power = filtered;
                }
                CycleSlot::Utility => {
                    self.utility = filtered;
                }
                CycleSlot::Left => {
                    self.left = filtered;
                }
                CycleSlot::Right => {
                    self.right = filtered;
                }
            }
        });
        //log::info!("hud_visible: {}", self.hud_visible);
        log::info!("Have a nice day and remember to put on a cloak if it starts snowing.");
    }

    // bincode serialization to cosave

    pub fn serialize_version() -> u32 {
        archive_v1::VERSION
    }

    // bincode serialization to cosave

    pub fn serialize(&self) -> Vec<u8> {
        archive_v1::serialize(self)
    }

    pub fn deserialize(bytes: &CxxVector<u8>, version: u32) -> Option<CycleData> {
        if version == 0 {
            archive_v0::deserialize(bytes)
        } else if version == 1 {
            archive_v1::deserialize(bytes)
        } else {
            log::info!("The cosave data is format version {version}, which this version of the plugin has never heard of.");
            None
        }
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

fn vec_to_debug_string(input: &[HudItem]) -> String {
    format!(
        "[{}]",
        input
            .iter()
            .map(|xs| xs.name())
            .collect::<Vec<String>>()
            .join(", ")
    )
}

pub mod archive_v1 {
    use bincode::{Decode, Encode};
    use cxx::CxxVector;

    use super::CycleData;
    use crate::data::HudItem;
    use crate::data::base::BaseType;
    use crate::plugin::formSpecToHudItem;

    pub const VERSION: u32 = 1;

    pub fn serialize(cycle: &CycleData) -> Vec<u8> {
        let value = CycleSerialized::from(cycle);
        let config = bincode::config::standard();
        let bytes: Vec<u8> = bincode::encode_to_vec(value, config).unwrap_or_default();
        log::debug!(
            "writing cosave format version {VERSION}; data len={};",
            bytes.len()
        );
        bytes
    }

    pub fn deserialize(bytes: &CxxVector<u8>) -> Option<CycleData> {
        let bytes: Vec<u8> = bytes.iter().copied().collect();
        let config = bincode::config::standard();
        log::debug!(
            "reading cosave format version {VERSION}; data len={};",
            bytes.len()
        );

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

    /// The serialization format is a list of form strings. Two drivers for
    /// this choice: 1) It's compact. 2) It can be deserialized into any
    /// representation of a TES form item we want, thus making it not care about
    /// implementation details of the hud item cache.
    #[derive(Decode, Encode, Hash, Debug, Clone, PartialEq, Eq)]
    pub struct CycleSerialized {
        left: Vec<String>,
        right: Vec<String>,
        power: Vec<String>,
        utility: Vec<String>,
        hud_visible: bool,
    }

    impl From<&CycleData> for CycleSerialized {
        fn from(value: &CycleData) -> Self {
            Self {
                left: value.left.iter().map(|xs| xs.form_string()).collect(),
                right: value.right.iter().map(|xs| xs.form_string()).collect(),
                power: value.power.iter().map(|xs| xs.form_string()).collect(),
                utility: value.utility.iter().map(|xs| xs.form_string()).collect(),
                hud_visible: value.hud_visible,
            }
        }
    }

    impl From<CycleSerialized> for CycleData {
        fn from(value: CycleSerialized) -> Self {
            fn filter_func(xs: &String) -> Option<HudItem> {
                match xs.as_str() {
                    "health_proxy" => Some(*crate::data::make_unarmed_proxy()),
                    "magicka_proxy" => Some(crate::data::make_magicka_proxy(1)),
                    "stamina_proxy" => Some(crate::data::make_stamina_proxy(1)),
                    "unarmed_proxy" => Some(crate::data::make_health_proxy(1)),
                    "" => None,
                    _ => {
                        cxx::let_cxx_string!(form_spec = xs);
                        let found = *formSpecToHudItem(&form_spec);
                        if matches!(found.kind(), BaseType::Empty) {
                            None
                        } else {
                            Some(found)
                        }
                    }
                }
            }

            Self {
                left: value.left.iter().filter_map(filter_func).collect(),
                right: value.right.iter().filter_map(filter_func).collect(),
                power: value.power.iter().filter_map(filter_func).collect(),
                utility: value.utility.iter().filter_map(filter_func).collect(),
                hud_visible: value.hud_visible,
                loaded: true,
            }
        }
    }
}

pub mod archive_v0 {
    use bincode::{Decode, Encode};
    use cxx::CxxVector;

    use super::{CycleData, HudItem};
    use crate::{controller::itemdata::ItemData, plugin::formSpecToHudItem, data::base::BaseType};

    const VERSION: u8 = 0;

    pub fn deserialize(bytes: &CxxVector<u8>) -> Option<CycleData> {
        let bytes: Vec<u8> = bytes.iter().copied().collect();
        let config = bincode::config::standard();
        log::debug!(
            "reading cosave format version {VERSION}; data len={};",
            bytes.len()
        );
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
        name_bytes: Vec<u8>,
        form_string: String,
        kind: u8,
        two_handed: bool,
        has_count: bool,
        count: u32,
    }

    impl From<CycleSerialized> for CycleData {
        fn from(value: CycleSerialized) -> Self {

            fn filter_func(item: &ItemSerialized) -> Option<HudItem> {
                let formstr = item.form_string.clone();
                match formstr.as_str() {
                    "health_proxy" => Some(*crate::data::make_unarmed_proxy()),
                    "magicka_proxy" => Some(crate::data::make_magicka_proxy(1)),
                    "stamina_proxy" => Some(crate::data::make_stamina_proxy(1)),
                    "unarmed_proxy" => Some(crate::data::make_health_proxy(1)),
                    "" => None,
                    _ => {
                        cxx::let_cxx_string!(form_spec = formstr);
                        let found = *formSpecToHudItem(&form_spec);
                        if matches!(found.kind(), BaseType::Empty) {
                            None
                        } else {
                            Some(found)
                        }
                    }
                }
            }

            Self {
                left: value.left.iter().filter_map(filter_func).collect(),
                right: value.right.iter().filter_map(filter_func).collect(),
                power: value.power.iter().filter_map(filter_func).collect(),
                utility: value.utility.iter().filter_map(filter_func).collect(),
                hud_visible: value.hud_visible,
                loaded: true,
            }

        }
    }

    impl From<&ItemSerialized> for HudItem {
        fn from(value: &ItemSerialized) -> Self {
            todo!()
        }
    }

    impl From<&ItemSerialized> for ItemData {
        fn from(value: &ItemSerialized) -> ItemData {
            ItemData::new(
                value.kind.into(),
                value.two_handed,
                value.has_count,
                value.count,
                value.name_bytes.clone(),
                &value.form_string,
            )
        }
    }
}

//! Management of the cycle data: serialization and mutation.

use std::fmt::Display;

use cxx::CxxVector;

use super::control::MenuEventResponse;
use super::keys::CycleSlot;
use super::user_settings;
use crate::data::item_cache::ItemCache;
use crate::data::{BaseType, HudItem, IsHudItem};
use crate::plugin::{
    hasItemOrSpell, healthPotionCount, itemCount, magickaPotionCount, staminaPotionCount,
    startAlphaTransition,
};

/// Manage the player's configured item cycles. Track changes, persist data in
/// files, and advance the cycle when the player presses a cycle button. This
/// struct now holds all data we need to persist across game starts.
#[derive(Debug, Clone)]
pub struct CycleData {
    /// Vec of item formspecs. A formspec looks like "mod.esp|0xdeadbeef":
    /// mod esp file and form id delimited by |.
    left: Vec<String>,
    /// Right hand cycle formspecs.
    right: Vec<String>,
    /// Shouts and powers cycle formspecs.
    power: Vec<String>,
    /// Utility items and consumables formspecs.
    utility: Vec<String>,
    /// Was the hud visible when we saved?
    hud_visible: bool,
    /// Was this cycle loaded from a cosave or are we operating on defaults?
    pub loaded: bool,
}

impl Default for CycleData {
    fn default() -> Self {
        Self {
            left: Default::default(),
            right: Default::default(),
            power: Default::default(),
            utility: Default::default(),
            hud_visible: true,
            loaded: false,
        }
    }
}

impl CycleData {
    pub fn clear(&mut self) {
        self.power.clear();
        self.utility.clear();
        self.left.clear();
        self.right.clear();
    }

    pub fn names(&self, which: &CycleSlot, cache: &mut ItemCache) -> Vec<String> {
        let cycle = match which {
            CycleSlot::Power => &self.power,
            CycleSlot::Left => &self.left,
            CycleSlot::Right => &self.right,
            CycleSlot::Utility => &self.utility,
        };
        cycle
            .iter()
            .filter_map(|xs| cache.get_or_none(xs.as_str()).map(|xs| xs.name()))
            .collect::<Vec<_>>()
    }

    /// Advance the given cycle by one. Returns a copy of the newly-top item.
    ///
    /// Called when the player presses a hotkey bound to one of the cycle slots.
    /// This does not equip or try to use the item in any way. It's pure management.
    pub fn advance(&mut self, which: &CycleSlot, amount: usize) -> Option<String> {
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

    pub fn advance_skipping(&mut self, which: &CycleSlot, skip: HudItem) -> Option<String> {
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
        let candidate = cycle.iter().find(|xs| *xs != &skip.form_string());
        if let Some(v) = candidate {
            let result = v.clone();
            self.set_top(which, &result);
            Some(result)
        } else {
            log::trace!("advance skip found nothing?????");
            None
        }
    }

    pub fn advance_skipping_twohanders(&mut self, cache: &mut ItemCache) -> Option<String> {
        // This can only be called for the right hand.
        if self.right.is_empty() {
            return None;
        }

        // This requires cache lookups.
        self.right.rotate_left(1);
        let candidate = self.right.iter().find(|xs| {
            let item = cache.get(xs);
            !item.two_handed()
        });
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
    pub fn set_top(&mut self, which: &CycleSlot, form_spec: &String) {
        let cycle = match which {
            CycleSlot::Power => &mut self.power,
            CycleSlot::Left => &mut self.left,
            CycleSlot::Right => &mut self.right,
            CycleSlot::Utility => &mut self.utility,
        };

        if let Some(idx) = cycle.iter().position(|xs| xs == form_spec) {
            cycle.rotate_left(idx);
        }
    }

    /// What's next in the given cycle?
    pub fn get_top(&self, which: &CycleSlot) -> Option<String> {
        let cycle = match which {
            CycleSlot::Power => &self.power,
            CycleSlot::Left => &self.left,
            CycleSlot::Right => &self.right,
            CycleSlot::Utility => &self.utility,
        };

        cycle.first().cloned()
    }

    /// Peek at the next item without advancing.
    pub fn peek_next(&self, which: &CycleSlot) -> Option<String> {
        let cycle = match which {
            CycleSlot::Power => &self.power,
            CycleSlot::Left => &self.left,
            CycleSlot::Right => &self.right,
            CycleSlot::Utility => &self.utility,
        };

        if cycle.len() == 1 {
            cycle.first().cloned()
        } else {
            cycle.get(1).cloned()
        }
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
                if !matches!(item.kind(), BaseType::Power | BaseType::Shout(_)) {
                    return MenuEventResponse::ItemInappropriate;
                }
                &mut self.power
            }
            CycleSlot::Left => {
                if !item.kind().left_hand_ok() {
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
        if let Some(idx) = cycle.iter().position(|xs| *xs == item.form_string()) {
            cycle.remove(idx);
            MenuEventResponse::ItemRemoved
        } else if cycle.len() >= settings.maxlen() as usize {
            return MenuEventResponse::TooManyItems;
        } else {
            cycle.push(item.form_string());
            MenuEventResponse::ItemAdded
        }
    }

    pub fn remove_zero_count_items(&mut self, form_spec: &str, kind: &BaseType, count: u32) {
        // If count is zero, remove from any cycles it's in.
        // If count is zero and item is equipped, advance the relevant cycle.
        if count > 0 {
            return;
        }

        if kind.is_utility() {
            self.utility.retain(|xs| xs != form_spec);
        }
        if kind.left_hand_ok() {
            self.left.retain(|xs| xs != form_spec);
        }
        if kind.right_hand_ok() {
            self.right.retain(|xs| xs != form_spec);
        }
    }

    pub fn includes(&self, which: &CycleSlot, item: &HudItem) -> bool {
        let cycle = match which {
            CycleSlot::Power => &self.power,
            CycleSlot::Left => &self.left,
            CycleSlot::Right => &self.right,
            CycleSlot::Utility => &self.utility,
        };
        cycle.iter().any(|xs| *xs == item.form_string())
    }

    pub fn include_item(&mut self, which: CycleSlot, item: &HudItem) -> bool {
        let cycle = match which {
            CycleSlot::Power => &mut self.power,
            CycleSlot::Left => &mut self.left,
            CycleSlot::Right => &mut self.right,
            CycleSlot::Utility => &mut self.utility,
        };
        let form = item.form_string();
        if cycle.iter().any(|xs| xs == &form) {
            false // we've already got one
        } else {
            cycle.push(form);
            true
        }
    }

    pub fn add_item(&mut self, which: CycleSlot, item: &HudItem) -> bool {
        let cycle = match which {
            CycleSlot::Power => &mut self.power,
            CycleSlot::Left => &mut self.left,
            CycleSlot::Right => &mut self.right,
            CycleSlot::Utility => &mut self.utility,
        };
        let form = item.form_string();
        if cycle.iter().any(|xs| xs == &form) {
            false
        } else {
            cycle.push(form);
            true
        }
    }

    pub fn remove_item(&mut self, which: CycleSlot, item: &HudItem) -> bool {
        let cycle = match which {
            CycleSlot::Power => &mut self.power,
            CycleSlot::Left => &mut self.left,
            CycleSlot::Right => &mut self.right,
            CycleSlot::Utility => &mut self.utility,
        };

        let form = item.form_string();
        let orig_len = cycle.len();
        cycle.retain(|xs| *xs != form);
        orig_len != cycle.len()
    }

    pub fn filter_kind(&mut self, which: &CycleSlot, unwanted: &BaseType, cache: &mut ItemCache) {
        let cycle = match which {
            CycleSlot::Power => &mut self.power,
            CycleSlot::Left => &mut self.left,
            CycleSlot::Right => &mut self.right,
            CycleSlot::Utility => &mut self.utility,
        };
        cycle.retain(|xs| {
            let found = cache.get(xs);
            found.kind() != unwanted
        });
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
    /// inventory. This is called rarely and at times where we can spend the
    /// cycles to look up the answer.
    pub fn validate(&mut self, cache: &mut ItemCache) {
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
                .iter()
                .filter_map(|incoming| {
                    let spec = incoming.clone();
                    let item = cache.get(&spec);

                    cxx::let_cxx_string!(form_spec = spec.clone());
                    if hasItemOrSpell(&form_spec) {
                        log::info!("    {item}");
                        return Some(spec);
                    }

                    let count = match spec.as_str() {
                        "health_proxy" => healthPotionCount(),
                        "magicka_proxy" => magickaPotionCount(),
                        "stamina_proxy" => staminaPotionCount(),
                        "unarmed_proxy" => 1,
                        _ => itemCount(&form_spec),
                    };
                    if count > 0 {
                        log::info!("    {incoming}");
                        Some(spec)
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

fn vec_to_debug_string(input: &[String]) -> String {
    format!("[{}]", input.join(", "))
}

pub mod archive_v1 {
    use bincode::{Decode, Encode};
    use cxx::CxxVector;

    use super::CycleData;
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
                left: value.left.to_vec(),
                right: value.right.to_vec(),
                power: value.power.to_vec(),
                utility: value.utility.to_vec(),
                hud_visible: value.hud_visible,
            }
        }
    }

    impl From<CycleSerialized> for CycleData {
        fn from(value: CycleSerialized) -> Self {
            fn filter_func(xs: &String) -> Option<String> {
                match xs.as_str() {
                    "health_proxy" => Some(xs.clone()),
                    "magicka_proxy" => Some(xs.clone()),
                    "stamina_proxy" => Some(xs.clone()),
                    "unarmed_proxy" => Some(xs.clone()),
                    "" => None,
                    _ => {
                        cxx::let_cxx_string!(form_spec = xs);
                        // Noting here that we do not go through the cache at all
                        // while loading these items. We probably should. TODO
                        let found = *formSpecToHudItem(&form_spec);
                        if matches!(found.kind(), BaseType::Empty) {
                            None
                        } else {
                            Some(found.form_string())
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

    use super::CycleData;
    use crate::data::base::BaseType;
    use crate::plugin::formSpecToHudItem;

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
            fn filter_func(item: &ItemSerialized) -> Option<String> {
                let formstr = item.form_string.clone();
                match formstr.as_str() {
                    "health_proxy" => Some(formstr.clone()),
                    "magicka_proxy" => Some(formstr.clone()),
                    "stamina_proxy" => Some(formstr.clone()),
                    "unarmed_proxy" => Some(formstr.clone()),
                    "" => None,
                    _ => {
                        cxx::let_cxx_string!(form_spec = formstr);
                        let found = *formSpecToHudItem(&form_spec);
                        if matches!(found.kind(), BaseType::Empty) {
                            None
                        } else {
                            Some(found.form_string())
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

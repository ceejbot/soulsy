//! Management of the cycle data: serialization and mutation.

use std::fmt::Display;

use cxx::CxxVector;

use super::control::MenuEventResponse;
use super::cycleentries::*;
use super::keys::CycleSlot;
use super::user_settings;
use crate::data::icons::Icon;
use crate::data::item_cache::ItemCache;
use crate::data::{BaseType, HudItem, IsHudItem};
use crate::plugin::{
    hasItemOrSpell, healthPotionCount, itemCount, magickaPotionCount, staminaPotionCount,
    startAlphaTransition, EquippedData,
};

/// Manage the player's configured item cycles. Track changes, persist data in
/// files, and advance the cycle when the player presses a cycle button. This
/// struct now holds all data we need to persist across game starts.
#[derive(Debug, Clone)]
pub struct CycleData {
    /// Vec of item formspecs. A formspec looks like "mod.esp|0xdeadbeef":
    /// mod esp file and form id delimited by |.
    pub left: Vec<String>,
    /// Right hand cycle formspecs.
    right: Vec<String>,
    /// Shouts and powers cycle formspecs.
    power: Vec<String>,
    /// Utility items and consumables formspecs.
    utility: Vec<String>,
    /// Equipment sets.
    equipsets: Vec<EquipSet>,
    /// Was the hud visible when we saved?
    pub hud_visible: bool,
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
            equipsets: Default::default(),
            hud_visible: true,
            loaded: false,
        }
    }
}

impl CycleData {
    /// Clear all cycles, including the equipsets.
    pub fn clear(&mut self) {
        self.power.clear();
        self.utility.clear();
        self.left.clear();
        self.right.clear();
        self.equipsets.clear();
    }

    /// Internal use only. Get a mutable reference to the named cycle.
    fn get_cycle_mut(&mut self, which: &CycleSlot) -> &mut Vec<String> {
        match which {
            CycleSlot::Power => &mut self.power,
            CycleSlot::Left => &mut self.left,
            CycleSlot::Right => &mut self.right,
            CycleSlot::Utility => &mut self.utility,
        }
    }

    /// Internal use only. Get the cycle for the given slot for reads.
    fn get_cycle(&self, which: &CycleSlot) -> &Vec<String> {
        match which {
            CycleSlot::Power => &self.power,
            CycleSlot::Left => &self.left,
            CycleSlot::Right => &self.right,
            CycleSlot::Utility => &self.utility,
        }
    }

    pub fn names(&self, which: &CycleSlot, cache: &mut ItemCache) -> Vec<String> {
        self.get_cycle(which).names(cache)
    }

    pub fn formids(&self, which: &CycleSlot) -> Vec<String> {
        let cycle = match which {
            CycleSlot::Power => &self.power,
            CycleSlot::Left => &self.left,
            CycleSlot::Right => &self.right,
            CycleSlot::Utility => &self.utility,
        };
        cycle.to_vec()
    }

    /// Advance the given cycle by one. Returns a copy of the newly-top item.
    ///
    /// Called when the player presses a hotkey bound to one of the cycle slots.
    /// This does not equip or try to use the item in any way. It's pure management.
    pub fn advance(&mut self, which: &CycleSlot, amount: usize) -> Option<String> {
        self.get_cycle_mut(which).advance(amount)
    }

    /// Advance the given cycle, skipping over the passed-in item if necessary.
    pub fn advance_skipping(&mut self, which: &CycleSlot, skip: HudItem) -> Option<String> {
        self.get_cycle_mut(which).advance_skipping(&skip)
    }

    /// Advance the right-hand cycle skipping over all two-handed items to the next one-hander.
    pub fn advance_skipping_twohanders(&mut self, cache: &mut ItemCache) -> Option<String> {
        // This is only relevant for the right hand.
        self.right.advance_skipping_twohanders(cache)
    }

    /// Get the length of the given cycle.
    pub fn cycle_len(&self, which: &CycleSlot) -> usize {
        self.get_cycle(which).len()
    }

    /// Attempt to set the current item in a cycle to the given form spec (mod.esp|formid).
    ///
    /// Responds with the entry for the item that ends up being the current for that
    /// cycle, and None if the cycle is empty. If the item is not found, we do not
    /// change the state of the cycle in any way.
    pub fn set_top(&mut self, which: &CycleSlot, form_spec: &str) {
        self.get_cycle_mut(which).set_top(form_spec);
    }

    /// What's next in the given cycle?
    pub fn get_top(&self, which: &CycleSlot) -> Option<String> {
        self.get_cycle(which).top().map(|xs| xs.identifier())
    }

    /// Peek at the next item without advancing.
    pub fn peek_next(&self, which: &CycleSlot) -> Option<String> {
        self.get_cycle(which).peek_next().map(|xs| xs.identifier())
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
        let spec = item.form_string();
        if cycle.includes(&spec) {
            cycle.delete(&spec);
            MenuEventResponse::ItemRemoved
        } else if cycle.len() >= settings.maxlen() as usize {
            return MenuEventResponse::TooManyItems;
        } else {
            cycle.add(&spec);
            MenuEventResponse::ItemAdded
        }
    }

    pub fn remove_zero_count_items(&mut self, form_spec: &str, kind: &BaseType, count: u32) {
        // If count is zero, remove from any cycles it's in.
        // If count is zero and item is equipped, advance the relevant cycle. <-- not happening erk
        if count > 0 {
            return;
        }

        if kind.is_utility() {
            self.utility.filter_id(form_spec);
        }
        if kind.left_hand_ok() {
            self.utility.filter_id(form_spec);
        }
        if kind.right_hand_ok() {
            self.utility.filter_id(form_spec);
        }
    }

    /// Check if the given cycle includes the example item or not.
    pub fn includes(&self, which: &CycleSlot, item: &HudItem) -> bool {
        self.get_cycle(which).includes(&item.form_string())
    }

    /// Make sure the given cycle includes this item, adding it if it does not.
    pub fn add_item(&mut self, which: CycleSlot, item: &HudItem) -> bool {
        self.get_cycle_mut(&which).add(&item.form_string())
    }

    pub fn remove_item(&mut self, which: CycleSlot, item: &HudItem) -> bool {
        self.get_cycle_mut(&which).delete(&item.form_string())
    }

    pub fn filter_kind(&mut self, which: &CycleSlot, unwanted: &BaseType, cache: &mut ItemCache) {
        self.get_cycle_mut(which).filter_kind(unwanted, cache);
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
                    let item = cache.get(&spec.identifier()); // works if vec of HudItem or vec<string>

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
        log::info!("Equipment sets:");
        self.equipsets.iter().for_each(|xs| {
            let names: Vec<String> = xs
                .items()
                .iter()
                .map(|xs| {
                    let item = cache.get(xs);
                    item.name()
                })
                .collect();
            log::info!("{}: {}", xs.id(), xs.name());
            log::info!("    {}", names.join(", "));
            log::info!("    {} empty slots", xs.empty_slots().len());
        });
        //log::info!("hud_visible: {}", self.hud_visible);
        log::info!("Have a nice day and remember to put on a cloak if it starts snowing.");
    }

    // equipset cycling

    pub fn get_top_equipset(&self) -> Option<EquipSet> {
        self.equipsets.top()
    }

    pub fn advance_equipset(&mut self, amount: usize) -> Option<EquipSet> {
        self.equipsets.advance(amount)
    }

    pub fn add_equipset(&mut self, name: String, data: EquippedData) -> bool {
        let id = self.equipsets.find_next_id();
        let set = EquipSet::new(
            id,
            name,
            data.items,
            data.empty_slots,
            "ArmorHeavy".to_string(),
        );
        self.equipsets.add(&set)
    }

    pub fn update_equipset(&mut self, id: u32, data: EquippedData) -> bool {
        self.equipsets.update_set(id, data.items, data.empty_slots)
    }

    pub fn remove_equipset(&mut self, id: String) -> bool {
        self.equipsets.filter_id(id.as_str())
    }

    pub fn rename_equipset(&mut self, id: u32, name: String) -> bool {
        self.equipsets.rename_by_id(id, name)
    }

    pub fn equipset_names(&self) -> Vec<String> {
        let mut sorted = self.equipsets.clone();
        sorted.sort_by_key(|xs| xs.id());
        sorted.iter().map(|xs| xs.name()).collect()
    }

    pub fn equipset_ids(&self) -> Vec<u32> {
        let mut ids = self
            .equipsets
            .iter()
            .map(|xs| xs.id())
            .collect::<Vec<u32>>();
        ids.sort();
        ids
    }

    pub fn equipset_by_id(&self, id: u32) -> Option<EquipSet> {
        self.equipsets.get_by_id(id).cloned()
    }

    pub fn equipset_by_name(&mut self, name: String) -> u32 {
        if let Some(set) = self.equipsets.iter().find(|xs| xs.name() == name) {
            set.id()
        } else {
            0
        }
    }

    pub fn set_icon_by_id(&mut self, id: u32, icon: Icon) -> bool {
        self.equipsets.set_icon_by_id(id, icon)
    }

    // bincode serialization to cosave

    pub fn serialize_version() -> u32 {
        cosave_v2::VERSION
    }

    pub fn serialize(&self) -> Vec<u8> {
        let value = cosave_v2::CycleSerialized::from(self);
        let config = bincode::config::standard();
        let bytes: Vec<u8> = bincode::encode_to_vec(value, config).unwrap_or_default();
        log::debug!(
            "writing cosave format version {}; data len={};",
            CycleData::serialize_version(),
            bytes.len()
        );
        bytes
    }

    pub fn deserialize(bytes: &CxxVector<u8>, version: u32) -> Option<CycleData> {
        let bytes: Vec<u8> = bytes.iter().copied().collect();
        match version {
            0 => cosave_v0::deserialize(bytes),
            1 => cosave_v1::deserialize(bytes),
            2 => cosave_v2::deserialize(bytes),
            _ => {
                log::warn!(
                    "Cosave data is version {version}, which this plugin version cannot decode."
                );
                None
            }
        }
    }
}

impl Display for CycleData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\npower: [{}];\nutility: [{}];\nleft: [{}];\nright: [{}];\nequipsets: [{}]",
            self.power.join(", "),
            self.utility.join(", "),
            self.left.join(", "),
            self.right.join(", "),
            self.equipsets
                .iter()
                .map(|xs| xs.name())
                .collect::<Vec<_>>()
                .join(",")
        )
    }
}

// cosave version modules.

pub mod cosave_v2 {
    use bincode::{Decode, Encode};

    use crate::controller::cycleentries::*;
    use crate::controller::cycles::CycleData;
    use crate::data::base::BaseType;
    use crate::data::item_cache::fetch_game_item;

    pub const VERSION: u32 = 2;

    pub fn deserialize(bytes: Vec<u8>) -> Option<CycleData> {
        let config = bincode::config::standard();
        log::debug!(
            "reading cosave format version {VERSION}; data len={};",
            bytes.len()
        );

        match bincode::decode_from_slice::<CycleSerialized, _>(&bytes[..], config) {
            Ok((value, _len)) => {
                log::info!("Cycles successfully read from cosave data version {VERSION}.");
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
    /// Rust type we want, thus making it not care about implementation details.
    /// So the struct uses only built-in rust types, no crate types.
    #[derive(Decode, Encode, Hash, Debug, Clone, PartialEq, Eq)]
    pub struct CycleSerialized {
        left: Vec<String>,
        right: Vec<String>,
        power: Vec<String>,
        utility: Vec<String>,
        // Vec of tuples of (id, name, Vec<formspec>, Vec<empty_slot>, icon_as_string)
        equipsets: Vec<(u32, String, Vec<String>, Vec<u8>, String)>,
        hud_visible: bool,
    }

    impl From<&CycleData> for CycleSerialized {
        fn from(value: &CycleData) -> Self {
            Self {
                left: value.left.ids(),
                right: value.right.ids(),
                power: value.power.ids(),
                utility: value.utility.ids(),
                equipsets: value
                    .equipsets
                    .iter()
                    .map(|xs| {
                        (
                            xs.id(),
                            xs.name(),
                            xs.items.to_vec(),
                            xs.empty.to_vec(),
                            xs.icon.to_string(),
                        )
                    })
                    .collect(),
                hud_visible: value.hud_visible,
            }
        }
    }

    impl From<CycleSerialized> for CycleData {
        fn from(value: CycleSerialized) -> Self {
            fn filter_func(xs: &str) -> Option<String> {
                match xs {
                    "health_proxy" => Some(xs.to_owned()),
                    "magicka_proxy" => Some(xs.to_owned()),
                    "stamina_proxy" => Some(xs.to_owned()),
                    "unarmed_proxy" => Some(xs.to_owned()),
                    "" => None,
                    _ => {
                        // Noting here that we do not go through the cache at all
                        // while loading these items. We probably should. TODO
                        let found = fetch_game_item(xs);
                        if matches!(found.kind(), BaseType::Empty) {
                            None
                        } else {
                            Some(found.form_string())
                        }
                    }
                }
            }

            Self {
                left: value
                    .left
                    .iter()
                    .filter_map(|xs| filter_func(xs.as_str()))
                    .collect(),
                right: value
                    .right
                    .iter()
                    .filter_map(|xs| filter_func(xs.as_str()))
                    .collect(),
                power: value
                    .power
                    .iter()
                    .filter_map(|xs| filter_func(xs.as_str()))
                    .collect(),
                utility: value
                    .utility
                    .iter()
                    .filter_map(|xs| filter_func(xs.as_str()))
                    .collect(),
                hud_visible: value.hud_visible,
                equipsets: value
                    .equipsets
                    .iter()
                    .map(|xs| {
                        EquipSet::new(
                            xs.0,
                            xs.1.clone(),
                            xs.2.to_vec(),
                            xs.3.to_vec(),
                            xs.4.clone(),
                        )
                    })
                    .collect(),
                loaded: true,
            }
        }
    }
}

pub mod cosave_v1 {
    use bincode::{Decode, Encode};

    use crate::controller::cycles::CycleData;
    use crate::data::base::BaseType;
    use crate::data::item_cache::fetch_game_item;

    pub const VERSION: u32 = 1;

    pub fn deserialize(bytes: Vec<u8>) -> Option<CycleData> {
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

    /// Same comment as above: form spec strings are flexible.
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
            fn filter_func(xs: &str) -> Option<String> {
                match xs {
                    "health_proxy" => Some(xs.to_owned()),
                    "magicka_proxy" => Some(xs.to_owned()),
                    "stamina_proxy" => Some(xs.to_owned()),
                    "unarmed_proxy" => Some(xs.to_owned()),
                    "" => None,
                    _ => {
                        // Noting here that we do not go through the cache at all
                        // while loading these items. We probably should. TODO
                        let found = fetch_game_item(xs);
                        if matches!(found.kind(), BaseType::Empty) {
                            None
                        } else {
                            Some(found.form_string())
                        }
                    }
                }
            }

            Self {
                left: value
                    .left
                    .iter()
                    .filter_map(|xs| filter_func(xs.as_str()))
                    .collect(),
                right: value
                    .right
                    .iter()
                    .filter_map(|xs| filter_func(xs.as_str()))
                    .collect(),
                power: value
                    .power
                    .iter()
                    .filter_map(|xs| filter_func(xs.as_str()))
                    .collect(),
                utility: value
                    .utility
                    .iter()
                    .filter_map(|xs| filter_func(xs.as_str()))
                    .collect(),
                hud_visible: value.hud_visible,
                equipsets: Vec::new(),
                loaded: true,
            }
        }
    }
}

pub mod cosave_v0 {
    use bincode::{Decode, Encode};

    use crate::controller::cycles::CycleData;
    use crate::data::base::BaseType;
    use crate::data::item_cache::fetch_game_item;

    const VERSION: u8 = 0;

    pub fn deserialize(bytes: Vec<u8>) -> Option<CycleData> {
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
                        let found = fetch_game_item(&formstr);
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
                equipsets: Vec::new(),
                hud_visible: value.hud_visible,
                loaded: true,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plugin::EquippedData;

    #[test]
    fn version_2() {
        let mut cache = ItemCache::default();
        let mut cycle = CycleData::default();

        let one = cache.get(&"fake-one".to_string());
        let two = cache.get(&"fake-two".to_string());
        let three = cache.get(&"fake-three".to_string());
        cycle.add_item(CycleSlot::Left, &one);
        cycle.add_item(CycleSlot::Left, &two);
        cycle.add_item(CycleSlot::Left, &three);

        let data = EquippedData {
            items: Vec::new(),
            empty_slots: Vec::new(),
        };
        cycle.add_equipset("set-one".to_string(), data.clone());
        cycle.add_equipset("set-two".to_string(), data.clone());

        let value = cosave_v2::CycleSerialized::from(&cycle);
        let config = bincode::config::standard();
        let bytes: Vec<u8> = bincode::encode_to_vec(value, config).unwrap_or_default();
        let decoded = cosave_v2::deserialize(bytes).expect("data should be decodeable");
        assert_eq!(decoded.loaded, !cycle.loaded);
        assert_eq!(decoded.left.len(), cycle.left.len());
        assert_eq!(decoded.equipsets.len(), cycle.equipsets.len());
        let set1 = decoded
            .get_top_equipset()
            .expect("expected actual equipsets");
        assert_eq!(set1.id(), 0);
    }

    #[test]
    fn version_1() {
        let mut cache = ItemCache::default();
        let mut cycle = CycleData::default();

        let one = cache.get(&"fake-one".to_string());
        let two = cache.get(&"fake-two".to_string());
        let three = cache.get(&"fake-three".to_string());
        cycle.add_item(CycleSlot::Left, &one);
        cycle.add_item(CycleSlot::Left, &two);
        cycle.add_item(CycleSlot::Left, &three);

        let value = cosave_v1::CycleSerialized::from(&cycle);
        let config = bincode::config::standard();
        let bytes: Vec<u8> = bincode::encode_to_vec(value, config).unwrap_or_default();
        let decoded = cosave_v1::deserialize(bytes).expect("data should be decodeable");
        assert_eq!(decoded.loaded, !cycle.loaded);
        assert_eq!(decoded.left.len(), cycle.left.len());
    }

    #[test]
    fn version_0() {
        // lowest priority to write tests for;
        // only used to load from very old saves
    }
}

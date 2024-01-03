use std::collections::HashMap;
use std::fmt::Display;

#[cfg(not(test))]
use cxx::let_cxx_string;
use enumset::{enum_set, EnumSet, EnumSetType};
use strfmt::strfmt;

use super::base::BaseType;
use super::HasIcon;
#[cfg(not(test))]
use crate::plugin::{chargeLevelByFormSpec, hasChargeByFormSpec, isPoisonedByFormSpec};
use crate::plugin::{Color, ItemCategory};
use crate::{images::icons::Icon, plugin};

/// A TESForm item that the player can use or equip, with the data
/// that drives the HUD cached for fast access.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HudItem {
    /// Name as utf8
    name: String,
    /// A string that can be turned back into form data; for serializing.
    form_string: String,
    /// An enum classifying this item for fast question-answering as well as icon selection.
    kind: BaseType,
    /// Cached count from inventory data. Relies on hooks to be updated.
    count: u32,
    /// Hashmap used by variable substitution in the HUD renderer.
    format_vars: HashMap<String, String>,
    /// An attempt to cache some extra data. (Not names however!)
    extra: EnumSet<ItemExtraData>,
    /// Charge level, if relevant. As a percentage.
    charge_level: f32,
    /// Time remaining, if relevant.
    time_left: f32,
}

#[derive(Debug, Default, Hash, EnumSetType)]
pub enum ItemExtraData {
    #[default]
    None,
    IsPoisoned,
    IsEnchanted,
    HasTimeLeft,
    HasCooldown, // maybe?
}

impl Display for ItemExtraData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ItemExtraData::None => write!(f, "none"),
            ItemExtraData::IsPoisoned => write!(f, "poisoned"),
            ItemExtraData::IsEnchanted => write!(f, "enchanted"),
            ItemExtraData::HasTimeLeft => write!(f, "time left"),
            ItemExtraData::HasCooldown => write!(f, "cooling down"),
        }
    }
}

const CHARGE_INDICATORS: EnumSet<ItemExtraData> =
    enum_set!(ItemExtraData::HasCooldown | ItemExtraData::IsEnchanted | ItemExtraData::HasTimeLeft);

/// This is the item extra data the hud cares about and displays (full name
/// not included).
#[derive(Debug, Default, Clone)]
pub struct RelevantExtraData {
    has_charge: bool,
    charge: f32, // percentage
    is_poisoned: bool,
    has_time_left: bool,
    time_left: f32, // units unknown atm
}

/*

ExtraEditorID - maybe some use?
ExtraHotkey - is favorite
ExtraMagicLight - magic light timer? investigate
ExtraSoul - investigate

*/

pub fn empty_extra_data() -> Box<RelevantExtraData> {
    Box::new(RelevantExtraData::default())
}

impl RelevantExtraData {
    pub fn new(
        has_charge: bool,
        charge: f32,
        is_poisoned: bool,
        has_time_left: bool,
        time_left: f32,
    ) -> Self {
        Self {
            has_charge,
            charge,
            is_poisoned,
            has_time_left,
            time_left,
        }
    }
}

impl HudItem {
    pub fn from_keywords(
        category: ItemCategory,
        keywords: Vec<String>,
        name: String,
        form_string: String,
        count: u32,
        twohanded: bool,
    ) -> Self {
        // log::trace!("calling BaseType::classify() with keywords={keywords:?};");
        let kind: BaseType = BaseType::classify(name.as_str(), category, keywords, twohanded);
        let format_vars = HudItem::make_format_vars(name.clone(), count);
        Self {
            name,
            form_string,
            count,
            kind,
            format_vars,
            ..Default::default()
        }
    }

    pub fn preclassified(name: String, form_string: String, count: u32, kind: BaseType) -> Self {
        let format_vars = HudItem::make_format_vars(name.clone(), count);
        Self {
            name,
            form_string,
            count,
            kind,
            format_vars,
            ..Default::default()
        }
    }

    pub fn for_equip_set(name: String, id: u32, icon: Icon) -> Self {
        let format_vars = HudItem::make_format_vars(name.clone(), 1);
        Self {
            name,
            form_string: format!("equipset_{id}"),
            count: 1,
            kind: BaseType::Equipset(icon),
            format_vars,
            ..Default::default()
        }
    }

    pub fn make_unarmed_proxy() -> Self {
        HudItem::preclassified(
            "Unarmed".to_string(),
            "unarmed_proxy".to_string(),
            1,
            BaseType::HandToHand,
        )
    }

    fn make_format_vars(name: String, count: u32) -> HashMap<String, String> {
        let mut vars = HashMap::new();
        if name.is_empty() {
            vars.insert("(no name)".to_string(), name);
        } else {
            vars.insert("name".to_string(), name);
        }
        vars.insert("count".to_string(), count.to_string());
        vars
    }

    pub fn fmtstr(&self, fmt: String) -> String {
        // This implementation caches nothing. It might be fast enough?
        // needs measurement
        let charge = self.charge_level();
        let mut vars = self.format_vars.clone();
        vars.insert("charge".to_string(), format!("{:.0}", charge));
        match strfmt(&fmt, &vars) {
            Ok(v) => v,
            Err(e) => {
                log::trace!("Failed to render format string for HUD item; error: {e:#}");
                "".to_string()
            }
        }
    }

    pub fn icon(&self) -> &Icon {
        self.kind().icon()
    }

    pub fn icon_file(&self) -> String {
        self.kind().icon().icon_file()
    }

    pub fn icon_key(&self) -> String {
        crate::images::key_for_icon(self.kind().icon()).to_string()
    }

    pub fn color(&self) -> Color {
        self.kind.color()
    }

    pub fn kind(&self) -> &BaseType {
        &self.kind
    }

    pub fn form_string(&self) -> String {
        self.form_string.clone()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn count(&self) -> u32 {
        self.count
    }

    pub fn set_count(&mut self, v: u32) {
        self.count = v;
        let format_vars = HudItem::make_format_vars(self.name.clone(), self.count);
        self.format_vars = format_vars;
    }

    // probably can get rid of this
    pub fn has_extra(&self, kind: ItemExtraData) -> bool {
        self.extra.contains(kind)
    }

    /// Return true if this item is poisoned.
    /// Does not update local flags; okay to use in tight loops.
    pub fn is_poisoned(&self) -> bool {
        self.extra.contains(ItemExtraData::IsPoisoned)
    }

    /// Checks game extra data to see if this item is poisoned and updates the item with the result.
    pub fn is_poisoned_refresh(&mut self) -> bool {
        if !self.is_weapon() {
            false
        } else {
            #[cfg(not(test))]
            {
                let_cxx_string!(form_spec = self.form_string());
                if isPoisonedByFormSpec(&form_spec) {
                    self.extra.insert(ItemExtraData::IsPoisoned);
                } else {
                    self.extra.remove(ItemExtraData::IsPoisoned);
                }
            }
            #[cfg(test)]
            {
                if rand::random::<f32>() > 0.5 {
                    self.extra.insert(ItemExtraData::IsPoisoned);
                } else {
                    self.extra.remove(ItemExtraData::IsPoisoned);
                }
            }
            self.extra.contains(ItemExtraData::IsPoisoned)
        }
    }

    /// Return true if this item has something to display in a meter.
    /// Does not update local flags; okay to use in tight loops.
    pub fn has_charge(&self) -> bool {
        !self.extra.is_disjoint(CHARGE_INDICATORS)
    }

    #[cfg(test)]
    pub fn has_charge_refresh(&mut self) -> bool {
        rand::random::<f32>() > 0.5
    }

    #[cfg(not(test))]
    pub fn has_charge_refresh(&mut self) -> bool {
        let_cxx_string!(form_spec = self.form_string());
        if hasChargeByFormSpec(&form_spec) {
            self.extra.insert(ItemExtraData::IsEnchanted);
            // get level and record it
            self.charge_level = self.charge_level_refresh();
            true
        } else {
            self.extra.remove(ItemExtraData::IsEnchanted);
            false
        }
    }

    /// Returns charge percentage level.
    /// Does not update the object; okay to use in tight loops.
    pub fn charge_level(&self) -> f32 {
        self.charge_level
    }

    /// Charge as a float from 0.0 to 1.0 inclusive. For enchanted weapons
    /// and torches or other fueled items. Consults extra data.
    pub fn charge_level_refresh(&mut self) -> f32 {
        if self.is_armor() || self.is_weapon() || matches!(self.kind, BaseType::Light(_)) {
            #[cfg(not(test))]
            {
                let_cxx_string!(form_spec = self.form_string());
                self.charge_level = chargeLevelByFormSpec(&form_spec);
            }
            #[cfg(test)]
            {
                self.charge_level = rand::random::<f32>() * 100.0f32;
            }
            self.charge_level
        } else {
            0.0
        }
    }

    /// Cooldown remaining; okay to use in tight loops.
    pub fn time_left(&self) -> f32 {
        self.time_left
    }

    #[cfg(test)]
    pub fn refresh_extra_data(&mut self) {
        // randomize it all for tests, for now
        let has_charge = self.has_charge_refresh();
        if has_charge {
            self.charge_level_refresh();
        }
        self.is_poisoned_refresh();
        if rand::random::<f32>() > 0.5 {
            self.time_left = rand::random::<f32>() * 100.0f32;
            self.extra.insert(ItemExtraData::HasTimeLeft);
        } else {
            self.extra.remove(ItemExtraData::HasTimeLeft);
        }
    }

    #[cfg(not(test))]
    pub fn refresh_extra_data(&mut self) {
        cxx::let_cxx_string!(form_spec = self.form_string());
        let extra = *plugin::relevantExtraData(&form_spec);

        if extra.has_charge {
            self.extra.insert(ItemExtraData::IsEnchanted);
        } else {
            self.extra.remove(ItemExtraData::IsEnchanted);
        }
        self.charge_level = extra.charge;

        if extra.has_time_left {
            self.extra.insert(ItemExtraData::HasTimeLeft);
        } else {
            self.extra.remove(ItemExtraData::HasTimeLeft);
        }
        self.time_left = extra.time_left;

        if extra.is_poisoned {
            self.extra.insert(ItemExtraData::IsPoisoned);
        } else {
            self.extra.remove(ItemExtraData::IsPoisoned);
        }
    }

    // We delegate everything to our object-kind. The goal is for most things
    // not to need to know about the item kind mess. Note that these functions
    // are all from the trait IsHudItem, which we can't implement here because
    // we offer these functions to the C++ side.

    /// Delegated to item kind.
    pub fn count_matters(&self) -> bool {
        self.kind.count_matters()
    }

    /// Delegated to item kind.
    pub fn is_ammo(&self) -> bool {
        self.kind.is_ammo()
    }

    /// Delegated to item kind.
    pub fn is_armor(&self) -> bool {
        self.kind.is_armor()
    }

    /// Delegated to item kind.
    pub fn is_magic(&self) -> bool {
        self.kind.is_magic()
    }

    /// Delegated to item kind.
    pub fn is_potion(&self) -> bool {
        self.kind.is_potion()
    }

    /// Delegated to item kind.
    pub fn is_power(&self) -> bool {
        self.kind.is_power()
    }

    /// Delegated to item kind.
    pub fn is_spell(&self) -> bool {
        self.kind.is_spell()
    }

    /// Delegated to item kind.
    pub fn is_utility(&self) -> bool {
        self.kind.is_utility()
    }

    /// Delegated to item kind.
    pub fn is_weapon(&self) -> bool {
        self.kind.is_weapon()
    }

    /// Delegated to item kind.
    pub fn is_one_handed(&self) -> bool {
        self.kind.is_one_handed()
    }

    /// Delegated to item kind.
    pub fn left_hand_ok(&self) -> bool {
        self.kind.left_hand_ok()
    }

    /// Delegated to item kind.
    pub fn right_hand_ok(&self) -> bool {
        self.kind.right_hand_ok()
    }

    /// Delegated to item kind.
    pub fn two_handed(&self) -> bool {
        self.kind.is_two_handed()
    }
}

impl Display for HudItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "name='{}'; kind='{:?}'; count={}; form_string='{}';",
            self.name(),
            self.kind(),
            self.count(),
            self.form_string(),
        )
    }
}

use std::collections::HashMap;
use std::fmt::Display;

use strfmt::strfmt;

use super::base::BaseType;
use super::HasIcon;
use crate::images::icons::Icon;
#[cfg(not(test))]
use crate::plugin::relevantExtraData;
use crate::plugin::{Color, ItemCategory};

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
    extra: RelevantExtraData,
    /// record the max cooldown time we've seen for this shout
    shout_cooldown: f32,
    /// Meter level, if relevant. As a percentage.
    meter_level: f32,
}

/// This is the item extra data the hud cares about and displays (full name
/// not included).
#[derive(Debug, Default, Clone, PartialEq)]
pub struct RelevantExtraData {
    has_charge: bool,
    max_charge: f32,
    charge: f32,
    is_poisoned: bool,
    has_time_left: bool,
    max_time: f32,  // 0 if we don't know
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
        max_charge: f32,
        charge: f32,
        is_poisoned: bool,
        has_time_left: bool,
        max_time: f32,
        time_left: f32,
    ) -> Self {
        Self {
            has_charge,
            charge,
            max_charge,
            is_poisoned,
            has_time_left,
            max_time,
            time_left,
        }
    }

    #[cfg(test)]
    pub fn randomize() -> Self {
        let has_charge = rand::random::<f32>() > 0.5;
        let max_charge = if has_charge {
        } else {
            0.0
        };

        let has_time_left = rand::random::<f32>() > 0.5;
        let is_poisoned = rand::random::<f32>() > 0.5;

        Self {
            has_charge,
            max_charge,
            charge,
            is_poisoned,
            has_time_left,
            max_time,
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
        let mut result = Self {
            name,
            form_string,
            count,
            kind,
            ..Default::default()
        };
        result.make_format_vars();
        result
    }

    pub fn preclassified(name: String, form_string: String, count: u32, kind: BaseType) -> Self {
        let mut result = Self {
            name,
            form_string,
            count,
            kind,
            ..Default::default()
        };
        result.make_format_vars();
        result
    }

    pub fn for_equip_set(name: String, id: u32, icon: Icon) -> Self {
        let mut result = Self {
            name,
            form_string: format!("equipset_{id}"),
            count: 1,
            kind: BaseType::Equipset(icon),
            ..Default::default()
        };
        result.make_format_vars();
        result
    }

    pub fn make_unarmed_proxy() -> Self {
        HudItem::preclassified(
            "Unarmed".to_string(),
            "unarmed_proxy".to_string(),
            1,
            BaseType::HandToHand,
        )
    }

    fn make_format_vars(&mut self) {
        let mut vars = HashMap::new();
        if self.name.is_empty() {
            vars.insert("(no name)".to_string(), self.name.clone());
        } else {
            vars.insert("name".to_string(), self.name.clone());
        }
        vars.insert("count".to_string(), self.count.to_string());
        vars.insert(
            "charge_max".to_string(),
            format!("{:.0}", self.extra.max_charge),
        );
        vars.insert("charge".to_string(), format!("{:.0}", self.extra.charge));

        if self.is_power() {
            vars.insert(
                "time_max".to_string(),
                format!("{:.0}", self.shout_cooldown),
            );
        } else {
            vars.insert(
                "time_max".to_string(),
                format!("{:.0}", self.extra.max_time),
            );
        }
        vars.insert(
            "time_left".to_string(),
            format!("{:.0}", self.extra.time_left),
        );
        vars.insert(
            "meter_level".to_string(),
            format!("{:.0}", self.meter_level),
        );
        if self.extra.is_poisoned {
            vars.insert("poison".to_string(), "poison".to_string());
        } else {
            vars.insert("poison".to_string(), "".to_string());
        }
        self.format_vars = vars;
    }

    pub fn fmtstr(&self, fmt: String) -> String {
        // This implementation caches nothing. It might be fast enough?
        // needs measurement
        match strfmt(&fmt, &self.format_vars) {
            Ok(v) => v,
            Err(e) => {
                log::debug!("Failed to render format string for HUD item; error: {e:#}");
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
        self.make_format_vars();
    }

    /// Return true if this item is poisoned.
    /// Does not update local flags; okay to use in tight loops.
    pub fn is_poisoned(&self) -> bool {
        self.is_weapon() && self.extra.is_poisoned
    }

    /// Return true if this item has something to display in a meter.
    /// Does not update local flags; okay to use in tight loops.
    pub fn show_meter(&self) -> bool {
        self.extra.has_charge || self.extra.has_time_left
    }

    /// Returns meter/time left percentage level.
    /// Does not update the object; okay to use in tight loops.
    pub fn meter_level(&self) -> f32 {
        self.meter_level
    }

    /// Return true if this item is enchanted.
    pub fn is_enchanted(&self) -> bool {
        self.extra.has_charge
    }

    /// Get the charge level of this item's enchantment.
    /// Only meaningful for items like weapons.
    pub fn charge_level(&self) -> f32 {
        self.extra.charge
    }

    /// CHeck if this item has a cooldown or a duration.
    pub fn has_time_left(&self) -> bool {
        self.extra.has_time_left
    }

    /// Cooldown remaining; okay to use in tight loops.
    pub fn time_left(&self) -> f32 {
        self.extra.time_left
    }

    pub fn refresh_extra_data(&mut self) {
        #[cfg(test)]
        let extra = RelevantExtraData::randomize();

        #[cfg(not(test))]
        let extra = {
            cxx::let_cxx_string!(form_spec = self.form_string());
            *relevantExtraData(&form_spec)
        };

        if extra.has_charge {
            self.meter_level = extra.charge * 100.0 / extra.max_charge;
        } else if self.extra.has_time_left {
            if self.is_power() {
                if self.shout_cooldown <= extra.time_left {
                    self.shout_cooldown = extra.time_left;
                    self.meter_level = 0.0;
                } else {
                    self.meter_level = extra.time_left * 100.0 / self.shout_cooldown;
                }
            } else {
                self.meter_level = extra.time_left * 100.0 / extra.max_time;
            }
        }

        self.extra = extra;
        self.make_format_vars();
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

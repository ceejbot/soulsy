use std::collections::HashMap;
use std::fmt::Display;

use cxx::let_cxx_string;
use strfmt::strfmt;

use super::base::BaseType;
use super::HasIcon;
use crate::images::icons::Icon;
use crate::plugin::{
    chargeLevelByFormSpec, hasChargeByFormSpec, isPoisonedByFormSpec, Color, ItemCategory,
};

/// A TESForm item that the player can use or equip, with the data
/// that drives the HUD cached for fast access.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
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

    pub fn is_poisoned(&self) -> bool {
        if !self.is_weapon() {
            false
        } else {
            let_cxx_string!(form_spec = self.form_string());
            isPoisonedByFormSpec(&form_spec)
        }
    }

    pub fn has_charge(&self) -> bool {
        let_cxx_string!(form_spec = self.form_string());
        hasChargeByFormSpec(&form_spec)
    }

    /// Charge as a float from 0.0 to 1.0 inclusive. For enchanted weapons
    /// and torches or other fueled items.
    pub fn charge_level(&self) -> f32 {
        if self.is_armor() || self.is_weapon() || matches!(self.kind, BaseType::Light(_)) {
            let_cxx_string!(form_spec = self.form_string());
            chargeLevelByFormSpec(&form_spec)
        } else {
            0.0
        }
    }

    // We delegate everything to our object-kind. The goal is for most things
    // not to need to know about the item kind mess. Note that these functions
    // are all from the trait IsHudItem, which we can't implement here because
    // we offer these functions to the C++ side.
    pub fn count_matters(&self) -> bool {
        self.kind.count_matters()
    }

    pub fn is_ammo(&self) -> bool {
        self.kind.is_ammo()
    }

    pub fn is_armor(&self) -> bool {
        self.kind.is_armor()
    }

    pub fn is_magic(&self) -> bool {
        self.kind.is_magic()
    }

    pub fn is_potion(&self) -> bool {
        self.kind.is_potion()
    }

    pub fn is_power(&self) -> bool {
        self.kind.is_power()
    }

    pub fn is_spell(&self) -> bool {
        self.kind.is_spell()
    }

    pub fn is_utility(&self) -> bool {
        self.kind.is_utility()
    }

    pub fn is_weapon(&self) -> bool {
        self.kind.is_weapon()
    }

    pub fn is_one_handed(&self) -> bool {
        self.kind.is_one_handed()
    }

    pub fn left_hand_ok(&self) -> bool {
        self.kind.left_hand_ok()
    }

    pub fn right_hand_ok(&self) -> bool {
        self.kind.right_hand_ok()
    }

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

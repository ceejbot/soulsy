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
        todo!()
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

// ---------- EntryKind, a shared enum

/// We cannot derive default for shared enums, so we define it here.
impl Default for EntryKind {
    fn default() -> Self {
        EntryKind::IconDefault
    }
}

impl EntryKind {
    /// Check if this entry is a magic spell or scroll of any type.
    fn is_magic(&self) -> bool {
        matches!(
            *self,
            EntryKind::Alteration
                | EntryKind::Conjuration
                | EntryKind::Destruction
                | EntryKind::DestructionFire
                | EntryKind::DestructionFrost
                | EntryKind::DestructionShock
                | EntryKind::Illusion
                | EntryKind::Restoration
                | EntryKind::SpellDefault
                | EntryKind::Scroll
        )
    }

    /// Check if this entry is a weapon of any kind.
    fn is_weapon(&self) -> bool {
        matches!(
            *self,
            EntryKind::AxeOneHanded
                | EntryKind::AxeTwoHanded
                | EntryKind::Bow
                | EntryKind::Claw
                | EntryKind::Crossbow
                | EntryKind::Dagger
                | EntryKind::Halberd
                | EntryKind::HandToHand
                | EntryKind::Katana
                | EntryKind::Mace
                | EntryKind::Pike
                | EntryKind::QuarterStaff
                | EntryKind::Rapier
                | EntryKind::Staff
                | EntryKind::SwordOneHanded
                | EntryKind::Whip
        )
    }

    /// Check if this entry can be equipped in the left hand.
    ///
    /// Returns true for weapons, magic, lights, and shields.
    fn left_hand_ok(&self) -> bool {
        self.is_weapon()
            || self.is_magic()
            || matches!(
                *self,
                EntryKind::Shield | EntryKind::Torch | EntryKind::Lantern
            )
    }

    /// Check if this entry can be equipped in the right hand.
    ///
    /// Shields and torches are intentionally excluded.
    fn right_hand_ok(&self) -> bool {
        self.is_weapon() || self.is_magic()
    }

    /// Check if this entry is a shout or power. Fus-ro-dah!
    fn is_power(&self) -> bool {
        matches!(*self, EntryKind::Shout | EntryKind::Power)
    }

    /// Check if this entry is a utility item, aka the bottom slot.
    ///
    /// These are consumables like potions and food, armor items, and poisons.
    fn is_utility(&self) -> bool {
        matches!(
            *self,
            EntryKind::ArmorClothing
                | EntryKind::ArmorHeavy
                | EntryKind::ArmorLight
                | EntryKind::Food
                | EntryKind::Lantern
                | EntryKind::Mask
                | EntryKind::PoisonDefault
                | EntryKind::PotionDefault
                | EntryKind::PotionFireResist
                | EntryKind::PotionFrostResist
                | EntryKind::PotionHealth
                | EntryKind::PotionMagicka
                | EntryKind::PotionMagicResist
                | EntryKind::PotionShockResist
                | EntryKind::PotionStamina
        )
    }

    /// Get the filename of the icon to use for this entry kind.
    pub fn icon_file(&self) -> String {
        // I regret my life choices.
        match *self {
            EntryKind::Alteration => "alteration.svg".to_string(),
            EntryKind::ArmorClothing => "armor_clothing.svg".to_string(),
            EntryKind::ArmorHeavy => "armor_heavy.svg".to_string(),
            EntryKind::ArmorLight => "armor_light.svg".to_string(),
            EntryKind::Arrow => "arrow.svg".to_string(),
            EntryKind::AxeOneHanded => "axe_one_handed.svg".to_string(),
            EntryKind::AxeTwoHanded => "axe_two_handed.svg".to_string(),
            EntryKind::Bow => "bow.svg".to_string(),
            EntryKind::Claw => "claw.svg".to_string(),
            EntryKind::Conjuration => "conjuration.svg".to_string(),
            EntryKind::Crossbow => "crossbow.svg".to_string(),
            EntryKind::Dagger => "dagger.svg".to_string(),
            EntryKind::DestructionFire => "destruction_fire.svg".to_string(),
            EntryKind::DestructionFrost => "destruction_frost.svg".to_string(),
            EntryKind::DestructionShock => "destruction_shock.svg".to_string(),
            EntryKind::Destruction => "destruction.svg".to_string(),
            EntryKind::Food => "food.svg".to_string(),
            EntryKind::Halberd => "halberd.svg".to_string(),
            EntryKind::HandToHand => "hand_to_hand.svg".to_string(),
            EntryKind::IconDefault => "icon_default.svg".to_string(),
            EntryKind::Illusion => "illusion.svg".to_string(),
            EntryKind::Katana => "katana.svg".to_string(),
            EntryKind::Lantern => "lantern.svg".to_string(),
            EntryKind::Mace => "mace.svg".to_string(),
            EntryKind::Mask => "mask.svg".to_string(),
            EntryKind::Pike => "pike.svg".to_string(),
            EntryKind::PoisonDefault => "poison_default.svg".to_string(),
            EntryKind::PotionDefault => "default_potion.svg".to_string(),
            EntryKind::PotionFireResist => "potion_fire_resist.svg".to_string(),
            EntryKind::PotionFrostResist => "potion_frost_resist.svg".to_string(),
            EntryKind::PotionHealth => "potion_health.svg".to_string(),
            EntryKind::PotionMagicka => "potion_magicka.svg".to_string(),
            EntryKind::PotionMagicResist => "potion_magic_resist.svg".to_string(),
            EntryKind::PotionShockResist => "potion_shock_resist.svg".to_string(),
            EntryKind::PotionStamina => "potion_stamina.svg".to_string(),
            EntryKind::Power => "power.svg".to_string(),
            EntryKind::QuarterStaff => "quarterstaff.svg".to_string(),
            EntryKind::Rapier => "rapier.svg".to_string(),
            EntryKind::Restoration => "restoration.svg".to_string(),
            EntryKind::Scroll => "scroll.svg".to_string(),
            EntryKind::Shield => "shield.svg".to_string(),
            EntryKind::Shout => "shout.svg".to_string(),
            EntryKind::SpellDefault => "spell_default.svg".to_string(),
            EntryKind::Staff => "staff.svg".to_string(),
            EntryKind::SwordOneHanded => "sword_one_handed.svg".to_string(),
            EntryKind::SwordTwoHanded => "sword_two_handed.svg".to_string(),
            EntryKind::Torch => "torch.svg".to_string(),
            EntryKind::WeaponDefault => "sword_one_handed.svg".to_string(),
            EntryKind::Whip => "whip.svg".to_string(),
            _ => "default_icon.svg".to_string(),
        }
    }
}

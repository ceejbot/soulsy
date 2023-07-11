use std::path::PathBuf;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::user_settings;
use crate::plugin::{Action, EntryIcon, MenuEventResponse};

// We cannot derive default for shared enums.
impl Default for EntryIcon {
    fn default() -> Self {
        EntryIcon::IconDefault
    }
}

impl EntryIcon {
    fn is_magic(&self) -> bool {
        matches!(
            *self,
            EntryIcon::Alteration
                | EntryIcon::Conjuration
                | EntryIcon::Destruction
                | EntryIcon::DestructionFire
                | EntryIcon::DestructionFrost
                | EntryIcon::DestructionShock
                | EntryIcon::Illusion
                | EntryIcon::Restoration
                | EntryIcon::SpellDefault
                | EntryIcon::Scroll
        )
    }

    fn is_weapon(&self) -> bool {
        matches!(
            *self,
            EntryIcon::AxeOneHanded
                | EntryIcon::AxeTwoHanded
                | EntryIcon::Bow
                | EntryIcon::Claw
                | EntryIcon::Crossbow
                | EntryIcon::Dagger
                | EntryIcon::Halberd
                | EntryIcon::HandToHand
                | EntryIcon::Katana
                | EntryIcon::Mace
                | EntryIcon::Pike
                | EntryIcon::QuarterStaff
                | EntryIcon::Rapier
                | EntryIcon::Staff
                | EntryIcon::SwordOneHanded
                | EntryIcon::Whip
        )
    }

    fn left_hand_ok(&self) -> bool {
        self.is_weapon() || self.is_magic() || matches!(*self, EntryIcon::Shield | EntryIcon::Torch)
    }

    fn right_hand_ok(&self) -> bool {
        self.is_weapon() || self.is_magic()
    }

    fn is_power(&self) -> bool {
        matches!(*self, EntryIcon::Shout | EntryIcon::Power)
    }

    fn is_utility(&self) -> bool {
        matches!(
            *self,
            EntryIcon::ArmorClothing
                | EntryIcon::ArmorHeavy
                | EntryIcon::ArmorLight
                | EntryIcon::DefaultPotion
                | EntryIcon::Food
                | EntryIcon::PotionFireResist
                | EntryIcon::PoisonDefault
                | EntryIcon::PotionFrostResist
                | EntryIcon::PotionHealth
                | EntryIcon::PotionMagicka
                | EntryIcon::PotionShockResist
                | EntryIcon::PotionStamina
        )
    }
}

// Haven't yet figured out how to serialize this to toml or anything yet.
// Still working on what data I want to track.
#[derive(Deserialize, Serialize, Debug, Clone, Default, Eq)]
pub struct CycleEntry {
    /// A string that can be turned back into form data; for serializing.
    form_string: String,
    /// An enum classifying this item for fast question-answering. Equiv to `type` from TESForm.
    kind: EntryIcon,
    /// True if this item requires both hands to use.
    two_handed: bool,
    /// True if this item should be displayed with count data.
    has_count: bool,
    /// Cached count from inventory data. Relies on hooks to be updated.
    count: usize,
}

// Testing the formstring is sufficient for our needs, which is figuring out if
// this form item is in a cycle already.
impl PartialEq for CycleEntry {
    fn eq(&self, other: &Self) -> bool {
        self.form_string == other.form_string
    }
}

pub fn create_cycle_entry(
    kind: EntryIcon,
    two_handed: bool,
    has_count: bool,
    count: usize,
    form_string: &str,
) -> Box<CycleEntry> {
    Box::new(CycleEntry::new(
        kind,
        two_handed,
        has_count,
        count,
        form_string,
    ))
}

impl CycleEntry {
    /// This is called from C++ when handing us a new item.
    pub fn new(
        kind: EntryIcon,
        two_handed: bool,
        has_count: bool,
        count: usize,
        form_string: &str,
    ) -> Self {
        CycleEntry {
            form_string: form_string.to_string(),
            kind,
            two_handed,
            has_count,
            count,
        }
    }

    pub fn icon_file(&self) -> String {
        // I regret my life choices.
        match self.kind {
            EntryIcon::Alteration => "alteration.svg".to_string(),
            EntryIcon::ArmorClothing => "armor_clothing.svg".to_string(),
            EntryIcon::ArmorHeavy => "armor_heavy.svg".to_string(),
            EntryIcon::ArmorLight => "armor_light.svg".to_string(),
            EntryIcon::Arrow => "arrow.svg".to_string(),
            EntryIcon::AxeOneHanded => "axe_one_handed.svg".to_string(),
            EntryIcon::AxeTwoHanded => "axe_two_handed.svg".to_string(),
            EntryIcon::Bow => "bow.svg".to_string(),
            EntryIcon::Claw => "claw.svg".to_string(),
            EntryIcon::Conjuration => "conjuration.svg".to_string(),
            EntryIcon::Crossbow => "crossbow.svg".to_string(),
            EntryIcon::Dagger => "dagger.svg".to_string(),
            EntryIcon::DefaultPotion => "default_potion.svg".to_string(),
            EntryIcon::DestructionFire => "destruction_fire.svg".to_string(),
            EntryIcon::DestructionFrost => "destruction_frost.svg".to_string(),
            EntryIcon::DestructionShock => "destruction_shock.svg".to_string(),
            EntryIcon::Destruction => "destruction.svg".to_string(),
            EntryIcon::Food => "food.svg".to_string(),
            EntryIcon::Halberd => "halberd.svg".to_string(),
            EntryIcon::HandToHand => "hand_to_hand.svg".to_string(),
            EntryIcon::IconDefault => "icon_default.svg".to_string(),
            EntryIcon::Illusion => "illusion.svg".to_string(),
            EntryIcon::Katana => "katana.svg".to_string(),
            EntryIcon::Mace => "mace.svg".to_string(),
            EntryIcon::Pike => "pike.svg".to_string(),
            EntryIcon::PoisonDefault => "poison_default.svg".to_string(),
            EntryIcon::PotionFireResist => "potion_fire_resist.svg".to_string(),
            EntryIcon::PotionFrostResist => "potion_frost_resist.svg".to_string(),
            EntryIcon::PotionHealth => "potion_health.svg".to_string(),
            EntryIcon::PotionMagicka => "potion_magicka.svg".to_string(),
            EntryIcon::PotionShockResist => "potion_shock_resist.svg".to_string(),
            EntryIcon::PotionStamina => "potion_stamina.svg".to_string(),
            EntryIcon::Power => "power.svg".to_string(),
            EntryIcon::QuarterStaff => "quarterStaff.svg".to_string(),
            EntryIcon::Rapier => "rapier.svg".to_string(),
            EntryIcon::Restoration => "restoration.svg".to_string(),
            EntryIcon::Scroll => "scroll.svg".to_string(),
            EntryIcon::Shield => "shield.svg".to_string(),
            EntryIcon::Shout => "shout.svg".to_string(),
            EntryIcon::SpellDefault => "spell_default.svg".to_string(),
            EntryIcon::Staff => "staff.svg".to_string(),
            EntryIcon::SwordOneHanded => "sword_one_handed.svg".to_string(),
            EntryIcon::SwordTwoHanded => "sword_two_handed.svg".to_string(),
            EntryIcon::Torch => "torch.svg".to_string(),
            EntryIcon::Whip => "whip.svg".to_string(),
            _ => "default_icon.svg".to_string(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct CycleData {
    left: Vec<CycleEntry>,
    right: Vec<CycleEntry>,
    power: Vec<CycleEntry>,
    utility: Vec<CycleEntry>,
}

static CYCLE_PATH: &str = "./data/SKSE/Plugins/SoulsyHUD_Cycles.toml";

impl CycleData {
    pub fn write(&self) -> Result<()> {
        let buf = toml::to_string(self)?;
        std::fs::write(CYCLE_PATH, buf)?;
        Ok(())
    }

    pub fn read() -> Result<Self> {
        let buf = std::fs::read_to_string(PathBuf::from(CYCLE_PATH))?;
        let layout = toml::from_str::<CycleData>(&buf)?;
        Ok(layout)
    }

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
        cycle.rotate_left(amount);
        cycle.first().cloned()
    }

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

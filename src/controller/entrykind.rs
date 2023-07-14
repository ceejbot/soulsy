//! Trait implementations and utilities for the shared enum EntryKind.
use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::plugin::EntryKind;

/// We cannot derive default for shared enums, so we define it here.
impl Default for EntryKind {
    fn default() -> Self {
        EntryKind::Empty
    }
}

impl EntryKind {
    /// Get the filename of the icon to use for this entry kind.
    pub fn icon_file(&self) -> String {
        if let Some(i) = ICON_MAP.get(self) {
            i.to_string()
        } else {
            "default_icon.svg".to_string()
        }
    }

    /// Check if this entry is a magic spell or scroll of any type.
    pub fn is_magic(&self) -> bool {
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
    pub fn is_weapon(&self) -> bool {
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

    /// Check if this item is wearable armor.
    pub fn is_armor(&self) -> bool {
        matches!(
            *self,
            EntryKind::ArmorClothing
                | EntryKind::ArmorHeavy
                | EntryKind::ArmorLight
                | EntryKind::Lantern
                | EntryKind::Mask
        )
    }

    /// Check if this entry can be equipped in the left hand.
    ///
    /// Returns true for weapons, magic, lights, and shields.
    pub fn left_hand_ok(&self) -> bool {
        self.is_weapon() || self.is_magic() || matches!(*self, EntryKind::Shield | EntryKind::Torch)
    }

    /// Check if this entry can be equipped in the right hand.
    ///
    /// Shields and torches are intentionally excluded.
    pub fn right_hand_ok(&self) -> bool {
        self.is_weapon() || self.is_magic()
    }

    /// Check if this entry is a shout or power. Fus-ro-dah!
    pub fn is_power(&self) -> bool {
        matches!(*self, EntryKind::Shout | EntryKind::Power)
    }

    /// Check if this entry is a utility item, aka the bottom slot.
    ///
    /// These are consumables like potions and food, armor items, and poisons.
    pub fn is_utility(&self) -> bool {
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
}

static ICON_MAP: Lazy<HashMap<EntryKind, String>> = Lazy::new(|| {
    // TODO EntryKind::Empty
    HashMap::from([
        (EntryKind::Alteration, "alteration.svg".to_string()),
        (EntryKind::ArmorClothing, "armor_clothing.svg".to_string()),
        (EntryKind::ArmorHeavy, "armor_heavy.svg".to_string()),
        (EntryKind::ArmorLight, "armor_light.svg".to_string()),
        (EntryKind::Arrow, "arrow.svg".to_string()),
        (EntryKind::AxeOneHanded, "axe_one_handed.svg".to_string()),
        (EntryKind::AxeTwoHanded, "axe_two_handed.svg".to_string()),
        (EntryKind::Bow, "bow.svg".to_string()),
        (EntryKind::Claw, "claw.svg".to_string()),
        (EntryKind::Conjuration, "conjuration.svg".to_string()),
        (EntryKind::Crossbow, "crossbow.svg".to_string()),
        (EntryKind::Dagger, "dagger.svg".to_string()),
        (
            EntryKind::DestructionFire,
            "destruction_fire.svg".to_string(),
        ),
        (
            EntryKind::DestructionFrost,
            "destruction_frost.svg".to_string(),
        ),
        (
            EntryKind::DestructionShock,
            "destruction_shock.svg".to_string(),
        ),
        (EntryKind::Destruction, "destruction.svg".to_string()),
        (EntryKind::Food, "food.svg".to_string()),
        (EntryKind::Halberd, "halberd.svg".to_string()),
        (EntryKind::HandToHand, "hand_to_hand.svg".to_string()),
        (EntryKind::IconDefault, "icon_default.svg".to_string()),
        (EntryKind::Illusion, "illusion.svg".to_string()),
        (EntryKind::Katana, "katana.svg".to_string()),
        (EntryKind::Lantern, "lantern.svg".to_string()),
        (EntryKind::Mace, "mace.svg".to_string()),
        (EntryKind::Mask, "mask.svg".to_string()),
        (EntryKind::Pike, "pike.svg".to_string()),
        (EntryKind::PoisonDefault, "poison_default.svg".to_string()),
        (EntryKind::PotionDefault, "default_potion.svg".to_string()),
        (
            EntryKind::PotionFireResist,
            "potion_fire_resist.svg".to_string(),
        ),
        (
            EntryKind::PotionFrostResist,
            "potion_frost_resist.svg".to_string(),
        ),
        (EntryKind::PotionHealth, "potion_health.svg".to_string()),
        (EntryKind::PotionMagicka, "potion_magicka.svg".to_string()),
        (
            EntryKind::PotionMagicResist,
            "potion_magic_resist.svg".to_string(),
        ),
        (
            EntryKind::PotionShockResist,
            "potion_shock_resist.svg".to_string(),
        ),
        (EntryKind::PotionStamina, "potion_stamina.svg".to_string()),
        (EntryKind::Power, "power.svg".to_string()),
        (EntryKind::QuarterStaff, "quarterstaff.svg".to_string()),
        (EntryKind::Rapier, "rapier.svg".to_string()),
        (EntryKind::Restoration, "restoration.svg".to_string()),
        (EntryKind::Scroll, "scroll.svg".to_string()),
        (EntryKind::Shield, "shield.svg".to_string()),
        (EntryKind::Shout, "shout.svg".to_string()),
        (EntryKind::SpellDefault, "spell_default.svg".to_string()),
        (EntryKind::Staff, "staff.svg".to_string()),
        (
            EntryKind::SwordOneHanded,
            "sword_one_handed.svg".to_string(),
        ),
        (
            EntryKind::SwordTwoHanded,
            "sword_two_handed.svg".to_string(),
        ),
        (EntryKind::Torch, "torch.svg".to_string()),
        (EntryKind::WeaponDefault, "sword_one_handed.svg".to_string()),
        (EntryKind::Whip, "whip.svg".to_string()),
    ])
});

// Sketching out what moving image data management to rust would look like.
// - we need to rasterize svgs
// - we need to return structs with Vec<u8>, width, and height when requested
// - there are additional images beyond just entry icons (hotkey images, hud bg, etc)
// - we might want to lazy-load and cache but implement the stupid version first

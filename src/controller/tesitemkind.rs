//! Trait implementations and utilities for the shared enum TesItemKind.
use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::plugin::TesItemKind;

pub fn kind_has_count(kind: TesItemKind) -> bool {
    kind.show_count()
}
pub fn kind_is_magic(kind: TesItemKind) -> bool {
    kind.is_magic()
}

/// We cannot derive default for shared enums, so we define it here.
impl Default for TesItemKind {
    fn default() -> Self {
        TesItemKind::Empty
    }
}

impl TesItemKind {
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
            TesItemKind::Alteration
                | TesItemKind::Conjuration
                | TesItemKind::Destruction
                | TesItemKind::DestructionFire
                | TesItemKind::DestructionFrost
                | TesItemKind::DestructionShock
                | TesItemKind::Illusion
                | TesItemKind::Restoration
                | TesItemKind::SpellDefault
                | TesItemKind::Scroll
        )
    }

    pub fn show_count(&self) -> bool {
        matches!(
            *self,
            TesItemKind::PoisonDefault
                | TesItemKind::PotionDefault
                | TesItemKind::PotionFireResist
                | TesItemKind::PotionFrostResist
                | TesItemKind::PotionHealth
                | TesItemKind::PotionMagicka
                | TesItemKind::PotionMagicResist
                | TesItemKind::PotionShockResist
                | TesItemKind::PotionStamina
                | TesItemKind::Scroll
                | TesItemKind::Arrow
        )
    }

    /// Check if this entry is a weapon of any kind.
    pub fn is_weapon(&self) -> bool {
        matches!(
            *self,
            TesItemKind::AxeOneHanded
                | TesItemKind::AxeTwoHanded
                | TesItemKind::Bow
                | TesItemKind::Claw
                | TesItemKind::Crossbow
                | TesItemKind::Dagger
                | TesItemKind::Halberd
                | TesItemKind::HandToHand
                | TesItemKind::Katana
                | TesItemKind::Mace
                | TesItemKind::Pike
                | TesItemKind::QuarterStaff
                | TesItemKind::Rapier
                | TesItemKind::Staff
                | TesItemKind::SwordOneHanded
                | TesItemKind::Whip
        )
    }

    /// Check if this item is wearable armor.
    pub fn is_armor(&self) -> bool {
        matches!(
            *self,
            TesItemKind::ArmorClothing
                | TesItemKind::ArmorHeavy
                | TesItemKind::ArmorLight
                | TesItemKind::Lantern
                | TesItemKind::Mask
        )
    }

    // TesItemKind::PoisonDefault,
    /// Check if this item is gluggable.
    pub fn is_potion(&self) -> bool {
        matches!(
            *self,
            TesItemKind::PotionDefault
                | TesItemKind::PotionFireResist
                | TesItemKind::PotionFrostResist
                | TesItemKind::PotionHealth
                | TesItemKind::PotionMagicka
                | TesItemKind::PotionMagicResist
                | TesItemKind::PotionShockResist
                | TesItemKind::PotionStamina
        )
    }

    /// Check if this entry can be equipped in the left hand.
    ///
    /// Returns true for weapons, magic, lights, and shields.
    pub fn left_hand_ok(&self) -> bool {
        self.is_weapon()
            || self.is_magic()
            || matches!(*self, TesItemKind::Shield | TesItemKind::Torch)
    }

    /// Check if this entry can be equipped in the right hand.
    ///
    /// Shields and torches are intentionally excluded.
    pub fn right_hand_ok(&self) -> bool {
        self.is_weapon() || self.is_magic()
    }

    /// Check if this entry is a shout or power. Fus-ro-dah!
    pub fn is_power(&self) -> bool {
        matches!(*self, TesItemKind::Shout | TesItemKind::Power)
    }

    /// Check if this entry is a utility item, aka the bottom slot.
    ///
    /// These are consumables like potions and food, armor items, and poisons.
    pub fn is_utility(&self) -> bool {
        matches!(
            *self,
            TesItemKind::ArmorClothing
                | TesItemKind::ArmorHeavy
                | TesItemKind::ArmorLight
                | TesItemKind::Food
                | TesItemKind::Lantern
                | TesItemKind::Mask
                | TesItemKind::PoisonDefault
                | TesItemKind::PotionDefault
                | TesItemKind::PotionFireResist
                | TesItemKind::PotionFrostResist
                | TesItemKind::PotionHealth
                | TesItemKind::PotionMagicka
                | TesItemKind::PotionMagicResist
                | TesItemKind::PotionShockResist
                | TesItemKind::PotionStamina
        )
    }
}

static ICON_MAP: Lazy<HashMap<TesItemKind, String>> = Lazy::new(|| {
    // TODO TesItemKind::Empty
    HashMap::from([
        (TesItemKind::Alteration, "alteration.svg".to_string()),
        (TesItemKind::ArmorClothing, "armor_clothing.svg".to_string()),
        (TesItemKind::ArmorHeavy, "armor_heavy.svg".to_string()),
        (TesItemKind::ArmorLight, "armor_light.svg".to_string()),
        (TesItemKind::Arrow, "arrow.svg".to_string()),
        (TesItemKind::AxeOneHanded, "axe_one_handed.svg".to_string()),
        (TesItemKind::AxeTwoHanded, "axe_two_handed.svg".to_string()),
        (TesItemKind::Bow, "bow.svg".to_string()),
        (TesItemKind::Claw, "claw.svg".to_string()),
        (TesItemKind::Conjuration, "conjuration.svg".to_string()),
        (TesItemKind::Crossbow, "crossbow.svg".to_string()),
        (TesItemKind::Dagger, "dagger.svg".to_string()),
        (
            TesItemKind::DestructionFire,
            "destruction_fire.svg".to_string(),
        ),
        (
            TesItemKind::DestructionFrost,
            "destruction_frost.svg".to_string(),
        ),
        (
            TesItemKind::DestructionShock,
            "destruction_shock.svg".to_string(),
        ),
        (TesItemKind::Destruction, "destruction.svg".to_string()),
        (TesItemKind::Food, "food.svg".to_string()),
        (TesItemKind::Halberd, "halberd.svg".to_string()),
        (TesItemKind::HandToHand, "hand_to_hand.svg".to_string()),
        (TesItemKind::IconDefault, "icon_default.svg".to_string()),
        (TesItemKind::Illusion, "illusion.svg".to_string()),
        (TesItemKind::Katana, "katana.svg".to_string()),
        (TesItemKind::Lantern, "lantern.svg".to_string()),
        (TesItemKind::Mace, "mace.svg".to_string()),
        (TesItemKind::Mask, "mask.svg".to_string()),
        (TesItemKind::Pike, "pike.svg".to_string()),
        (TesItemKind::PoisonDefault, "poison_default.svg".to_string()),
        (TesItemKind::PotionDefault, "default_potion.svg".to_string()),
        (
            TesItemKind::PotionFireResist,
            "potion_fire_resist.svg".to_string(),
        ),
        (
            TesItemKind::PotionFrostResist,
            "potion_frost_resist.svg".to_string(),
        ),
        (TesItemKind::PotionHealth, "potion_health.svg".to_string()),
        (TesItemKind::PotionMagicka, "potion_magicka.svg".to_string()),
        (
            TesItemKind::PotionMagicResist,
            "potion_magic_resist.svg".to_string(),
        ),
        (
            TesItemKind::PotionShockResist,
            "potion_shock_resist.svg".to_string(),
        ),
        (TesItemKind::PotionStamina, "potion_stamina.svg".to_string()),
        (TesItemKind::Power, "power.svg".to_string()),
        (TesItemKind::QuarterStaff, "quarter_staff.svg".to_string()),
        (TesItemKind::Rapier, "rapier.svg".to_string()),
        (TesItemKind::Restoration, "restoration.svg".to_string()),
        (TesItemKind::Scroll, "scroll.svg".to_string()),
        (TesItemKind::Shield, "shield.svg".to_string()),
        (TesItemKind::Shout, "shout.svg".to_string()),
        (TesItemKind::SpellDefault, "spell_default.svg".to_string()),
        (TesItemKind::Staff, "staff.svg".to_string()),
        (
            TesItemKind::SwordOneHanded,
            "sword_one_handed.svg".to_string(),
        ),
        (
            TesItemKind::SwordTwoHanded,
            "sword_two_handed.svg".to_string(),
        ),
        (TesItemKind::Torch, "torch.svg".to_string()),
        (
            TesItemKind::WeaponDefault,
            "sword_one_handed.svg".to_string(),
        ),
        (TesItemKind::Whip, "whip.svg".to_string()),
    ])
});

// Sketching out what moving image data management to rust would look like.
// - we need to rasterize svgs
// - we need to return structs with Vec<u8>, width, and height when requested
// - there are additional images beyond just entry icons (hotkey images, hud bg, etc)
// - we might want to lazy-load and cache but implement the stupid version first

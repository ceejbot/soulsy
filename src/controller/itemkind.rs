//! Trait implementations and utilities for the shared enum TesItemKind.
use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::plugin::ItemKind;

/// Given an entry kind, return the filename of the icon to use for it.
/// Exposed to C++.
pub fn get_icon_file(kind: &ItemKind) -> String {
    kind.icon_file()
}

pub fn kind_has_count(kind: ItemKind) -> bool {
    kind.show_count()
}

pub fn kind_is_magic(kind: ItemKind) -> bool {
    kind.is_magic()
}

/// We cannot derive default for shared enums, so we define it here.
impl Default for ItemKind {
    fn default() -> Self {
        ItemKind::NotFound
    }
}

impl ItemKind {
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
            ItemKind::Alteration
                | ItemKind::Conjuration
                | ItemKind::Destruction
                | ItemKind::DestructionFire
                | ItemKind::DestructionFrost
                | ItemKind::DestructionShock
                | ItemKind::Illusion
                | ItemKind::Restoration
                | ItemKind::SpellDefault
                | ItemKind::Scroll
        )
    }

    pub fn show_count(&self) -> bool {
        self.is_armor()
            || self.is_weapon()
            || self.is_potion()
            || matches!(*self, ItemKind::Arrow | ItemKind::Scroll)
    }

    /// Check if this entry is a weapon of any kind.
    pub fn is_weapon(&self) -> bool {
        matches!(
            *self,
            ItemKind::AxeOneHanded
                | ItemKind::AxeTwoHanded
                | ItemKind::Bow
                | ItemKind::Claw
                | ItemKind::Crossbow
                | ItemKind::Dagger
                | ItemKind::Halberd
                | ItemKind::HandToHand
                | ItemKind::Katana
                | ItemKind::Mace
                | ItemKind::Pike
                | ItemKind::QuarterStaff
                | ItemKind::Rapier
                | ItemKind::Staff
                | ItemKind::SwordOneHanded
                | ItemKind::SwordTwoHanded
                | ItemKind::Whip
        )
    }

    pub fn is_one_handed_weapon(&self) -> bool {
        matches!(
            *self,
            ItemKind::AxeOneHanded
                | ItemKind::Claw
                | ItemKind::Dagger
                | ItemKind::HandToHand
                | ItemKind::Katana
                | ItemKind::Mace
                | ItemKind::Rapier
                | ItemKind::SwordOneHanded
                | ItemKind::Whip
        )
    }

    /// Check if this item is wearable armor.
    pub fn is_armor(&self) -> bool {
        matches!(
            *self,
            ItemKind::ArmorClothing
                | ItemKind::ArmorHeavy
                | ItemKind::ArmorLight
                | ItemKind::Lantern
                | ItemKind::Mask
        )
    }

    // TesItemKind::PoisonDefault,
    /// Check if this item is gluggable.
    pub fn is_potion(&self) -> bool {
        matches!(
            *self,
            ItemKind::PotionDefault
                | ItemKind::PotionFireResist
                | ItemKind::PotionFrostResist
                | ItemKind::PotionHealth
                | ItemKind::PotionMagicka
                | ItemKind::PotionMagicResist
                | ItemKind::PotionShockResist
                | ItemKind::PotionStamina
        )
    }

    /// Check if this entry can be equipped in the left hand.
    ///
    /// Returns true for weapons, magic, lights, and shields.
    pub fn left_hand_ok(&self) -> bool {
        self.is_one_handed_weapon()
            || self.is_magic()
            || matches!(*self, ItemKind::Shield | ItemKind::Torch)
    }

    /// Check if this entry can be equipped in the right hand.
    ///
    /// Shields and torches are intentionally excluded.
    pub fn right_hand_ok(&self) -> bool {
        self.is_weapon() || self.is_magic()
    }

    /// Check if this entry is a shout or power. Fus-ro-dah!
    pub fn is_power(&self) -> bool {
        matches!(*self, ItemKind::Shout | ItemKind::Power)
    }

    /// Check if this entry is a kind of ammo.
    pub fn is_ammo(&self) -> bool {
        matches!(*self, ItemKind::Arrow)
    }

    /// Check if this entry is a utility item, aka the bottom slot.
    ///
    /// These are consumables like potions and food, armor items, and poisons.
    pub fn is_utility(&self) -> bool {
        matches!(
            *self,
            ItemKind::Arrow
                | ItemKind::ArmorClothing
                | ItemKind::ArmorHeavy
                | ItemKind::ArmorLight
                | ItemKind::Food
                | ItemKind::Lantern
                | ItemKind::Mask
                | ItemKind::PoisonDefault
                | ItemKind::PotionDefault
                | ItemKind::PotionFireResist
                | ItemKind::PotionFrostResist
                | ItemKind::PotionHealth
                | ItemKind::PotionMagicka
                | ItemKind::PotionMagicResist
                | ItemKind::PotionShockResist
                | ItemKind::PotionStamina
        )
    }
}

static ICON_MAP: Lazy<HashMap<ItemKind, String>> = Lazy::new(|| {
    HashMap::from([
        (ItemKind::Alteration, "alteration.svg".to_string()),
        (ItemKind::ArmorClothing, "armor_clothing.svg".to_string()),
        (ItemKind::ArmorHeavy, "armor_heavy.svg".to_string()),
        (ItemKind::ArmorLight, "armor_light.svg".to_string()),
        (ItemKind::Arrow, "arrow.svg".to_string()),
        (ItemKind::AxeOneHanded, "axe_one_handed.svg".to_string()),
        (ItemKind::AxeTwoHanded, "axe_two_handed.svg".to_string()),
        (ItemKind::Bow, "bow.svg".to_string()),
        (ItemKind::Claw, "claw.svg".to_string()),
        (ItemKind::Conjuration, "conjuration.svg".to_string()),
        (ItemKind::Crossbow, "crossbow.svg".to_string()),
        (ItemKind::Dagger, "dagger.svg".to_string()),
        (
            ItemKind::DestructionFire,
            "destruction_fire.svg".to_string(),
        ),
        (
            ItemKind::DestructionFrost,
            "destruction_frost.svg".to_string(),
        ),
        (
            ItemKind::DestructionShock,
            "destruction_shock.svg".to_string(),
        ),
        (ItemKind::Destruction, "destruction.svg".to_string()),
        (ItemKind::Food, "food.svg".to_string()),
        (ItemKind::Halberd, "halberd.svg".to_string()),
        (ItemKind::HandToHand, "hand_to_hand.svg".to_string()),
        (ItemKind::IconDefault, "icon_default.svg".to_string()),
        (ItemKind::Illusion, "illusion.svg".to_string()),
        (ItemKind::Katana, "katana.svg".to_string()),
        (ItemKind::Lantern, "lantern.svg".to_string()),
        (ItemKind::Mace, "mace.svg".to_string()),
        (ItemKind::Mask, "mask.svg".to_string()),
        (ItemKind::Pike, "pike.svg".to_string()),
        (ItemKind::PoisonDefault, "poison_default.svg".to_string()),
        (ItemKind::PotionDefault, "default_potion.svg".to_string()),
        (
            ItemKind::PotionFireResist,
            "potion_fire_resist.svg".to_string(),
        ),
        (
            ItemKind::PotionFrostResist,
            "potion_frost_resist.svg".to_string(),
        ),
        (ItemKind::PotionHealth, "potion_health.svg".to_string()),
        (ItemKind::PotionMagicka, "potion_magicka.svg".to_string()),
        (
            ItemKind::PotionMagicResist,
            "potion_magic_resist.svg".to_string(),
        ),
        (
            ItemKind::PotionShockResist,
            "potion_shock_resist.svg".to_string(),
        ),
        (ItemKind::PotionStamina, "potion_stamina.svg".to_string()),
        (ItemKind::Power, "power.svg".to_string()),
        (ItemKind::QuarterStaff, "quarter_staff.svg".to_string()),
        (ItemKind::Rapier, "rapier.svg".to_string()),
        (ItemKind::Restoration, "restoration.svg".to_string()),
        (ItemKind::Scroll, "scroll.svg".to_string()),
        (ItemKind::Shield, "shield.svg".to_string()),
        (ItemKind::Shout, "shout.svg".to_string()),
        (ItemKind::SpellDefault, "spell_default.svg".to_string()),
        (ItemKind::Staff, "staff.svg".to_string()),
        (ItemKind::SwordOneHanded, "sword_one_handed.svg".to_string()),
        (ItemKind::SwordTwoHanded, "sword_two_handed.svg".to_string()),
        (ItemKind::Torch, "torch.svg".to_string()),
        (ItemKind::WeaponDefault, "sword_one_handed.svg".to_string()),
        (ItemKind::Whip, "whip.svg".to_string()),
    ])
});

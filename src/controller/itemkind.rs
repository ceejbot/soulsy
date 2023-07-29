//! Trait implementations and utilities for the shared enum ItemKind.
use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::plugin::ItemKind;

/// Given an entry kind, return the filename of the icon to use for it.
/// Exposed to C++.
pub fn get_icon_file(kind: &ItemKind) -> String {
    kind.icon_file()
}

pub fn kind_has_count(kind: ItemKind) -> bool {
    kind.count_matters()
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
        self.is_spell() || matches!(*self, ItemKind::Scroll)
    }

    /// is a spell specifically, not a scroll
    pub fn is_spell(&self) -> bool {
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
        )
    }

    pub fn count_matters(&self) -> bool {
        !matches!(*self, ItemKind::HandToHand)
            && (self.is_weapon()
                || self.is_potion()
                || self.is_ammo()
                || matches!(*self, ItemKind::Scroll))
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

    // ItemKind::PoisonDefault,
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

// This is horrific. There has to be a better way.
// A way to iterate, perhaps?
impl From<u8> for ItemKind {
    fn from(value: u8) -> Self {
        if value == ItemKind::Empty.repr {
            ItemKind::Empty
        } else if value == ItemKind::Alteration.repr {
            ItemKind::Alteration
        } else if value == ItemKind::ArmorClothing.repr {
            ItemKind::ArmorClothing
        } else if value == ItemKind::ArmorHeavy.repr {
            ItemKind::ArmorHeavy
        } else if value == ItemKind::ArmorLight.repr {
            ItemKind::ArmorLight
        } else if value == ItemKind::Arrow.repr {
            ItemKind::Arrow
        } else if value == ItemKind::AxeOneHanded.repr {
            ItemKind::AxeOneHanded
        } else if value == ItemKind::AxeTwoHanded.repr {
            ItemKind::AxeTwoHanded
        } else if value == ItemKind::Bow.repr {
            ItemKind::Bow
        } else if value == ItemKind::Claw.repr {
            ItemKind::Claw
        } else if value == ItemKind::Conjuration.repr {
            ItemKind::Conjuration
        } else if value == ItemKind::Crossbow.repr {
            ItemKind::Crossbow
        } else if value == ItemKind::Dagger.repr {
            ItemKind::Dagger
        } else if value == ItemKind::DestructionFire.repr {
            ItemKind::DestructionFire
        } else if value == ItemKind::DestructionFrost.repr {
            ItemKind::DestructionFrost
        } else if value == ItemKind::DestructionShock.repr {
            ItemKind::DestructionShock
        } else if value == ItemKind::Destruction.repr {
            ItemKind::Destruction
        } else if value == ItemKind::Food.repr {
            ItemKind::Food
        } else if value == ItemKind::Halberd.repr {
            ItemKind::Halberd
        } else if value == ItemKind::HandToHand.repr {
            ItemKind::HandToHand
        } else if value == ItemKind::IconDefault.repr {
            ItemKind::IconDefault
        } else if value == ItemKind::Illusion.repr {
            ItemKind::Illusion
        } else if value == ItemKind::Katana.repr {
            ItemKind::Katana
        } else if value == ItemKind::Lantern.repr {
            ItemKind::Lantern
        } else if value == ItemKind::Mace.repr {
            ItemKind::Mace
        } else if value == ItemKind::Mask.repr {
            ItemKind::Mask
        } else if value == ItemKind::Pike.repr {
            ItemKind::Pike
        } else if value == ItemKind::PoisonDefault.repr {
            ItemKind::PoisonDefault
        } else if value == ItemKind::PotionDefault.repr {
            ItemKind::PotionDefault
        } else if value == ItemKind::PotionFireResist.repr {
            ItemKind::PotionFireResist
        } else if value == ItemKind::PotionFrostResist.repr {
            ItemKind::PotionFrostResist
        } else if value == ItemKind::PotionHealth.repr {
            ItemKind::PotionHealth
        } else if value == ItemKind::PotionMagicka.repr {
            ItemKind::PotionMagicka
        } else if value == ItemKind::PotionMagicResist.repr {
            ItemKind::PotionMagicResist
        } else if value == ItemKind::PotionShockResist.repr {
            ItemKind::PotionShockResist
        } else if value == ItemKind::PotionStamina.repr {
            ItemKind::PotionStamina
        } else if value == ItemKind::Power.repr {
            ItemKind::Power
        } else if value == ItemKind::QuarterStaff.repr {
            ItemKind::QuarterStaff
        } else if value == ItemKind::Rapier.repr {
            ItemKind::Rapier
        } else if value == ItemKind::Restoration.repr {
            ItemKind::Restoration
        } else if value == ItemKind::Scroll.repr {
            ItemKind::Scroll
        } else if value == ItemKind::Shield.repr {
            ItemKind::Shield
        } else if value == ItemKind::Shout.repr {
            ItemKind::Shout
        } else if value == ItemKind::SpellDefault.repr {
            ItemKind::SpellDefault
        } else if value == ItemKind::Staff.repr {
            ItemKind::Staff
        } else if value == ItemKind::SwordOneHanded.repr {
            ItemKind::SwordOneHanded
        } else if value == ItemKind::SwordTwoHanded.repr {
            ItemKind::SwordTwoHanded
        } else if value == ItemKind::Torch.repr {
            ItemKind::Torch
        } else if value == ItemKind::WeaponDefault.repr {
            ItemKind::WeaponDefault
        } else if value == ItemKind::Whip.repr {
            ItemKind::Whip
        } else {
            ItemKind::Empty
        }
    }
}

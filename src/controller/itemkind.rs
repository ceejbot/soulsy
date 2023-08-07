//! Trait implementations and utilities for the shared enum ItemKind.
use std::collections::HashMap;

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

/// The type of an item stored in a cycle.
///
/// This lets us determine the icon as well as which cycle slot an item can
/// be added to.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash, Display, EnumString)]
pub enum ItemKind {
    Empty,
    Alteration,
    ArmorClothing,
    ArmorHeavy,
    ArmorLight,
    Arrow,
    AxeOneHanded,
    AxeTwoHanded,
    Bow,
    Claw,
    Conjuration,
    Crossbow,
    Dagger,
    DestructionFire,  // novice
    DestructionFrost, // novice
    DestructionShock, // novice
    Destruction,
    Food,
    Halberd,
    HandToHand,
    IconDefault,
    Illusion,
    Katana,
    Lantern,
    Mace,
    Mask,
    Pike,
    PoisonDefault,
    PotionDefault,
    PotionFireResist,
    PotionFrostResist,
    PotionHealth,
    PotionMagicka,
    PotionMagicResist,
    PotionShockResist,
    PotionStamina,
    Power,
    QuarterStaff,
    Rapier,
    Restoration,
    Scroll,
    Shield,
    Shout,
    SpellDefault,
    Staff,
    SwordOneHanded,
    SwordTwoHanded,
    Torch,
    WeaponDefault,
    Whip,
    NotFound,
    // must follow the earlier kinds
    ArmorClothingHead,
    ArmorClothingHands,
    ArmorClothingFeet,
    ArmorLightHead,
    ArmorLightHands,
    ArmorLightFeet,
    ArmorHeavyHead,
    ArmorHeavyHands,
    ArmorHeavyFeet,
    ArmorCloak,
    ArmorBackpack,
    ArmorBelt,
    ArmorRing,
    ArmorAmulet,
    AlterationDetect,
    AlterationFeather,
    AlterationLight,
    AlterationWind,
    ConjurationBoundWeapon,
    ConjurationSkeleton,
    ConjurationSoulTrap,
    ConjurationWolf,
    ConjurationZombie,
    DestructionFireApprentice,
    DestructionFireAdept,
    DestructionFireExpert,
    DestructionFireMaster,
    DestructionFrostApprentice,
    DestructionFrostAdept,
    DestructionFrostExpert,
    DestructionFrostMaster,
    DestructionShockApprentice,
    DestructionShockAdept,
    DestructionShockExpert,
    DestructionShockMaster,
    Flail,
    Gun,
    IllusionClairvoyance,
    IllusionDemoralize,
    IllusionMuffle,
    IllusionNightEye,
    RestorationCure,
    RestorationHeal,
    RestorationPoison,
    RestorationSunDamage,
    RestorationWard,
    SpellParalyze,
    SpellReflect,
}

/// Given an entry kind, return the filename of the icon to use for it.
/// Exposed to C++.
pub fn get_icon_file(kind: &ItemKind) -> String {
    kind.icon_file()
}

pub fn get_icon_fallback(kind: &ItemKind) -> String {
    kind.fallback_kind().icon_file()
}

pub fn kind_has_count(kind: ItemKind) -> bool {
    kind.has_count()
}

pub fn kind_is_magic(kind: ItemKind) -> bool {
    kind.is_magic()
}

/// We cannot derive default for shared enums, so we define it here.
impl Default for ItemKind {
    fn default() -> Self {
        ItemKind::Empty
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
                | ItemKind::AlterationDetect
                | ItemKind::AlterationFeather
                | ItemKind::AlterationLight
                | ItemKind::AlterationWind
                | ItemKind::ConjurationSkeleton
                | ItemKind::ConjurationSoulTrap
                | ItemKind::ConjurationWolf
                | ItemKind::ConjurationZombie
                | ItemKind::DestructionFireApprentice
                | ItemKind::DestructionFireAdept
                | ItemKind::DestructionFireExpert
                | ItemKind::DestructionFireMaster
                | ItemKind::DestructionFrostApprentice
                | ItemKind::DestructionFrostAdept
                | ItemKind::DestructionFrostExpert
                | ItemKind::DestructionFrostMaster
                | ItemKind::DestructionShockApprentice
                | ItemKind::DestructionShockAdept
                | ItemKind::DestructionShockExpert
                | ItemKind::DestructionShockMaster
                | ItemKind::IllusionClairvoyance
                | ItemKind::IllusionDemoralize
                | ItemKind::IllusionMuffle
                | ItemKind::IllusionNightEye
                | ItemKind::RestorationCure
                | ItemKind::RestorationHeal
                | ItemKind::RestorationPoison
                | ItemKind::RestorationSunDamage
                | ItemKind::RestorationWard
                | ItemKind::SpellParalyze
                | ItemKind::SpellReflect
                | ItemKind::ConjurationBoundWeapon
        )
    }

    pub fn has_count(&self) -> bool {
        self.count_matters()
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
                | ItemKind::ArmorClothingHead
                | ItemKind::ArmorClothingHands
                | ItemKind::ArmorClothingFeet
                | ItemKind::ArmorLightHead
                | ItemKind::ArmorLightHands
                | ItemKind::ArmorLightFeet
                | ItemKind::ArmorHeavyHead
                | ItemKind::ArmorHeavyHands
                | ItemKind::ArmorHeavyFeet
                | ItemKind::ArmorCloak
                | ItemKind::ArmorBackpack
                | ItemKind::ArmorRing
                | ItemKind::ArmorAmulet
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
        self.is_armor()
            || matches!(
                *self,
                ItemKind::Arrow
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

    /// If the player's layout doesn't have a specific icon for this kind, we use a more
    /// generic one.
    pub fn fallback_kind(&self) -> &ItemKind {
        if matches!(
            *self,
            ItemKind::ArmorClothingHead
                | ItemKind::ArmorClothingHands
                | ItemKind::ArmorClothingFeet
        ) {
            return &ItemKind::ArmorClothing;
        }
        if matches!(
            *self,
            ItemKind::ArmorLightHead | ItemKind::ArmorLightHands | ItemKind::ArmorLightFeet
        ) {
            return &ItemKind::ArmorLight;
        }
        if matches!(
            *self,
            ItemKind::ArmorHeavyHead | ItemKind::ArmorHeavyHands | ItemKind::ArmorHeavyFeet
        ) {
            return &ItemKind::ArmorClothing;
        }
        if matches!(
            *self,
            ItemKind::ArmorCloak
                | ItemKind::ArmorBackpack
                | ItemKind::ArmorRing
                | ItemKind::ArmorAmulet
        ) {
            return &ItemKind::ArmorClothing;
        }
        if matches!(
            *self,
            ItemKind::AlterationDetect
                | ItemKind::AlterationLight
                | ItemKind::AlterationWind
                | ItemKind::AlterationFeather
        ) {
            return &ItemKind::Alteration;
        }
        if matches!(
            *self,
            ItemKind::ConjurationSoulTrap
                | ItemKind::ConjurationZombie
                | ItemKind::ConjurationBoundWeapon
        ) {
            return &ItemKind::Conjuration;
        }
        if matches!(
            *self,
            ItemKind::DestructionFireApprentice
                | ItemKind::DestructionFireAdept
                | ItemKind::DestructionFireExpert
                | ItemKind::DestructionFireMaster
        ) {
            return &ItemKind::DestructionFire;
        }
        if matches!(
            *self,
            ItemKind::DestructionFrostApprentice
                | ItemKind::DestructionFrostAdept
                | ItemKind::DestructionFrostExpert
                | ItemKind::DestructionFrostMaster
        ) {
            return &ItemKind::DestructionFrost;
        }
        if matches!(
            *self,
            ItemKind::DestructionShockApprentice
                | ItemKind::DestructionShockAdept
                | ItemKind::DestructionShockExpert
                | ItemKind::DestructionShockMaster
        ) {
            return &ItemKind::DestructionShock;
        }
        if matches!(
            *self,
            ItemKind::IllusionClairvoyance
                | ItemKind::IllusionDemoralize
                | ItemKind::IllusionMuffle
                | ItemKind::IllusionNightEye
        ) {
            return &ItemKind::Illusion;
        }
        if matches!(
            *self,
            ItemKind::RestorationCure
                | ItemKind::RestorationHeal
                | ItemKind::RestorationPoison
                | ItemKind::RestorationSunDamage
                | ItemKind::RestorationWard
        ) {
            return &ItemKind::Restoration;
        }
        if matches!(*self, ItemKind::Flail | ItemKind::Gun) {
            return &ItemKind::WeaponDefault;
        }
        if matches!(*self, ItemKind::SpellParalyze | ItemKind::SpellReflect) {
            return &ItemKind::Alteration;
        }

        &ItemKind::IconDefault
    }
}

static ICON_MAP: Lazy<HashMap<ItemKind, String>> = Lazy::new(|| {
    HashMap::from([
        (ItemKind::Alteration, "alteration.svg".to_string()),
        (ItemKind::ArmorClothing, "armor_clothing.svg".to_string()),
        (ItemKind::ArmorHeavy, "armor_heavy.svg".to_string()),
        (ItemKind::ArmorLight, "armor_light.svg".to_string()),
        (ItemKind::Arrow, "arrow.svg".to_string()),
        (
            ItemKind::AxeOneHanded,
            "weapon_axe_one_handed.svg".to_string(),
        ),
        (
            ItemKind::AxeTwoHanded,
            "weapon_axe_two_handed.svg".to_string(),
        ),
        (ItemKind::Bow, "weapon_bow.svg".to_string()),
        (ItemKind::Claw, "weapon_claw.svg".to_string()),
        (ItemKind::Conjuration, "conjuration.svg".to_string()),
        (ItemKind::Crossbow, "weapon_crossbow.svg".to_string()),
        (ItemKind::Dagger, "weapon_dagger.svg".to_string()),
        (ItemKind::DestructionFire, "spell_fire.svg".to_string()),
        (ItemKind::DestructionFrost, "spell_frost.svg".to_string()),
        (ItemKind::DestructionShock, "spell_shock.svg".to_string()),
        (ItemKind::Destruction, "destruction.svg".to_string()),
        (ItemKind::Food, "food.svg".to_string()),
        (ItemKind::Halberd, "weapon_halberd.svg".to_string()),
        (ItemKind::HandToHand, "hand_to_hand.svg".to_string()),
        (ItemKind::IconDefault, "icon_default.svg".to_string()),
        (ItemKind::Illusion, "illusion.svg".to_string()),
        (ItemKind::Katana, "weapon_katana.svg".to_string()),
        (ItemKind::Lantern, "lantern.svg".to_string()),
        (ItemKind::Mace, "weapon_mace.svg".to_string()),
        (ItemKind::Mask, "mask.svg".to_string()),
        (ItemKind::Pike, "weapon_pike.svg".to_string()),
        (ItemKind::PoisonDefault, "poison_default.svg".to_string()),
        (ItemKind::PotionDefault, "potion_default.svg".to_string()),
        (
            ItemKind::PotionFireResist,
            "potion_resist_fire.svg".to_string(),
        ),
        (
            ItemKind::PotionFrostResist,
            "potion_resist_frost.svg".to_string(),
        ),
        (ItemKind::PotionHealth, "potion_health.svg".to_string()),
        (ItemKind::PotionMagicka, "potion_magicka.svg".to_string()),
        (
            ItemKind::PotionMagicResist,
            "potion_resist_magic.svg".to_string(),
        ),
        (
            ItemKind::PotionShockResist,
            "potion_resist_shock.svg".to_string(),
        ),
        (ItemKind::PotionStamina, "potion_stamina.svg".to_string()),
        (ItemKind::Power, "power.svg".to_string()),
        (
            ItemKind::QuarterStaff,
            "weapon_quarterstaff.svg".to_string(),
        ),
        (ItemKind::Rapier, "weapon_rapier.svg".to_string()),
        (ItemKind::Restoration, "restoration.svg".to_string()),
        (ItemKind::Scroll, "scroll.svg".to_string()),
        (ItemKind::Shield, "armor_shield.svg".to_string()),
        (ItemKind::Shout, "shout.svg".to_string()),
        (ItemKind::SpellDefault, "spell_default.svg".to_string()),
        (ItemKind::Staff, "weapon_staff.svg".to_string()),
        (
            ItemKind::SwordOneHanded,
            "weapon_sword_one_handed.svg".to_string(),
        ),
        (
            ItemKind::SwordTwoHanded,
            "weapon_sword_two_handed.svg".to_string(),
        ),
        (ItemKind::Torch, "torch.svg".to_string()),
        (
            ItemKind::WeaponDefault,
            "weapon_sword_one_handed.svg".to_string(),
        ),
        (ItemKind::Whip, "weapon_whip.svg".to_string()),
        // newer items
        (
            ItemKind::ArmorClothingHead,
            "armor_clothing_head.svg".to_string(),
        ),
        (
            ItemKind::ArmorClothingHands,
            "armor_clothing_hands.svg".to_string(),
        ),
        (
            ItemKind::ArmorClothingFeet,
            "armor_clothing_feet.svg".to_string(),
        ),
        (ItemKind::ArmorLightHead, "armor_light_head.svg".to_string()),
        (
            ItemKind::ArmorLightHands,
            "armor_light_hands.svg".to_string(),
        ),
        (ItemKind::ArmorLightFeet, "armor_light_feed.svg".to_string()),
        (ItemKind::ArmorHeavyHead, "armor_heavy_head.svg".to_string()),
        (
            ItemKind::ArmorHeavyHands,
            "armor_heavy_hands.svg".to_string(),
        ),
        (ItemKind::ArmorHeavyFeet, "armor_heavy_feet.svg".to_string()),
        (ItemKind::ArmorCloak, "armor_cloak.svg".to_string()),
        (ItemKind::ArmorBackpack, "armor_backpack.svg".to_string()),
        (ItemKind::ArmorRing, "armor_ring.svg".to_string()),
        (ItemKind::ArmorAmulet, "armor_amulet.svg".to_string()),
        (
            ItemKind::AlterationDetect,
            "alteration_detect.svg".to_string(),
        ),
        (
            ItemKind::AlterationFeather,
            "alteration_feather.svg".to_string(),
        ),
        (
            ItemKind::AlterationLight,
            "alteration_light.svg".to_string(),
        ),
        (ItemKind::AlterationWind, "alteration_wind.svg".to_string()),
        (
            ItemKind::ConjurationBoundWeapon,
            "conjuration_bound_weapon.svg".to_string(),
        ),
        (
            ItemKind::ConjurationSoulTrap,
            "conjuration_soultrap.svg".to_string(),
        ),
        (
            ItemKind::ConjurationZombie,
            "conjuration_zombie.svg".to_string(),
        ),
        (
            ItemKind::DestructionFireApprentice,
            "destruction_fire_apprentice.svg".to_string(),
        ),
        (
            ItemKind::DestructionFireAdept,
            "destruction_fire_adept.svg".to_string(),
        ),
        (
            ItemKind::DestructionFireExpert,
            "destruction_fire_expert.svg".to_string(),
        ),
        (
            ItemKind::DestructionFireMaster,
            "destruction_fire_master.svg".to_string(),
        ),
        (
            ItemKind::DestructionFrostApprentice,
            "destruction_frost_apprentice.svg".to_string(),
        ),
        (
            ItemKind::DestructionFrostAdept,
            "destruction_fire_adept.svg".to_string(),
        ),
        (
            ItemKind::DestructionFrostExpert,
            "destruction_fire_expert.svg".to_string(),
        ),
        (
            ItemKind::DestructionFrostMaster,
            "destruction_fire_master.svg".to_string(),
        ),
        (
            ItemKind::DestructionShockApprentice,
            "destruction_frost_apprentice.svg".to_string(),
        ),
        (
            ItemKind::DestructionShockAdept,
            "destruction_fire_adept.svg".to_string(),
        ),
        (
            ItemKind::DestructionShockExpert,
            "destruction_fire_expert.svg".to_string(),
        ),
        (
            ItemKind::DestructionShockMaster,
            "destruction_fire_master.svg".to_string(),
        ),
        (ItemKind::Flail, "flail.svg".to_string()),
        (ItemKind::Gun, "gun.svg".to_string()),
        (
            ItemKind::IllusionClairvoyance,
            "illusion_clairvoyance.svg".to_string(),
        ),
        (
            ItemKind::IllusionDemoralize,
            "illusion_demoralize.svg".to_string(),
        ),
        (ItemKind::IllusionMuffle, "illusion_muffle.svg".to_string()),
        (
            ItemKind::IllusionNightEye,
            "illusion_nighteye.svg".to_string(),
        ),
        (
            ItemKind::RestorationCure,
            "restoration_cure.svg".to_string(),
        ),
        (
            ItemKind::RestorationHeal,
            "restoration_heal.svg".to_string(),
        ),
        (
            ItemKind::RestorationPoison,
            "restoration_poison.svg".to_string(),
        ),
        (
            ItemKind::RestorationSunDamage,
            "restoration_sundamage.svg".to_string(),
        ),
        (
            ItemKind::RestorationWard,
            "restoration_ward.svg".to_string(),
        ),
        (ItemKind::SpellParalyze, "spell_paralyze.svg".to_string()),
        (ItemKind::SpellReflect, "spell_reflect.svg".to_string()),
    ])
});

// This is horrific. There has to be a better way.
// A way to iterate, perhaps?
impl From<u8> for ItemKind {
    fn from(value: u8) -> Self {
        if value == 0 {
            ItemKind::Empty
        } else if value == 1 {
            ItemKind::Alteration
        } else if value == 2 {
            ItemKind::ArmorClothing
        } else if value == 3 {
            ItemKind::ArmorHeavy
        } else if value == 4 {
            ItemKind::ArmorLight
        } else if value == 5 {
            ItemKind::Arrow
        } else if value == 6 {
            ItemKind::AxeOneHanded
        } else if value == 7 {
            ItemKind::AxeTwoHanded
        } else if value == 8 {
            ItemKind::Bow
        } else if value == 9 {
            ItemKind::Claw
        } else if value == 10 {
            ItemKind::Conjuration
        } else if value == 11 {
            ItemKind::Crossbow
        } else if value == 12 {
            ItemKind::Dagger
        } else if value == 13 {
            ItemKind::DestructionFire
        } else if value == 14 {
            ItemKind::DestructionFrost
        } else if value == 15 {
            ItemKind::DestructionShock
        } else if value == 16 {
            ItemKind::Destruction
        } else if value == 17 {
            ItemKind::Food
        } else if value == 18 {
            ItemKind::Halberd
        } else if value == 19 {
            ItemKind::HandToHand
        } else if value == 20 {
            ItemKind::IconDefault
        } else if value == 21 {
            ItemKind::Illusion
        } else if value == 22 {
            ItemKind::Katana
        } else if value == 23 {
            ItemKind::Lantern
        } else if value == 24 {
            ItemKind::Mace
        } else if value == 25 {
            ItemKind::Mask
        } else if value == 26 {
            ItemKind::Pike
        } else if value == 27 {
            ItemKind::PoisonDefault
        } else if value == 28 {
            ItemKind::PotionDefault
        } else if value == 29 {
            ItemKind::PotionFireResist
        } else if value == 30 {
            ItemKind::PotionFrostResist
        } else if value == 31 {
            ItemKind::PotionHealth
        } else if value == 32 {
            ItemKind::PotionMagicka
        } else if value == 33 {
            ItemKind::PotionMagicResist
        } else if value == 34 {
            ItemKind::PotionShockResist
        } else if value == 35 {
            ItemKind::PotionStamina
        } else if value == 36 {
            ItemKind::Power
        } else if value == 37 {
            ItemKind::QuarterStaff
        } else if value == 38 {
            ItemKind::Rapier
        } else if value == 39 {
            ItemKind::Restoration
        } else if value == 40 {
            ItemKind::Scroll
        } else if value == 41 {
            ItemKind::Shield
        } else if value == 42 {
            ItemKind::Shout
        } else if value == 43 {
            ItemKind::SpellDefault
        } else if value == 44 {
            ItemKind::Staff
        } else if value == 45 {
            ItemKind::SwordOneHanded
        } else if value == 46 {
            ItemKind::SwordTwoHanded
        } else if value == 47 {
            ItemKind::Torch
        } else if value == 48 {
            ItemKind::WeaponDefault
        } else if value == 49 {
            ItemKind::Whip
        } else {
            ItemKind::Empty
        }
    }
}

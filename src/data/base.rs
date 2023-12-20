//! Base item kinds, from the starting icon set.

use strum::Display;

use super::ammo::AmmoType;
use super::armor::ArmorType;
use super::color::InvColor;
use super::food::FoodType;
use super::potion::PotionType;
use super::power::PowerType;
use super::shout::ShoutType;
use super::spell::SpellType;
use super::weapon::WeaponType;
use super::{HasIcon, HasKeywords};
use crate::images::icons::Icon;
use crate::plugin::{Color, ItemCategory};

#[derive(Clone, Debug, Default, Display, Eq, Hash, PartialEq)]
pub enum BaseType {
    #[default]
    Empty,
    Ammo(AmmoType),
    Armor(ArmorType),
    Food(FoodType),
    HandToHand,
    Light(LightType),
    Potion(PotionType),
    PotionProxy(Proxy),
    Power(PowerType),
    Scroll(SpellType),
    Shout(ShoutType),
    Spell(SpellType),
    Weapon(WeaponType),
    Equipset(Icon),
}

#[derive(Clone, Debug, Display, Eq, Hash, PartialEq)]
pub enum Proxy {
    Health,
    Magicka,
    Stamina,
}

impl HasIcon for Proxy {
    fn color(&self) -> Color {
        match self {
            Proxy::Health => InvColor::Red.color(),
            Proxy::Magicka => InvColor::Blue.color(),
            Proxy::Stamina => InvColor::Green.color(),
        }
    }

    fn icon(&self) -> &Icon {
        match self {
            Proxy::Health => &Icon::PotionHealth,
            Proxy::Magicka => &Icon::PotionMagicka,
            Proxy::Stamina => &Icon::PotionStamina,
        }
    }
}

#[derive(Clone, Debug, Display, Eq, Hash, PartialEq, Default)]
pub enum LightType {
    #[default]
    Torch,
    Lantern,
}

impl HasIcon for LightType {
    fn color(&self) -> Color {
        InvColor::Sun.color()
    }

    fn icon(&self) -> &Icon {
        match self {
            LightType::Torch => &Icon::MiscTorch,
            LightType::Lantern => &Icon::MiscLantern,
        }
    }
}

impl BaseType {
    pub fn classify(
        name: &str,
        category: ItemCategory,
        keywords: Vec<String>,
        twohanded: bool,
    ) -> Self {
        match category {
            ItemCategory::Ammo => Self::Ammo(AmmoType::classify(name, keywords.clone(), twohanded)),
            ItemCategory::Armor => {
                Self::Armor(ArmorType::classify(name, keywords.clone(), twohanded))
            }
            ItemCategory::Food => Self::Food(FoodType::classify(name, keywords.clone(), twohanded)),
            ItemCategory::HandToHand => Self::HandToHand,
            ItemCategory::Lantern => Self::Light(LightType::Lantern),
            ItemCategory::Potion => Self::Potion(PotionType::Default),
            ItemCategory::Power => Self::Power(PowerType::new(name, keywords)),
            ItemCategory::Scroll => Self::Scroll(SpellType::default()),
            ItemCategory::Shout => Self::Shout(ShoutType::new(keywords)),
            ItemCategory::Spell => Self::Spell(SpellType::default()),
            ItemCategory::Torch => Self::Light(LightType::Torch),
            ItemCategory::Weapon => {
                Self::Weapon(WeaponType::classify(name, keywords.clone(), twohanded))
            }
            _ => BaseType::Empty,
        }
    }

    pub fn count_matters(&self) -> bool {
        match *self {
            BaseType::Empty => false,
            BaseType::Ammo(_) => true,
            BaseType::Armor(_) => true,
            BaseType::Equipset(_) => false,
            BaseType::Food(_) => true,
            BaseType::HandToHand => false,
            BaseType::Light(_) => true,
            BaseType::Potion(_) => true,
            BaseType::PotionProxy(_) => true,
            BaseType::Power(_) => false,
            BaseType::Scroll(_) => true,
            BaseType::Shout(_) => false,
            BaseType::Spell(_) => false,
            BaseType::Weapon(_) => true,
        }
    }

    pub fn is_ammo(&self) -> bool {
        matches!(self, BaseType::Ammo(_))
    }

    pub fn is_armor(&self) -> bool {
        matches!(self, BaseType::Armor(_))
    }

    pub fn is_magic(&self) -> bool {
        matches!(self, BaseType::Spell(_) | BaseType::Scroll(_))
    }

    pub fn is_potion(&self) -> bool {
        matches!(self, BaseType::Potion(_))
    }

    pub fn is_power(&self) -> bool {
        matches!(self, BaseType::Power(_) | BaseType::Shout(_))
    }

    pub fn is_spell(&self) -> bool {
        matches!(self, BaseType::Spell(_))
    }

    pub fn is_utility(&self) -> bool {
        match self {
            BaseType::Empty => false,
            BaseType::Ammo(_) => false,
            BaseType::Armor(t) => t.is_utility(),
            BaseType::Equipset(_) => false,
            BaseType::Food(_) => true,
            BaseType::HandToHand => false,
            BaseType::Light(_) => false, // These are held lights, not worn lights. Worn lights are armor.
            BaseType::Potion(_) => true,
            BaseType::PotionProxy(_) => true,
            BaseType::Power(_) => false,
            BaseType::Scroll(_) => false,
            BaseType::Shout(_) => false,
            BaseType::Spell(_) => false,
            BaseType::Weapon(_) => false,
        }
    }

    pub fn is_weapon(&self) -> bool {
        matches!(self, BaseType::Weapon(_))
    }

    pub fn is_one_handed(&self) -> bool {
        match self {
            BaseType::Weapon(t) => t.is_one_handed(),
            BaseType::Spell(v) => !v.is_two_handed(),
            _ => true,
        }
    }

    pub fn is_two_handed(&self) -> bool {
        match self {
            BaseType::Weapon(t) => t.is_two_handed(),
            BaseType::Scroll(t) => t.is_two_handed(),
            BaseType::Spell(t) => t.is_two_handed(),
            _ => false,
        }
    }

    pub fn left_hand_ok(&self) -> bool {
        match self {
            BaseType::Empty => false,
            BaseType::Ammo(_) => false,
            BaseType::Armor(t) => !t.is_utility(),
            BaseType::Equipset(_) => false,
            BaseType::Food(_) => false,
            BaseType::HandToHand => true,
            BaseType::Light(_) => true,
            BaseType::Potion(_) => false,
            BaseType::PotionProxy(_) => false,
            BaseType::Power(_) => false,
            BaseType::Scroll(t) => !t.is_two_handed(),
            BaseType::Shout(_) => false,
            BaseType::Spell(t) => !t.is_two_handed(),
            BaseType::Weapon(t) => t.left_hand_ok(),
        }
    }

    pub fn right_hand_ok(&self) -> bool {
        match self {
            BaseType::Empty => false,
            BaseType::Ammo(_) => false,
            BaseType::Armor(_) => false,
            BaseType::Equipset(_) => false,
            BaseType::Food(_) => false,
            BaseType::HandToHand => true,
            BaseType::Light(_) => false,
            BaseType::Potion(_) => false,
            BaseType::PotionProxy(_) => false,
            BaseType::Power(_) => false,
            BaseType::Scroll(_) => true,
            BaseType::Shout(_) => false,
            BaseType::Spell(_) => true,
            BaseType::Weapon(t) => t.right_hand_ok(),
        }
    }
}

impl HasIcon for BaseType {
    fn color(&self) -> Color {
        match self {
            BaseType::Empty => Color::default(),
            BaseType::Ammo(t) => t.color(),
            BaseType::Armor(t) => t.color(),
            BaseType::Equipset(_) => Color::default(),
            BaseType::Food(t) => t.color(),
            BaseType::HandToHand => Color::default(),
            BaseType::Light(t) => t.color(),
            BaseType::Potion(t) => t.color(),
            BaseType::PotionProxy(t) => t.color(),
            BaseType::Power(t) => t.color(),
            BaseType::Scroll(t) => t.color(),
            BaseType::Shout(t) => t.color(),
            BaseType::Spell(t) => t.color(),
            BaseType::Weapon(t) => t.color(),
        }
    }

    fn icon(&self) -> &Icon {
        match self {
            BaseType::Empty => &Icon::IconDefault,
            BaseType::Ammo(t) => t.icon(),
            BaseType::Armor(t) => t.icon(),
            BaseType::Equipset(t) => t,
            BaseType::Food(t) => t.icon(),
            BaseType::HandToHand => &Icon::HandToHand,
            BaseType::Light(t) => t.icon(),
            BaseType::Potion(t) => t.icon(),
            BaseType::PotionProxy(t) => t.icon(),
            BaseType::Power(t) => t.icon(),
            BaseType::Scroll(_) => &Icon::Scroll,
            BaseType::Shout(t) => t.icon(),
            BaseType::Spell(t) => t.icon(),
            BaseType::Weapon(t) => t.icon(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::color::{color_from_keywords, InvColor};
    use crate::data::magic::{MagicCategory, SpellData};
    use crate::data::weapon::{WeaponEquipType, WeaponType};

    #[test]
    fn can_extract_color() {
        let input = vec![
            "OCF_InvColorFire".to_string(),
            "OCF_AccessoryBelt".to_string(),
            "Armor".to_string(),
        ];
        assert_eq!(color_from_keywords(&input), Some(InvColor::Fire));

        let input = vec![
            "Ammo".to_string(),
            "OCF_AmmoTypeBullet1H".to_string(),
            "OCF_AmmoTypeBullet".to_string(),
            "OCF_InvColorWater".to_string(),
            "OCF_AmmoTypeBullet1H_Basic".to_string(),
        ];
        let color_keywords: Vec<InvColor> = input
            .iter()
            .filter_map(|xs| InvColor::try_from(xs.as_str()).ok())
            .collect();
        assert_eq!(1, color_keywords.len());
        assert_eq!(color_from_keywords(&input), Some(InvColor::Water));
    }

    #[test]
    fn keywords_convert() {
        let input = vec![
            "Ammo".to_string(),
            "OCF_AmmoTypeBullet1H".to_string(),
            "OCF_AmmoTypeBullet".to_string(),
            "OCF_InvColorWater".to_string(),
            "OCF_AmmoTypeBullet1H_Basic".to_string(),
        ];

        let result = BaseType::classify("TestName", ItemCategory::Ammo, input, true);
        assert_eq!(result, BaseType::Ammo(AmmoType::Bullet(InvColor::Water)));

        let input = vec![
            "OCF_InvColorFire".to_string(),
            "OCF_AccessoryBelt".to_string(),
            "Armor".to_string(),
        ];
        let result = BaseType::classify("TestName", ItemCategory::Armor, input, false);
        let at = ArmorType::new(Icon::ArmorBelt, InvColor::Fire);
        assert_eq!(result, BaseType::Armor(at));

        let input = vec![
            "OCF_InvColorFire".to_string(),
            "OCF_WeapTypeLongsword2H".to_string(),
            "Weapon".to_string(),
            "*Longsword".to_string(),
        ];
        let result = BaseType::classify("TestName", ItemCategory::Weapon, input, true);
        let wt = WeaponType::new(
            Icon::WeaponSwordTwoHanded,
            InvColor::Fire,
            WeaponEquipType::TwoHanded,
        );
        assert_eq!(result, BaseType::Weapon(wt));
    }

    #[test]
    fn utility_items() {
        let shield = ArmorType::new(Icon::ArmorShieldHeavy, InvColor::Ash);
        let item = BaseType::Armor(shield);
        assert!(!item.is_utility());

        let armor = ArmorType::new(Icon::ArmorHeavyHead, InvColor::Ash);
        let item = BaseType::Armor(armor);
        assert!(item.is_utility());

        assert!(BaseType::Potion(PotionType::Health).is_utility());
        assert!(BaseType::Potion(PotionType::Magicka).is_utility());
        assert!(BaseType::Potion(PotionType::Stamina).is_utility());
        assert!(BaseType::Potion(PotionType::Poison).is_utility());
        assert!(BaseType::Potion(PotionType::Default).is_utility());
        assert!(BaseType::Potion(PotionType::Resist(MagicCategory::Shock)).is_utility());

        let food = BaseType::Food(FoodType::default());
        assert!(food.is_utility());
    }

    #[test]
    fn lanterns_and_torches() {
        // these are held items
        assert!(!BaseType::Light(LightType::Lantern).is_utility());
        assert!(!BaseType::Light(LightType::Torch).is_utility());

        // this is worn
        let lantern = BaseType::Armor(ArmorType::new(Icon::MiscLantern, InvColor::Sun));
        assert!(lantern.is_utility());
        assert!(!lantern.left_hand_ok());
    }

    #[test]
    fn left_hand_items() {
        assert!(BaseType::Light(LightType::Lantern).left_hand_ok());
        let shield = BaseType::Armor(ArmorType::new(Icon::ArmorShieldHeavy, InvColor::Ash));
        assert!(shield.left_hand_ok());
        let dagger = BaseType::Weapon(WeaponType::new(
            Icon::WeaponDagger,
            InvColor::Blue,
            WeaponEquipType::EitherHand,
        ));
        assert!(dagger.left_hand_ok());
        let dagger = BaseType::Weapon(WeaponType::new(
            Icon::WeaponDagger,
            InvColor::Blue,
            WeaponEquipType::LeftHand,
        ));
        assert!(dagger.left_hand_ok());

        let sword = BaseType::Weapon(WeaponType::new(
            Icon::WeaponSwordTwoHanded,
            InvColor::Blue,
            WeaponEquipType::TwoHanded,
        ));
        assert!(!sword.left_hand_ok());

        let sword = BaseType::Weapon(WeaponType::new(
            Icon::WeaponSwordTwoHanded,
            InvColor::Blue,
            WeaponEquipType::RightHand,
        ));
        assert!(!sword.left_hand_ok());

        let spell = BaseType::Spell(SpellType::new(
            SpellData::new(
                true, 43, // ActorValue::ResistFrost
                false, 20, // School::Destruction,
                25, 1, // SpellArchetype::ValueModifier,
            ),
            Vec::new(),
        ));
        assert!(spell.left_hand_ok());

        let master_spell = BaseType::Spell(SpellType::new(
            SpellData::new(
                false, 0, // ActorValue::ResistFrost
                true, 22, // School::Restoration,
                100, 1, // SpellArchetype::ValueModifier,
            ),
            Vec::new(),
        ));
        assert!(!master_spell.left_hand_ok());
    }

    #[test]
    fn right_hand_items() {
        assert!(!BaseType::Light(LightType::Lantern).right_hand_ok());

        let shield = BaseType::Armor(ArmorType::new(Icon::ArmorShieldHeavy, InvColor::Ash));
        assert!(!shield.right_hand_ok());

        let dagger = BaseType::Weapon(WeaponType::new(
            Icon::WeaponDagger,
            InvColor::Blue,
            WeaponEquipType::EitherHand,
        ));
        assert!(dagger.right_hand_ok());

        let dagger = BaseType::Weapon(WeaponType::new(
            Icon::WeaponDagger,
            InvColor::Blue,
            WeaponEquipType::LeftHand,
        ));
        assert!(!dagger.right_hand_ok());

        let sword = BaseType::Weapon(WeaponType::new(
            Icon::WeaponSwordTwoHanded,
            InvColor::Blue,
            WeaponEquipType::TwoHanded,
        ));
        assert!(sword.right_hand_ok());

        let sword = BaseType::Weapon(WeaponType::new(
            Icon::WeaponSwordTwoHanded,
            InvColor::Blue,
            WeaponEquipType::RightHand,
        ));
        assert!(sword.right_hand_ok());

        let spell = BaseType::Spell(SpellType::new(
            SpellData::new(
                true, 43, // ActorValue::ResistFrost
                false, 20, // School::Destruction,
                25, 1, // SpellArchetype::ValueModifier,
            ),
            Vec::new(),
        ));
        assert!(spell.right_hand_ok());

        let master_spell = BaseType::Spell(SpellType::new(
            SpellData::new(
                false, 0, // ActorValue::ResistFrost
                true, 22, // School::Restoration,
                100, 1, // SpellArchetype::ValueModifier,
            ),
            Vec::new(),
        ));
        assert!(master_spell.right_hand_ok());
    }
}

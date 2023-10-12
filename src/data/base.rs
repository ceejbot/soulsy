//! Base item kinds, from the starting icon set.

use strum::Display;

use super::ammo::AmmoType;
use super::armor::ArmorType;
use super::color::InvColor;
use super::food::FoodType;
use super::icons::Icon;
use super::potion::PotionType;
use super::shout::ShoutType;
use super::spell::SpellType;
use super::weapon::WeaponType;
use super::{HasIcon, HasKeywords, IsHudItem};
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
    Power,
    Scroll(SpellType),
    Shout(ShoutType),
    Spell(SpellType),
    Weapon(WeaponType),
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

    fn icon_file(&self) -> String {
        match self {
            Proxy::Health => Icon::PotionHealth.icon_file(),
            Proxy::Magicka => Icon::PotionMagicka.icon_file(),
            Proxy::Stamina => Icon::PotionStamina.icon_file(),
        }
    }

    fn icon_fallback(&self) -> String {
        Icon::PotionDefault.icon_file()
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

    fn icon_file(&self) -> String {
        match self {
            LightType::Torch => Icon::Torch.icon_file(),
            LightType::Lantern => Icon::Lantern.icon_file(),
        }
    }

    fn icon_fallback(&self) -> String {
        Icon::Torch.icon_file()
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
            ItemCategory::Power => Self::Power,
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

    pub fn is_ranged(&self) -> bool {
        match self {
            BaseType::Weapon(t) => t.is_ranged(),
            _ => false,
        }
    }
}

pub fn color_from_keywords(keywords: &[String]) -> InvColor {
    let color_keywords: Vec<InvColor> = keywords
        .iter()
        .filter_map(|xs| InvColor::try_from(xs.as_str()).ok())
        .collect();
    if let Some(c) = color_keywords.first() {
        c.clone()
    } else {
        InvColor::default()
    }
}

impl HasIcon for BaseType {
    fn color(&self) -> Color {
        match self {
            BaseType::Empty => Color::default(),
            BaseType::Ammo(t) => t.color(),
            BaseType::Armor(t) => t.color(),
            BaseType::Food(t) => t.color(),
            BaseType::HandToHand => Color::default(),
            BaseType::Light(t) => t.color(),
            BaseType::Potion(t) => t.color(),
            BaseType::PotionProxy(t) => t.color(),
            BaseType::Power => Color::default(),
            BaseType::Scroll(t) => t.color(),
            BaseType::Shout(t) => t.color(),
            BaseType::Spell(t) => t.color(),
            BaseType::Weapon(t) => t.color(),
        }
    }

    fn icon_file(&self) -> String {
        match self {
            BaseType::Empty => Icon::IconDefault.icon_file(),
            BaseType::Ammo(_) => Icon::Arrow.icon_file(),
            BaseType::Armor(t) => t.icon_file(),
            BaseType::Food(t) => t.icon_file(), // TODO
            BaseType::HandToHand => Icon::HandToHand.icon_file(),
            BaseType::Light(t) => t.icon_file(),
            BaseType::Potion(t) => t.icon_file(),
            BaseType::PotionProxy(t) => t.icon_file(),
            BaseType::Power => Icon::Power.icon_file(),
            BaseType::Scroll(_) => Icon::Scroll.icon_file(),
            BaseType::Shout(t) => t.icon_file(),
            BaseType::Spell(t) => t.icon_file(),
            BaseType::Weapon(t) => t.icon_file(),
        }
    }

    fn icon_fallback(&self) -> String {
        match self {
            BaseType::Empty => Icon::IconDefault.icon_file(),
            BaseType::Ammo(_) => Icon::Arrow.icon_file(),
            BaseType::Armor(t) => t.icon_fallback(),
            BaseType::Food(t) => t.icon_fallback(),
            BaseType::HandToHand => Icon::HandToHand.icon_file(),
            BaseType::Light(t) => t.icon_file(),
            BaseType::Potion(t) => t.icon_fallback(),
            BaseType::PotionProxy(t) => t.icon_fallback(),
            BaseType::Power => Icon::Shout.icon_file(),
            BaseType::Scroll(_) => Icon::Scroll.icon_file(),
            BaseType::Shout(t) => t.icon_fallback(),
            BaseType::Spell(t) => t.icon_fallback(),
            BaseType::Weapon(t) => t.icon_fallback(),
        }
    }
}

impl IsHudItem for BaseType {
    fn count_matters(&self) -> bool {
        match *self {
            BaseType::Empty => false,
            BaseType::Ammo(_) => true,
            BaseType::Armor(_) => true,
            BaseType::Food(_) => true,
            BaseType::HandToHand => false,
            BaseType::Light(_) => true,
            BaseType::Potion(_) => true,
            BaseType::PotionProxy(_) => true,
            BaseType::Power => false,
            BaseType::Scroll(_) => true,
            BaseType::Shout(_) => false,
            BaseType::Spell(_) => false,
            BaseType::Weapon(_) => true,
        }
    }

    fn is_ammo(&self) -> bool {
        matches!(self, BaseType::Ammo(_))
    }

    fn is_armor(&self) -> bool {
        matches!(self, BaseType::Armor(_))
    }

    fn is_magic(&self) -> bool {
        matches!(self, BaseType::Spell(_) | BaseType::Scroll(_))
    }

    fn is_potion(&self) -> bool {
        matches!(self, BaseType::Potion(_))
    }

    fn is_power(&self) -> bool {
        matches!(self, BaseType::Power | BaseType::Shout(_))
    }

    fn is_spell(&self) -> bool {
        matches!(self, BaseType::Spell(_))
    }

    fn is_utility(&self) -> bool {
        match self {
            BaseType::Empty => false,
            BaseType::Ammo(_) => false,
            BaseType::Armor(t) => t.is_utility(),
            BaseType::Food(_) => true,
            BaseType::HandToHand => false,
            BaseType::Light(_) => false,
            BaseType::Potion(_) => true,
            BaseType::PotionProxy(_) => true,
            BaseType::Power => false,
            BaseType::Scroll(_) => false,
            BaseType::Shout(_) => false,
            BaseType::Spell(_) => false,
            BaseType::Weapon(_) => false,
        }
    }

    fn is_weapon(&self) -> bool {
        matches!(self, BaseType::Weapon(_))
    }

    fn is_one_handed(&self) -> bool {
        match self {
            BaseType::Weapon(t) => t.is_one_handed(),
            BaseType::Spell(v) => !v.two_handed(),
            _ => true,
        }
    }

    fn two_handed(&self) -> bool {
        if matches!(self, BaseType::Armor(..)) {
            false
        } else {
            !self.is_one_handed()
        }
    }

    fn left_hand_ok(&self) -> bool {
        match self {
            BaseType::Empty => false,
            BaseType::Ammo(_) => false,
            BaseType::Armor(t) => !t.is_utility(),
            BaseType::Food(_) => false,
            BaseType::HandToHand => true,
            BaseType::Light(_) => true,
            BaseType::Potion(_) => false,
            BaseType::PotionProxy(_) => false,
            BaseType::Power => false,
            BaseType::Scroll(t) => !t.two_handed(),
            BaseType::Shout(_) => false,
            BaseType::Spell(t) => !t.two_handed(),
            BaseType::Weapon(t) => t.left_hand_ok(),
        }
    }

    fn right_hand_ok(&self) -> bool {
        match self {
            BaseType::Empty => false,
            BaseType::Ammo(_) => false,
            BaseType::Armor(_) => false,
            BaseType::Food(_) => false,
            BaseType::HandToHand => true,
            BaseType::Light(_) => false,
            BaseType::Potion(_) => false,
            BaseType::PotionProxy(_) => false,
            BaseType::Power => false,
            BaseType::Scroll(_) => true,
            BaseType::Shout(_) => false,
            BaseType::Spell(_) => true,
            BaseType::Weapon(t) => t.right_hand_ok(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::color::InvColor;
    use crate::data::weapon::{WeaponEquipType, WeaponType};

    #[test]
    fn can_extract_color() {
        let input = vec![
            "OCF_InvColorFire".to_string(),
            "OCF_AccessoryBelt".to_string(),
            "Armor".to_string(),
        ];
        assert_eq!(color_from_keywords(&input), InvColor::Fire);

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
        assert_eq!(color_from_keywords(&input), InvColor::Water);
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
        assert_eq!(item.is_utility(), false);

        let armor = ArmorType::new(Icon::ArmorHeavyHead, InvColor::Ash);
        let item = BaseType::Armor(armor);
        assert_eq!(item.is_utility(), true);

        assert_eq!(BaseType::Light(LightType::Lantern).is_utility(), false);

        let potion = BaseType::Potion(PotionType::Health);
        assert_eq!(potion.is_utility(), true);

        let food = BaseType::Food(FoodType::default());
        assert_eq!(food.is_utility(), true);
    }

    #[test]
    fn left_hand_items() {
        assert_eq!(BaseType::Light(LightType::Lantern).left_hand_ok(), true);
        let shield = BaseType::Armor(ArmorType::new(Icon::ArmorShieldHeavy, InvColor::Ash));
        assert_eq!(shield.left_hand_ok(), true);
        let dagger = BaseType::Weapon(WeaponType::new(
            Icon::WeaponDagger,
            InvColor::Blue,
            WeaponEquipType::EitherHand,
        ));
        assert_eq!(dagger.left_hand_ok(), true);
        let dagger = BaseType::Weapon(WeaponType::new(
            Icon::WeaponDagger,
            InvColor::Blue,
            WeaponEquipType::LeftHand,
        ));
        assert_eq!(dagger.left_hand_ok(), true);

        let sword = BaseType::Weapon(WeaponType::new(
            Icon::WeaponSwordTwoHanded,
            InvColor::Blue,
            WeaponEquipType::TwoHanded,
        ));
        assert_eq!(sword.left_hand_ok(), false);

        let sword = BaseType::Weapon(WeaponType::new(
            Icon::WeaponSwordTwoHanded,
            InvColor::Blue,
            WeaponEquipType::RightHand,
        ));
        assert_eq!(sword.left_hand_ok(), false);
    }

    #[test]
    fn right_hand_items() {
        assert_eq!(BaseType::Light(LightType::Lantern).right_hand_ok(), false);

        let shield = BaseType::Armor(ArmorType::new(Icon::ArmorShieldHeavy, InvColor::Ash));
        assert_eq!(shield.right_hand_ok(), false);

        let dagger = BaseType::Weapon(WeaponType::new(
            Icon::WeaponDagger,
            InvColor::Blue,
            WeaponEquipType::EitherHand,
        ));
        assert_eq!(dagger.right_hand_ok(), true);

        let dagger = BaseType::Weapon(WeaponType::new(
            Icon::WeaponDagger,
            InvColor::Blue,
            WeaponEquipType::LeftHand,
        ));
        assert_eq!(dagger.right_hand_ok(), false);

        let sword = BaseType::Weapon(WeaponType::new(
            Icon::WeaponSwordTwoHanded,
            InvColor::Blue,
            WeaponEquipType::TwoHanded,
        ));
        assert_eq!(sword.right_hand_ok(), true);

        let sword = BaseType::Weapon(WeaponType::new(
            Icon::WeaponSwordTwoHanded,
            InvColor::Blue,
            WeaponEquipType::RightHand,
        ));
        assert_eq!(sword.right_hand_ok(), true);
    }
}

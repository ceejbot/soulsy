//! Base item kinds, from the starting icon set.

use strum::Display;

use super::ammo::AmmoType;
use super::armor::ArmorType;
use super::color::InvColor;
use super::icons::Icon;
use super::potion::PotionType;
use super::shout::ShoutVariant;
use super::spell::{SpellData, SpellType};
use super::weapon::{WeaponEquipType, WeaponType};
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
    Light,
    Potion(PotionType),
    PotionProxy(Proxy),
    Power,
    Scroll(SpellType),
    Shout(ShoutVariant),
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

#[derive(Clone, Debug, Display, Eq, Hash, PartialEq)]
pub enum FoodType {
    // TODO
    Soup,
    Meat,
    Vegetable,
    Fruit,
    Water,
}

impl HasIcon for FoodType {
    fn color(&self) -> Color {
        Color::default()
    }

    fn icon_file(&self) -> String {
        Icon::Food.icon_file()
    }

    fn icon_fallback(&self) -> String {
        Icon::Food.icon_file()
    }
}

impl BaseType {
    pub fn create_spell(data: SpellData) -> Self {
        BaseType::Spell(SpellType::from_spell_data(data))
    }

    pub fn create_scroll(data: SpellData) -> Self {
        BaseType::Scroll(SpellType::from_spell_data(data))
    }

    pub fn create_power(_spell: SpellData) -> Self {
        // todo
        BaseType::Power
    }

    pub fn create_shout(data: SpellData, form_string: String) -> Self {
        let variant = ShoutVariant::from_spell_data(data, form_string);
        BaseType::Shout(variant)
    }

    pub fn classify(category: ItemCategory, keywords: Vec<String>, twohanded: bool) -> Self {
        match category {
            ItemCategory::Ammo => Self::Ammo(AmmoType::classify(keywords.clone(), twohanded)),
            ItemCategory::Armor => Self::Armor(ArmorType::classify(keywords.clone(), twohanded)),
            ItemCategory::Food => Self::Food(FoodType::Fruit), // for now
            ItemCategory::HandToHand => Self::HandToHand,
            ItemCategory::Light => Self::Light, // TODO
            ItemCategory::Potion => Self::Potion(PotionType::Default),
            ItemCategory::Power => Self::Power,
            ItemCategory::Scroll => Self::Scroll(SpellType::default()),
            ItemCategory::Shout => Self::Shout(ShoutVariant::default()),
            ItemCategory::Spell => Self::Spell(SpellType::default()),
            ItemCategory::Weapon => Self::Weapon(WeaponType::classify(keywords.clone(), twohanded)),
            _ => BaseType::Empty,
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
            BaseType::Light => InvColor::Sun.color(),
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
            BaseType::Food(_) => Icon::Food.icon_file(), // TODO
            BaseType::HandToHand => Icon::HandToHand.icon_file(),
            BaseType::Light => Icon::Torch.icon_file(),
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
            BaseType::Food(_) => Icon::IconDefault.icon_file(),
            BaseType::HandToHand => Icon::HandToHand.icon_file(),
            BaseType::Light => Icon::Torch.icon_file(),
            BaseType::Potion(t) => t.icon_fallback(),
            BaseType::PotionProxy(t) => t.icon_fallback(),
            BaseType::Power => Icon::Scroll.icon_file(),
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
            BaseType::Light => true,
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
        !matches!(self, BaseType::Armor(ArmorType::Shield(_, _))) && 
        matches!(
            self,
            BaseType::Potion(_) | BaseType::Armor(_) | BaseType::Food(_)
        )
    }

    fn is_weapon(&self) -> bool {
        matches!(self, BaseType::Weapon(_))
    }

    fn is_one_handed(&self) -> bool {
        match self {
            BaseType::Weapon(t) => match t {
                WeaponType::AxeOneHanded(t, _) => matches!(t, WeaponEquipType::OneHanded),
                WeaponType::AxeTwoHanded(t, _) => matches!(t, WeaponEquipType::OneHanded),
                WeaponType::BowShort(t, _) => matches!(t, WeaponEquipType::OneHanded),
                WeaponType::Bow(t, _) => matches!(t, WeaponEquipType::OneHanded),
                WeaponType::Claw(t, _) => matches!(t, WeaponEquipType::OneHanded),
                WeaponType::Crossbow(t, _) => matches!(t, WeaponEquipType::OneHanded),
                WeaponType::Dagger(t, _) => matches!(t, WeaponEquipType::OneHanded),
                WeaponType::Flail(t, _) => matches!(t, WeaponEquipType::OneHanded),
                WeaponType::Grenade(t, _) => matches!(t, WeaponEquipType::OneHanded),
                WeaponType::Gun(t, _) => matches!(t, WeaponEquipType::OneHanded),
                WeaponType::Halberd(t, _) => matches!(t, WeaponEquipType::OneHanded),
                WeaponType::Hammer(t, _) => matches!(t, WeaponEquipType::OneHanded),
                WeaponType::HandToHand(t, _) => matches!(t, WeaponEquipType::OneHanded),
                WeaponType::Katana(t, _) => matches!(t, WeaponEquipType::OneHanded),
                WeaponType::Lance(t, _) => matches!(t, WeaponEquipType::OneHanded),
                WeaponType::Mace(t, _) => matches!(t, WeaponEquipType::OneHanded),
                WeaponType::Quarterstaff(t, _) => matches!(t, WeaponEquipType::OneHanded),
                WeaponType::Rapier(t, _) => matches!(t, WeaponEquipType::OneHanded),
                WeaponType::Scythe(t, _) => matches!(t, WeaponEquipType::OneHanded),
                WeaponType::Staff(t, _) => matches!(t, WeaponEquipType::OneHanded),
                WeaponType::SwordOneHanded(t, _) => matches!(t, WeaponEquipType::OneHanded),
                WeaponType::SwordTwoHanded(t, _) => matches!(t, WeaponEquipType::OneHanded),
                WeaponType::WeaponDefault(t, _) => matches!(t, WeaponEquipType::OneHanded),
                WeaponType::Whip(t, _) => matches!(t, WeaponEquipType::OneHanded),
            },
            BaseType::Spell(v) => !v.data.twohanded,
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
        self.is_one_handed()
            || matches!(
                self,
                BaseType::Armor(ArmorType::Shield(_, _))
                    | BaseType::Light
                    | BaseType::Scroll(_)
                    | BaseType::Spell(SpellType {
                        data: SpellData {
                            twohanded: false,
                            ..
                        },
                        ..
                    })
            )
    }

    fn right_hand_ok(&self) -> bool {
        self.is_weapon() || matches!(self, BaseType::Scroll(_) | BaseType::Spell(_))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::color::InvColor;
    use crate::data::weapon::WeaponType;

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

        let result = BaseType::classify(ItemCategory::Ammo, input, true);
        assert_eq!(result, BaseType::Ammo(AmmoType::Bullet(InvColor::Water)));

        let input = vec![
            "OCF_InvColorFire".to_string(),
            "OCF_AccessoryBelt".to_string(),
            "Armor".to_string(),
        ];
        let result = BaseType::classify(ItemCategory::Armor, input, false);
        assert_eq!(result, BaseType::Armor(ArmorType::Belt));

        let input = vec![
            "OCF_InvColorFire".to_string(),
            "OCF_WeapTypeLongsword2H".to_string(),
            "Weapon".to_string(),
            "*Longsword".to_string(),
            "-varWeapNotLongsword".to_string(),
            "TwoHandSword".to_string(),
        ];
        let result = BaseType::classify(ItemCategory::Weapon, input, true);
        assert_eq!(
            result,
            BaseType::Weapon(WeaponType::SwordTwoHanded(
                WeaponEquipType::TwoHanded,
                InvColor::Fire
            ))
        );
    }
}

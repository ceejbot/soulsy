//! Base item kinds, from the starting icon set.

use strum::Display;

use super::ammo::AmmoType;
use super::armor::ArmorType;
use super::color::InvColor;
use super::potion::PotionType;
use super::spell::SpellData;
use super::weapon::{WeaponEquipType, WeaponType};
use super::{HasIcon, HasKeywords, IsHudItem};
use crate::plugin::{Color, ItemCategory};
use super::icons::Icon;

#[derive(
   Clone, Debug, Default, Display, Eq, Hash, PartialEq, 
)]
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
    Scroll(SpellData),
    Shout,
    Spell(SpellData),
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
            Proxy::Health => InvColor::OCF_InvColorRed.color(),
            Proxy::Magicka => InvColor::OCF_InvColorBlue.color(),
            Proxy::Stamina => InvColor::OCF_InvColorGreen.color(),
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
    pub fn create_spell(spell: SpellData) -> Self {
        // TODO
        BaseType::Spell(spell)
    }

    pub fn create_scroll(spell: SpellData) -> Self {
        // todo
        BaseType::Scroll(spell)
    }

    pub fn create_power(_spell: SpellData) -> Self {
        // todo
        BaseType::Power
    }

    pub fn create_shout(_spell: SpellData) -> Self {
        // todo (shout data is spell data yes)
        BaseType::Shout
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
            ItemCategory::Scroll => Self::Scroll(SpellData::default()),
            ItemCategory::Shout => Self::Shout,
            ItemCategory::Spell => Self::Spell(SpellData::default()),
            ItemCategory::Weapon => Self::Weapon(WeaponType::classify(keywords.clone(), twohanded)),
            _ => BaseType::Empty,
        }
    }
}

fn color_from_keywords(keywords: Vec<String>) -> InvColor {
    let color_keywords: Vec<InvColor> = keywords
        .iter()
        .filter_map(|xs| InvColor::try_from(xs.as_str()).map_or(None, |color| Some(color)))
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
            BaseType::Light => InvColor::OCF_InvColorSun.color(),
            BaseType::Potion(t) => t.color(),
            BaseType::PotionProxy(t) => t.color(),
            BaseType::Power => Color::default(),
            BaseType::Scroll(t) => t.color(),
            BaseType::Shout => Color::default(),
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
            BaseType::Shout => Icon::Shout.icon_file(),
            BaseType::Spell(t) => t.icon_file(),
            BaseType::Weapon(t) => t.icon_file(),
        }
    }

    fn icon_fallback(&self) -> String {
        match self {
            BaseType::Empty => "icon_default.svg".to_string(),
            BaseType::Ammo(_) => "arrow.svg".to_string(),
            BaseType::Armor(t) => t.icon_fallback(),
            BaseType::Food(_) => "icon_default.svg".to_string(),
            BaseType::HandToHand => "hand_to_hand.svg".to_string(),
            BaseType::Light => "torch.svg".to_string(),
            BaseType::Potion(t) => t.icon_fallback(),
            BaseType::PotionProxy(t) => t.icon_fallback(),
            BaseType::Power => "power.svg".to_string(),
            BaseType::Scroll(_) => "scroll.svg".to_string(),
            BaseType::Shout => "shout.svg".to_string(),
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
            BaseType::Shout => false,
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
        matches!(self, BaseType::Power | BaseType::Shout)
    }

    fn is_spell(&self) -> bool {
        matches!(self, BaseType::Spell(_))
    }

    fn is_utility(&self) -> bool {
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
            BaseType::Spell(v) => !v.twohanded,
            BaseType::Scroll(_) => true,
            _ => false,
        }
    }

    fn two_handed(&self) -> bool {
        !self.is_one_handed()
    }

    fn left_hand_ok(&self) -> bool {
        self.is_one_handed()
            || matches!(
                self,
                BaseType::Armor(ArmorType::Shield(_, _))
                    | BaseType::Light
                    | BaseType::Scroll(_)
                    | BaseType::Spell(SpellData {
                        effect: _,
                        resist: _,
                        twohanded: false,
                        school: _,
                        level: _,
                        archetype: _,
                        variant: _
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
    fn keywords_convert() {
        // All keyword test lists adapted from OCF data.

        let input = vec![
            "Ammo".to_string(),
            "OCF_AmmoTypeBullet1H".to_string(),
            "OCF_AmmoTypeBullet".to_string(),
            "OCF_InvColorWater".to_string(),
            "OCF_AmmoTypeBullet1H_Basic".to_string(),
        ];

        let result = BaseType::classify(input, true);
        assert_eq!(
            result,
            BaseType::Ammo(AmmoType::OCF_AmmoTypeBullet(InvColor::OCF_InvColorWater))
        );

        let input = vec![
            "OCF_InvColorFire".to_string(),
            "OCF_AccessoryBelt".to_string(),
            "Armor".to_string(),
        ];
        let result = BaseType::classify(input, false);
        assert_eq!(result, BaseType::Armor(ArmorType::Belt));

        let input = vec![
            "OCF_InvColorFire".to_string(),
            "OCF_WeapTypeLongsword2H".to_string(),
            "Weapon".to_string(),
            "*Longsword".to_string(),
            "-varWeapNotLongsword".to_string(),
            "TwoHandSword".to_string(),
        ];
        let result = BaseType::classify(input, true);
        assert_eq!(
            result,
            BaseType::Weapon(WeaponType::SwordTwoHanded(
                WeaponEquipType::TwoHanded,
                InvColor::OCF_InvColorFire
            ))
        );
    }
}

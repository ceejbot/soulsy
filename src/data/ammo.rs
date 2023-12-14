#![allow(non_snake_case, non_camel_case_types)]

use strum::Display;

use super::color::InvColor;
use super::{HasIcon, HasKeywords};
use crate::images::icons::Icon;

/// Known ammunition variants. These variants correspond with OCF tags
/// and represent things we are able to categorize. This is a simpler
/// item type so variants map directly to icons, with color hints from
/// OCF being the only other data tracked.
#[derive(Clone, Debug, Display, Eq, Hash, PartialEq)]
pub enum AmmoType {
    Arrow(InvColor),
    Bolt(InvColor),
    Bullet(InvColor),
    Dart(InvColor),
    Slingshot(InvColor),
    Melee(InvColor),
    Grenade(InvColor),
    BodkinArrow(InvColor),
    BroadheadArrow(InvColor),
    HammerheadArrow(InvColor),
    CrescentArrow(InvColor),
    FireArrow(InvColor),
    WhistleArrow(InvColor),
    PractceArrow(InvColor),
}

/// The default ammunition is an arrow drawn in white.
impl Default for AmmoType {
    fn default() -> Self {
        AmmoType::Arrow(InvColor::default())
    }
}

impl HasKeywords for AmmoType {
    /// Use OCF keywords to identify this ammunition type and map it to
    /// one of the enum variants.
    fn classify(_name: &str, keywords: Vec<String>, _ignored: bool) -> Self {
        let color = super::color::color_from_keywords(&keywords);

        let ammo_keywords: Vec<AmmoType> = keywords
            .iter()
            .filter_map(|xs| match xs.as_str() {
                "ArrowBodkin" => Some(Self::BodkinArrow(color.clone())),
                "ArrowBroadhead" => Some(Self::BroadheadArrow(color.clone())),
                "ArrowHammer" => Some(Self::HammerheadArrow(color.clone())),
                "ArrowCrescent" => Some(Self::CrescentArrow(color.clone())),
                "ArrowFire" => Some(Self::FireArrow(color.clone())),
                "ArrowWhistle" => Some(Self::WhistleArrow(color.clone())),
                "ArrowPractice" => Some(Self::PractceArrowArrow(color.clone())),
                "OCF_AmmoTypeArrow" => Some(Self::Arrow(color.clone())),
                "OCF_AmmoTypeBolt" => Some(Self::Bolt(color.clone())),
                "OCF_AmmoTypeBullet" => Some(Self::Bullet(color.clone())),
                "OCF_AmmoTypeDart" => Some(Self::Dart(color.clone())),
                "OCF_AmmoTypeSlingshot" => Some(Self::Slingshot(color.clone())),
                "OCF_WeapTypeMelee" => Some(Self::Melee(color.clone())),
                "WAF_WeapTypeGrenade" => Some(Self::Grenade(color.clone())),
                _ => None,
            })
            .collect();
        if let Some(keyword) = ammo_keywords.first() {
            keyword.clone()
        } else {
            Self::Arrow(color)
        }
    }
}

impl HasIcon for AmmoType {
    fn color(&self) -> crate::plugin::Color {
        match self {
            Self::Arrow(c) => c.color(),
            Self::Bolt(c) => c.color(),
            Self::Bullet(c) => c.color(),
            Self::Dart(c) => c.color(),
            Self::Slingshot(c) => c.color(),
            Self::Melee(c) => c.color(),
            Self::Grenade(c) => c.color(),
            Self::BodkinArrow(c) => c.color(),
            Self::BroadheadArrow(c) => c.color(),
            Self::HammerheadArrow(c) => c.color(),
            Self::CrescentArrow(c) => c.color(),
            Self::FireArrow(c) => c.color(),
            Self::WhistleArrow(c) => c.color(),
            Self::PractceArrow(c) => c.color(),
        }
    }

    fn icon(&self) -> &Icon {
        match self {
            AmmoType::Bullet(_) => &Icon::AmmoBullet,
            AmmoType::Bolt(_) => &Icon::AmmoBolt,
            AmmoType::Dart(_) => &Icon::AmmoDart,
            AmmoType::Slingshot(_) => &Icon::AmmoSlingshot,
            AmmoType::BodkinArrow(_) => &Icon::AmmoBodkinArrow,
            AmmoType::BroadheadArrow(_) => &Icon::AmmoBroadheadArrow,
            AmmoType::HammerheadArrow(_) => &Icon::AmmoHammerheadArrow,
            AmmoType::CrescentArrow(_) => &Icon::AmmoCrescentArrow,
            AmmoType::FireArrow(_) => &Icon::AmmoFireArrow,
            AmmoType::WhistleArrow(_) => &Icon::AmmoWhistleArrow,
            AmmoType::PractceArrow(_) => &Icon::AmmoPractceArrow,
            _ => &Icon::AmmoArrow,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keywords_convert() {
        let input = vec![
            "OCF_InvColorFire".to_string(),
            "OCF_AmmoTypeBullet1H".to_string(),
            "OCF_AmmoTypeBullet".to_string(),
            "OCF_AmmoTypeBullet1H_Basic".to_string(),
        ];

        let result = AmmoType::classify("TestAmmo", input, false);
        assert_eq!(result, AmmoType::Bullet(InvColor::Fire));
    }
}

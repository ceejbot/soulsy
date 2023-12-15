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
    BodkinArrow(InvColor),
    Bolt(InvColor),
    BroadheadArrow(InvColor),
    Bullet(InvColor),
    CrescentArrow(InvColor),
    Dart(InvColor),
    FireArrow(InvColor),
    Grenade(InvColor),
    HammerheadArrow(InvColor),
    Melee(InvColor),
    PracticeArrow(InvColor),
    Slingshot(InvColor),
    WhistleArrow(InvColor),
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
        let color = super::color::color_from_keywords(&keywords).clone();

        let ammo_kinds: Vec<AmmoType> = keywords
            .iter()
            .filter_map(|xs| match xs.as_str() {
                "ArrowBodkin" => Some(Self::BodkinArrow(color.clone().unwrap_or_default())),
                "ArrowBroadhead" => Some(Self::BroadheadArrow(color.clone().unwrap_or_default())),
                "ArrowHammer" => Some(Self::HammerheadArrow(color.clone().unwrap_or_default())),
                "ArrowCrescent" => Some(Self::CrescentArrow(color.clone().unwrap_or_default())),
                "ArrowFire" => Some(Self::FireArrow(
                    color.clone().unwrap_or_else(|| InvColor::Fire),
                )),
                "ArrowWhistle" => Some(Self::WhistleArrow(color.clone().unwrap_or_default())),
                "ArrowPractice" => Some(Self::PracticeArrow(color.clone().unwrap_or_default())),
                "OCF_AmmoTypeArrow" => Some(Self::Arrow(color.clone().unwrap_or_default())),
                "OCF_AmmoTypeBolt" => Some(Self::Bolt(color.clone().unwrap_or_default())),
                "OCF_AmmoTypeBullet" => Some(Self::Bullet(color.clone().unwrap_or_default())),
                "OCF_AmmoTypeDart" => Some(Self::Dart(color.clone().unwrap_or_default())),
                "OCF_AmmoTypeSlingshot" => Some(Self::Slingshot(color.clone().unwrap_or_default())),
                "OCF_WeapTypeMelee" => Some(Self::Melee(color.clone().unwrap_or_default())),
                "WAF_WeapTypeGrenade" => Some(Self::Grenade(color.clone().unwrap_or_default())),
                _ => None,
            })
            .collect();
        if let Some(ammo) = ammo_kinds.first() {
            ammo.clone()
        } else {
            Self::Arrow(color.unwrap_or_default())
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
            Self::PracticeArrow(c) => c.color(),
        }
    }

    fn icon(&self) -> &Icon {
        match self {
            AmmoType::Bullet(_) => &Icon::AmmoBullet,
            AmmoType::Bolt(_) => &Icon::AmmoBolt,
            AmmoType::Dart(_) => &Icon::AmmoDart,
            AmmoType::Slingshot(_) => &Icon::AmmoSlingshot,
            AmmoType::BodkinArrow(_) => &Icon::AmmoArrowBodkin,
            AmmoType::BroadheadArrow(_) => &Icon::AmmoArrowBroadhead,
            AmmoType::HammerheadArrow(_) => &Icon::AmmoArrowHammerhead,
            AmmoType::CrescentArrow(_) => &Icon::AmmoArrowCrescent,
            AmmoType::FireArrow(_) => &Icon::AmmoArrowFire,
            AmmoType::WhistleArrow(_) => &Icon::AmmoArrowWhistle,
            AmmoType::PracticeArrow(_) => &Icon::AmmoArrowPractice,
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

#![allow(non_snake_case, non_camel_case_types)]

use strum::Display;

use super::color::InvColor;
use super::icons::Icon;
use super::{base, HasIcon, HasKeywords};

#[derive(Clone, Debug, Display, Eq, Hash, PartialEq)]
pub enum AmmoType {
    Arrow(InvColor),
    Bolt(InvColor),
    Bullet(InvColor),
    Dart(InvColor),
    Slingshot(InvColor),
    Melee(InvColor),
    Grenade(InvColor),
}

impl Default for AmmoType {
    fn default() -> Self {
        AmmoType::Arrow(InvColor::default())
    }
}

impl HasKeywords for AmmoType {
    fn classify(keywords: Vec<String>, _ignored: bool) -> Self {
        let color = base::color_from_keywords(&keywords);

        let ammo_keywords: Vec<AmmoType> = keywords
            .iter()
            .filter_map(|xs| match xs.as_str() {
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
        }
    }

    fn icon_file(&self) -> String {
        Icon::Arrow.icon_file()
    }

    fn icon_fallback(&self) -> String {
        Icon::Arrow.icon_file()
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

        let result = AmmoType::classify(input, false);
        assert_eq!(result, AmmoType::Bullet(InvColor::Fire));
    }
}

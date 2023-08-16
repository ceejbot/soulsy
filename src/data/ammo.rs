#![allow(non_snake_case, non_camel_case_types)]

use strum::Display;

use super::color::InvColor;
use super::icons::Icon;
use super::{base, HasIcon, HasKeywords};

#[derive(Clone, Debug, Display, Eq, Hash, PartialEq)]
pub enum AmmoType {
    OCF_AmmoTypeArrow(InvColor),
    OCF_AmmoTypeBolt(InvColor),
    OCF_AmmoTypeBullet(InvColor),
    OCF_AmmoTypeDart(InvColor),
    OCF_AmmoTypeSlingshot(InvColor),
    OCF_WeapTypeMelee(InvColor),
    WAF_WeapTypeGrenade(InvColor),
}

impl Default for AmmoType {
    fn default() -> Self {
        AmmoType::OCF_AmmoTypeArrow(InvColor::default())
    }
}

impl HasKeywords for AmmoType {
    fn classify(keywords: Vec<String>, _ignored: bool) -> Self {
        let color = base::color_from_keywords(&keywords);

        let ammo_keywords: Vec<AmmoType> = keywords
            .iter()
            .filter_map(|xs| match xs.as_str() {
                "OCF_AmmoTypeArrow" => Some(Self::OCF_AmmoTypeArrow(color.clone())),
                "OCF_AmmoTypeBolt" => Some(Self::OCF_AmmoTypeBolt(color.clone())),
                "OCF_AmmoTypeBullet" => Some(Self::OCF_AmmoTypeBullet(color.clone())),
                "OCF_AmmoTypeDart" => Some(Self::OCF_AmmoTypeDart(color.clone())),
                "OCF_AmmoTypeSlingshot" => Some(Self::OCF_AmmoTypeSlingshot(color.clone())),
                "OCF_WeapTypeMelee" => Some(Self::OCF_WeapTypeMelee(color.clone())),
                "WAF_WeapTypeGrenade" => Some(Self::WAF_WeapTypeGrenade(color.clone())),
                _ => None,
            })
            .collect();
        if let Some(keyword) = ammo_keywords.first() {
            keyword.clone()
        } else {
            Self::OCF_AmmoTypeArrow(color)
        }
    }
}

impl HasIcon for AmmoType {
    fn color(&self) -> crate::plugin::Color {
        match self {
            Self::OCF_AmmoTypeArrow(c) => c.color(),
            Self::OCF_AmmoTypeBolt(c) => c.color(),
            Self::OCF_AmmoTypeBullet(c) => c.color(),
            Self::OCF_AmmoTypeDart(c) => c.color(),
            Self::OCF_AmmoTypeSlingshot(c) => c.color(),
            Self::OCF_WeapTypeMelee(c) => c.color(),
            Self::WAF_WeapTypeGrenade(c) => c.color(),
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
        assert_eq!(result, AmmoType::OCF_AmmoTypeBullet(InvColor::Fire));
    }
}

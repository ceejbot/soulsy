#![allow(non_snake_case, non_camel_case_types)]

use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use strum::Display;

use super::game_enums::ActorValue;
use super::spell::MagicDamageType;
use super::{HasIcon, InvColor};
use crate::plugin::Color;

#[derive(
    Decode, Encode, Clone, Debug, Default, Deserialize, Display, Hash, Eq, PartialEq, Serialize,
)]
#[strum(serialize_all = "snake_case")]
pub enum PotionType {
    #[default]
    Default,
    Poison,
    Resist(MagicDamageType),
    Health,
    Magicka,
    Stamina,
}

impl PotionType {
    pub fn from_effect(is_poison: bool, effect: ActorValue) -> Self {
        if is_poison {
            return PotionType::Poison;
        }

        match effect {
            ActorValue::Health => PotionType::Health,
            ActorValue::HealRateMult => PotionType::Health,
            ActorValue::HealRate => PotionType::Health,
            ActorValue::Stamina => PotionType::Stamina,
            ActorValue::StaminaRateMult => PotionType::Stamina,
            ActorValue::StaminaRate => PotionType::Stamina,
            ActorValue::Magicka => PotionType::Magicka,
            ActorValue::MagickaRateMult => PotionType::Magicka,
            ActorValue::MagickaRate => PotionType::Magicka,
            ActorValue::ResistFire => PotionType::Resist(MagicDamageType::Fire),
            ActorValue::ResistFrost => PotionType::Resist(MagicDamageType::Frost),
            ActorValue::ResistShock => PotionType::Resist(MagicDamageType::Shock),
            ActorValue::ResistMagic => PotionType::Resist(MagicDamageType::Magic),
            ActorValue::ResistDisease => PotionType::Resist(MagicDamageType::Disease),
            ActorValue::PoisonResist => PotionType::Resist(MagicDamageType::Poison),
            _ => PotionType::Default,
        }
    }
}

impl HasIcon for PotionType {
    fn color(&self) -> Color {
        match self {
            PotionType::Poison => InvColor::OCF_InvColorPoison.color(),
            PotionType::Resist(t) => match t {
                MagicDamageType::Disease => Color::default(), // TODO
                MagicDamageType::Fire => InvColor::OCF_InvColorFire.color(),
                MagicDamageType::Frost => InvColor::OCF_InvColorFrost.color(),
                MagicDamageType::Magic => InvColor::OCF_InvColorBlue.color(),
                MagicDamageType::Poison => InvColor::OCF_InvColorPoison.color(),
                MagicDamageType::Shock => InvColor::OCF_InvColorShock.color(),
                MagicDamageType::Sun => InvColor::OCF_InvColorSun.color(),
                MagicDamageType::None => Color::default(),
            },
            PotionType::Health => InvColor::OCF_InvColorRed.color(),
            PotionType::Magicka => InvColor::OCF_InvColorBlue.color(),
            PotionType::Stamina => InvColor::OCF_InvColorGreen.color(),
            PotionType::Default => Color::default(),
        }
    }

    fn icon_file(&self) -> String {
        match self {
            PotionType::Poison => "potion_poison_default.svg".to_string(),
            PotionType::Resist(t) => match t {
                MagicDamageType::Disease => "potion_resist.svg".to_string(),
                MagicDamageType::Fire => "potion_resist_fire.svg".to_string(),
                MagicDamageType::Frost => "potion_resist_frost.svg".to_string(),
                MagicDamageType::Magic => "potion_resist_magic.svg".to_string(),
                MagicDamageType::Poison => "potion_resist.svg".to_string(),
                MagicDamageType::Shock => "potion_resist_shock.svg".to_string(),
                MagicDamageType::Sun => "potion_resist_fire.svg".to_string(),
                MagicDamageType::None => "potion_resist.svg".to_string(),
            },
            PotionType::Health => "potion_health.svg".to_string(),
            PotionType::Magicka => "potion_magicka.svg".to_string(),
            PotionType::Stamina => "potion_stamina.svg".to_string(),
            _ => format!("potion_default.svg"),
        }
    }

    fn icon_fallback(&self) -> String {
        "potion_default.svg".to_string()
    }
}

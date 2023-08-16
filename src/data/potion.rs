#![allow(non_snake_case, non_camel_case_types)]

use strum::Display;

use super::game_enums::ActorValue;
use super::icons::Icon;
use super::spell::MagicDamageType;
use super::{HasIcon, InvColor};
use crate::plugin::Color;

#[derive(Clone, Debug, Default, Display, Hash, Eq, PartialEq)]
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
            _ => {
                log::debug!("default potion type; effect={effect};");
                PotionType::Default
            }
        }
    }
}

impl HasIcon for PotionType {
    fn color(&self) -> Color {
        match self {
            PotionType::Poison => InvColor::Poison.color(),
            PotionType::Resist(t) => match t {
                MagicDamageType::Disease => Color::default(), // TODO
                MagicDamageType::Fire => InvColor::Fire.color(),
                MagicDamageType::Frost => InvColor::Frost.color(),
                MagicDamageType::Magic => InvColor::Blue.color(),
                MagicDamageType::Poison => InvColor::Poison.color(),
                MagicDamageType::Shock => InvColor::Shock.color(),
                MagicDamageType::Sun => InvColor::Sun.color(),
                MagicDamageType::None => Color::default(),
            },
            PotionType::Health => InvColor::Red.color(),
            PotionType::Magicka => InvColor::Blue.color(),
            PotionType::Stamina => InvColor::Green.color(),
            PotionType::Default => Color::default(),
        }
    }

    fn icon_file(&self) -> String {
        match self {
            PotionType::Poison => Icon::PotionPoison.icon_file(),
            PotionType::Resist(t) => match t {
                MagicDamageType::Disease => Icon::PotionResist.icon_file(),
                MagicDamageType::Fire => Icon::PotionResistFire.icon_file(),
                MagicDamageType::Frost => Icon::PotionResistFrost.icon_file(),
                MagicDamageType::Magic => Icon::PotionResist.icon_file(),
                MagicDamageType::Poison => Icon::PotionResist.icon_file(),
                MagicDamageType::Shock => Icon::PotionResistShock.icon_file(),
                MagicDamageType::Sun => Icon::PotionResistFire.icon_file(),
                MagicDamageType::None => Icon::PotionResist.icon_file(),
            },
            PotionType::Health => Icon::PotionHealth.icon_file(),
            PotionType::Magicka => Icon::PotionMagicka.icon_file(),
            PotionType::Stamina => Icon::PotionStamina.icon_file(),
            _ => Icon::PotionDefault.icon_file(),
        }
    }

    fn icon_fallback(&self) -> String {
        Icon::PotionDefault.icon_file()
    }
}

#![allow(non_snake_case, non_camel_case_types)]

use strum::Display;

use super::game_enums::ActorValue;
use super::icons::Icon;
use super::magic::MagicColor;
use super::{HasIcon, InvColor};
use crate::plugin::Color;

#[derive(Clone, Debug, Default, Display, Hash, Eq, PartialEq)]
#[strum(serialize_all = "snake_case")]
pub enum PotionType {
    #[default]
    Default,
    Poison,
    Resist(MagicColor),
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
            ActorValue::ResistFire => PotionType::Resist(MagicColor::Fire),
            ActorValue::ResistFrost => PotionType::Resist(MagicColor::Frost),
            ActorValue::ResistShock => PotionType::Resist(MagicColor::Shock),
            ActorValue::ResistMagic => PotionType::Resist(MagicColor::Magic),
            ActorValue::ResistDisease => PotionType::Resist(MagicColor::Disease),
            ActorValue::PoisonResist => PotionType::Resist(MagicColor::Poison),
            _ => {
                log::debug!("Falling back to default potion type; effect={effect}");
                PotionType::Default
            }
        }
    }
}

impl HasIcon for PotionType {
    fn color(&self) -> Color {
        match self {
            PotionType::Poison => InvColor::Poison.color(),
            PotionType::Resist(t) => t.color().color(),
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
                MagicColor::Disease => Icon::PotionResist.icon_file(),
                MagicColor::Fire => Icon::PotionResistFire.icon_file(),
                MagicColor::Frost => Icon::PotionResistFrost.icon_file(),
                MagicColor::Magic => Icon::PotionResist.icon_file(),
                MagicColor::Poison => Icon::PotionResist.icon_file(),
                MagicColor::Shock => Icon::PotionResistShock.icon_file(),
                MagicColor::Sun => Icon::PotionResistFire.icon_file(),
                _ => Icon::PotionResist.icon_file(),
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

/*

// alchemy keywords
MagicAlchBeneficial,
MagicAlchDamageHealth,
// lots of these; TODO


*/

#![allow(non_snake_case, non_camel_case_types)]

use strum::Display;

use super::game_enums::ActorValue;
use super::magic::MagicCategory;
use super::{HasIcon, InvColor};
use crate::images::icons::Icon;
use crate::plugin::Color;

#[derive(Clone, Debug, Default, Display, Hash, Eq, PartialEq)]
#[strum(serialize_all = "snake_case")]
pub enum PotionType {
    #[default]
    Default,
    Poison,
    Resist(MagicCategory),
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
            ActorValue::ResistFire => PotionType::Resist(MagicCategory::Fire),
            ActorValue::ResistFrost => PotionType::Resist(MagicCategory::Frost),
            ActorValue::ResistShock => PotionType::Resist(MagicCategory::Shock),
            ActorValue::ResistMagic => PotionType::Resist(MagicCategory::Magic),
            ActorValue::ResistDisease => PotionType::Resist(MagicCategory::Disease),
            ActorValue::PoisonResist => PotionType::Resist(MagicCategory::Poison),
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

    fn icon(&self) -> &Icon {
        match self {
            PotionType::Poison => &Icon::PotionPoison,
            PotionType::Resist(t) => match t {
                MagicCategory::Disease => &Icon::PotionResist,
                MagicCategory::Fire => &Icon::PotionResistFire,
                MagicCategory::Frost => &Icon::PotionResistFrost,
                MagicCategory::Magic => &Icon::PotionResist,
                MagicCategory::Poison => &Icon::PotionResist,
                MagicCategory::Shock => &Icon::PotionResistShock,
                MagicCategory::Sun => &Icon::PotionResistFire,
                _ => &Icon::PotionResist,
            },
            PotionType::Health => &Icon::PotionHealth,
            PotionType::Magicka => &Icon::PotionMagicka,
            PotionType::Stamina => &Icon::PotionStamina,
            _ => &Icon::PotionDefault,
        }
    }
}

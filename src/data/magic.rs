use strum::Display;

use super::color::InvColor;
use super::game_enums::{ActorValue, SpellArchetype};
use crate::images::icons::Icon;

#[derive(Default, Clone, Debug, Eq, Hash, PartialEq)]
pub struct SpellData {
    pub hostile: bool,
    pub damage: MagicCategory,
    pub twohanded: bool,
    pub school: School,
    pub level: MagicSpellLevel,
    pub archetype: SpellArchetype,
}

impl SpellData {
    pub fn new(
        hostile: bool,
        resist: i32,
        twohanded: bool,
        school: i32,
        level: u32,
        archetype: i32,
    ) -> Self {
        let school = School::from(school);
        let resist = ActorValue::from(resist);
        let archetype = SpellArchetype::from(archetype);

        let damage = match resist {
            ActorValue::ResistFire => MagicCategory::Fire,
            ActorValue::ResistFrost => MagicCategory::Frost,
            ActorValue::ResistShock => MagicCategory::Shock,
            ActorValue::ResistMagic => MagicCategory::Arcane,
            ActorValue::ResistDisease => MagicCategory::Disease,
            ActorValue::PoisonResist => MagicCategory::Poison,
            _ => MagicCategory::None,
        };

        Self {
            hostile,
            twohanded,
            school,
            archetype,
            level: level.into(),
            damage,
        }
    }
}

#[derive(Clone, Debug, Default, Display, Hash, Eq, PartialEq)]
pub enum MagicCategory {
    #[default]
    None,
    Arcane,
    ArcaneFire,
    Ashfire,
    Astral,
    Bleed,
    Disease,
    Earth,
    Fire,
    Frost,
    FrostFire,
    Lunar,
    Magic,
    Necrotic,
    Poison,
    Shadow,
    Shock,
    ShockArc,
    Stamina,
    Sun,
    Water,
    Wind,
}

impl MagicCategory {
    pub fn color(&self) -> InvColor {
        match self {
            MagicCategory::None => InvColor::default(),
            MagicCategory::Arcane => InvColor::Blue,
            MagicCategory::ArcaneFire => InvColor::Pink,
            MagicCategory::Ashfire => InvColor::Ash,
            MagicCategory::Astral => InvColor::Silver,
            MagicCategory::Bleed => InvColor::Blood,
            MagicCategory::Disease => InvColor::Green,
            MagicCategory::Earth => InvColor::Brown,
            MagicCategory::Fire => InvColor::Fire,
            MagicCategory::Frost => InvColor::Frost,
            MagicCategory::FrostFire => InvColor::Frost,
            MagicCategory::Lunar => InvColor::Silver,
            MagicCategory::Magic => InvColor::Blue,
            MagicCategory::Necrotic => InvColor::Purple,
            MagicCategory::Poison => InvColor::Poison,
            MagicCategory::Shadow => InvColor::Purple,
            MagicCategory::Shock => InvColor::Shock,
            MagicCategory::ShockArc => InvColor::Water,
            MagicCategory::Stamina => InvColor::Green,
            MagicCategory::Sun => InvColor::Sun,
            MagicCategory::Water => InvColor::Water,
            MagicCategory::Wind => InvColor::Gray,
        }
    }

    pub fn icon(&self) -> Option<Icon> {
        match self {
            MagicCategory::Arcane => Some(Icon::SpellStars),
            MagicCategory::ShockArc => Some(Icon::SpellArclight),
            MagicCategory::Astral => Some(Icon::SpellStars),
            MagicCategory::Bleed => Some(Icon::SpellBleed),
            MagicCategory::FrostFire => Some(Icon::SpellFire),
            MagicCategory::Earth => Some(Icon::SpellEarth),
            MagicCategory::Fire => Some(Icon::SpellFire),
            MagicCategory::Frost => Some(Icon::SpellFrost),
            MagicCategory::Lunar => Some(Icon::SpellMoon),
            MagicCategory::Necrotic => Some(Icon::SpellDesecration),
            MagicCategory::Poison => Some(Icon::SpellPoison),
            MagicCategory::Shadow => Some(Icon::SpellShadow),
            MagicCategory::Shock => Some(Icon::SpellShock),
            MagicCategory::Sun => Some(Icon::SpellSun),
            MagicCategory::Water => Some(Icon::SpellWater),
            MagicCategory::Wind => Some(Icon::SpellWind),
            _ => None,
        }
    }
}

#[derive(Debug, Default, Clone, Hash, Display, PartialEq, Eq)]
#[strum(serialize_all = "lowercase")]
pub enum School {
    Alteration = 18,
    Conjuration,
    Destruction,
    Illusion,
    Restoration,
    #[default]
    None,
}

impl From<i32> for School {
    fn from(value: i32) -> Self {
        match value {
            18 => School::Alteration,
            19 => School::Conjuration,
            20 => School::Destruction,
            21 => School::Illusion,
            22 => School::Restoration,
            _ => School::None,
        }
    }
}

#[derive(Debug, Default, Clone, Hash, Display, PartialEq, Eq)]
#[strum(serialize_all = "lowercase")]
pub enum MagicSpellLevel {
    #[default]
    Novice,
    Apprentice,
    Adept,
    Master,
    Expert,
}

impl From<u32> for MagicSpellLevel {
    fn from(skill_level: u32) -> Self {
        if skill_level >= 100 {
            MagicSpellLevel::Master
        } else if skill_level >= 75 {
            MagicSpellLevel::Expert
        } else if skill_level >= 50 {
            MagicSpellLevel::Adept
        } else if skill_level >= 25 {
            MagicSpellLevel::Apprentice
        } else {
            MagicSpellLevel::Novice
        }
    }
}

#[derive(Debug, Clone, Hash, Display, PartialEq, Eq)]
pub enum CastingType {
    ConstantEffect,
    FireAndForget,
    Concentration,
    Scroll,
}

impl From<u32> for CastingType {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::ConstantEffect,
            1 => Self::FireAndForget,
            2 => Self::Concentration,
            _ => Self::Scroll,
        }
    }
}

#[derive(Debug, Clone, Hash, Display, PartialEq, Eq)]
pub enum Delivery {
    Player,
    Touch,
    Aimed,
    TargetActor,
    TargetLocation,
}

impl From<u32> for Delivery {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::Player,
            1 => Self::Touch,
            2 => Self::Aimed,
            3 => Self::TargetActor,
            _ => Self::TargetLocation, // surely this won't burn me. surely!
        }
    }
}

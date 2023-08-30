use strum::Display;

use super::color::InvColor;
use super::game_enums::{ActorValue, SpellArchetype};
use super::icons::Icon;
use super::HasIcon;
use crate::plugin::Color;

#[derive(Default, Clone, Debug, Eq, Hash, PartialEq)]
pub struct SpellData {
    pub hostile: bool,
    pub twohanded: bool,
    pub school: School,
    pub level: MagicSpellLevel,
    pub archetype: SpellArchetype,
    pub damage: MagicColor,
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
            ActorValue::ResistFire => MagicColor::Fire,
            ActorValue::ResistFrost => MagicColor::Frost,
            ActorValue::ResistShock => MagicColor::Shock,
            ActorValue::ResistMagic => MagicColor::Arcane,
            ActorValue::ResistDisease => MagicColor::Disease,
            ActorValue::PoisonResist => MagicColor::Poison,
            _ => MagicColor::None,
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
pub enum MagicColor {
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

impl MagicColor {
    pub fn color(&self) -> InvColor {
        match self {
            MagicColor::None => InvColor::default(),
            MagicColor::Arcane => InvColor::Water,
            MagicColor::ArcaneFire => InvColor::Blue,
            MagicColor::Ashfire => InvColor::Ash,
            MagicColor::Astral => InvColor::Silver,
            MagicColor::Bleed => InvColor::Blood,
            MagicColor::Disease => InvColor::Green,
            MagicColor::Earth => InvColor::Brown,
            MagicColor::Fire => InvColor::Fire,
            MagicColor::Frost => InvColor::Frost,
            MagicColor::FrostFire => InvColor::Frost,
            MagicColor::Lunar => InvColor::Silver,
            MagicColor::Magic => InvColor::Blue,
            MagicColor::Necrotic => InvColor::Eldritch,
            MagicColor::Poison => InvColor::Poison,
            MagicColor::Shadow => InvColor::Purple,
            MagicColor::Shock => InvColor::Shock,
            MagicColor::ShockArc => InvColor::Water,
            MagicColor::Stamina => InvColor::Green,
            MagicColor::Sun => InvColor::Sun,
            MagicColor::Water => InvColor::Water,
            MagicColor::Wind => InvColor::Gray,
        }
    }

    pub fn icon(&self) -> Option<Icon> {
        match self {
            MagicColor::Arcane => Some(Icon::SpellAstral),
            MagicColor::ShockArc => Some(Icon::SpellArclight),
            MagicColor::Astral => Some(Icon::SpellAstral),
            MagicColor::Bleed => Some(Icon::SpellBleed),
            MagicColor::FrostFire => Some(Icon::SpellFire),
            MagicColor::Earth => Some(Icon::SpellEarth),
            MagicColor::Fire => Some(Icon::SpellFire),
            MagicColor::Frost => Some(Icon::SpellFrost),
            MagicColor::Lunar => Some(Icon::SpellMoon),
            MagicColor::Necrotic => Some(Icon::SpellNecrotic),
            MagicColor::Poison => Some(Icon::SpellPoison),
            MagicColor::Shadow => Some(Icon::SpellShadow),
            MagicColor::Shock => Some(Icon::SpellShock),
            MagicColor::Sun => Some(Icon::SpellSun),
            MagicColor::Water => Some(Icon::SpellWater),
            MagicColor::Wind => Some(Icon::SpellWind),
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

impl HasIcon for School {
    fn color(&self) -> Color {
        Color::default()
    }

    fn icon_file(&self) -> String {
        match self {
            School::Alteration => Icon::Alteration.icon_file(),
            School::Conjuration => Icon::Conjuration.icon_file(),
            School::Destruction => Icon::Destruction.icon_file(),
            School::Illusion => Icon::Illusion.icon_file(),
            School::Restoration => Icon::Restoration.icon_file(),
            School::None => Icon::IconDefault.icon_file(),
        }
    }

    fn icon_fallback(&self) -> String {
        Icon::IconDefault.icon_file()
    }
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

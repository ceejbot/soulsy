use strum::Display;

use super::color::InvColor;
use super::game_enums::{ActorValue, SpellArchetype};
use super::icons::Icon;
use super::HasIcon;
use crate::plugin::Color;

#[derive(Default, Clone, Debug, Eq, Hash, PartialEq)]
pub struct SpellData {
    pub hostile: bool,
    pub effect: ActorValue,
    pub secondary: ActorValue,
    pub twohanded: bool,
    pub school: School,
    pub level: MagicSpellLevel,
    pub archetype: SpellArchetype,
    pub damage: MagicDamageType,
    pub associated: String,
}

impl SpellData {
    pub fn new(
        hostile: bool,
        effect: i32,
        effect2: i32,
        resist: i32,
        twohanded: bool,
        school: i32,
        level: u32,
        archetype: i32,
        associated: String,
    ) -> Self {
        let school = School::from(school);
        let effect = ActorValue::from(effect);
        let secondary = ActorValue::from(effect2);
        let resist = ActorValue::from(resist);
        let archetype = SpellArchetype::from(archetype);

        let damage = match resist {
            ActorValue::ResistFire => MagicDamageType::Fire,
            ActorValue::ResistFrost => MagicDamageType::Frost,
            ActorValue::ResistShock => MagicDamageType::Shock,
            ActorValue::ResistMagic => MagicDamageType::Magic,
            ActorValue::ResistDisease => MagicDamageType::Disease,
            ActorValue::PoisonResist => MagicDamageType::Poison,
            // ActorValue::SOMETHING => MagicDamageType::Sun, // TODO SSEdit inspection
            _ => MagicDamageType::None,
        };

        Self {
            hostile,
            effect,
            secondary,
            twohanded,
            school,
            archetype,
            level: level.into(),
            damage,
            associated: associated.clone(),
        }
    }
}

#[derive(Clone, Debug, Default, Display, Hash, Eq, PartialEq)]
pub enum MagicDamageType {
    #[default]
    None,
    Bleed,
    Disease,
    Earth,
    Fire,
    Frost,
    Lunar,
    Magic,
    Other,
    Poison,
    Shock,
    Stamina,
    Sun,
    Water,
    Wind,
}

impl HasIcon for MagicDamageType {
    fn color(&self) -> Color {
        match self {
            MagicDamageType::None => Color::default(),
            MagicDamageType::Bleed => InvColor::Blood.color(),
            MagicDamageType::Disease => InvColor::Green.color(),
            MagicDamageType::Earth => InvColor::Brown.color(),
            MagicDamageType::Fire => InvColor::Fire.color(),
            MagicDamageType::Frost => InvColor::Frost.color(),
            MagicDamageType::Lunar => InvColor::Silver.color(),
            MagicDamageType::Magic => InvColor::Blue.color(),
            MagicDamageType::Other => InvColor::Eldritch.color(),
            MagicDamageType::Poison => InvColor::Poison.color(),
            MagicDamageType::Shock => InvColor::Shock.color(),
            MagicDamageType::Stamina => InvColor::Green.color(),
            MagicDamageType::Sun => InvColor::Sun.color(),
            MagicDamageType::Water => InvColor::Water.color(),
            MagicDamageType::Wind => InvColor::Gray.color(),
        }
    }

    fn icon_file(&self) -> String {
        match self {
            // These spells have ONLY damage type as their distinguisher.
            MagicDamageType::None => self.icon_fallback(),
            MagicDamageType::Bleed => Icon::SpellBleed.icon_file(),
            MagicDamageType::Disease => self.icon_fallback(),
            MagicDamageType::Earth => Icon::SpellEarth.icon_file(),
            MagicDamageType::Fire => Icon::SpellFire.icon_file(),
            MagicDamageType::Frost => Icon::SpellFrost.icon_file(),
            MagicDamageType::Lunar => self.icon_fallback(), // Icon::SpellMoon.icon_file(),
            MagicDamageType::Magic => self.icon_fallback(),
            MagicDamageType::Other => Icon::SpellBolt.icon_file(),
            MagicDamageType::Poison => Icon::SpellPoison.icon_file(),
            MagicDamageType::Shock => Icon::SpellShock.icon_file(),
            MagicDamageType::Stamina => self.icon_fallback(),
            MagicDamageType::Sun => Icon::SpellSun.icon_file(),
            MagicDamageType::Water => Icon::SpellWater.icon_file(),
            MagicDamageType::Wind => Icon::SpellWind.icon_file(),
        }
    }

    fn icon_fallback(&self) -> String {
        Icon::Destruction.icon_file()
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

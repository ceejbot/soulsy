#![allow(non_snake_case, non_camel_case_types)]

use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use strum::Display;

use super::color::InvColor;
use super::game_enums::{ActorValue, SpellArchetype};
use super::HasIcon;
use crate::plugin::Color;

// Spells must be classified by querying game data about actor values, resist types,
// and spell archetypes. SpellData holds Rust expressions of the C++ enum values.
// In all cases, we choose the primary actor value from the most expensive effect
// of a spell or potion.

#[derive(Decode, Default, Encode, Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct SpellData {
    pub effect: ActorValue,
    pub resist: ActorValue,
    pub twohanded: bool,
    pub school: School,
    pub level: MagicSpellLevel,
    pub archetype: SpellArchetype,
    pub variant: SpellVariant,
}

impl SpellData {
    pub fn from_game_data(
        effect: i32,
        resist: i32,
        twohanded: bool,
        school: i32,
        level: u32,
        archetype: i32,
    ) -> Self {
        let school = School::from(school);
        let effect = ActorValue::from(effect);
        let resist = ActorValue::from(resist);
        let archetype = SpellArchetype::from(archetype);

        let damage = match resist {
            ActorValue::ResistFire => MagicDamageType::Fire,
            ActorValue::ResistFrost => MagicDamageType::Frost,
            ActorValue::ResistShock => MagicDamageType::Shock,
            ActorValue::ResistMagic => MagicDamageType::Magic,
            ActorValue::ResistDisease => MagicDamageType::Disease,
            ActorValue::PoisonResist => MagicDamageType::Poison,
            // sun damage is in here somewhere -- is it healing? todo SSEdit
            _ => MagicDamageType::None,
        };

        // well, this will be fun™
        let variant = match archetype {
            SpellArchetype::ValueModifier => {
                if matches!(effect, ActorValue::Health) && matches!(school, School::Restoration) {
                    Some(SpellVariant::Heal)
                } else {
                    None
                }
            }
            SpellArchetype::CureDisease => Some(SpellVariant::Cure),
            // SpellArchetype::Calm => SpellVariant::Calm, //do I have one?
            SpellArchetype::Demoralize => Some(SpellVariant::Demoralize),
            // SpellArchetype::Invisibility => todo!(),
            SpellArchetype::Light => Some(SpellVariant::Light),
            // SpellArchetype::NightEye => todo!(),
            SpellArchetype::BoundWeapon => Some(SpellVariant::BoundWeapon),
            SpellArchetype::SummonCreature => Some(SpellVariant::Summon),
            SpellArchetype::DetectLife => Some(SpellVariant::Detect),
            //SpellArchetype::Paralysis => todo!(),
            SpellArchetype::Reanimate => Some(SpellVariant::Reanimate),
            SpellArchetype::SoulTrap => Some(SpellVariant::SoulTrap),
            SpellArchetype::Guide => Some(SpellVariant::Guide),
            // SpellArchetype::CurePoison => todo!(),
            // SpellArchetype::Etherealize => todo!(),
            _ => Some(SpellVariant::Damage(damage.clone())),
        };

        let variant = if let Some(v) = variant {
            v
        } else {
            SpellVariant::Unknown
        };

        Self {
            effect,
            resist,
            twohanded,
            school,
            archetype,
            level: level.into(),
            variant,
        }
    }
}

impl HasIcon for SpellData {
    fn color(&self) -> Color {
        match self.school {
            School::Alteration => Color::default(),
            School::Conjuration => Color::default(),
            School::Destruction => match self.resist {
                ActorValue::ResistFire => InvColor::OCF_InvColorFire.color(),
                ActorValue::ResistFrost => InvColor::OCF_InvColorFrost.color(),
                ActorValue::ResistShock => InvColor::OCF_InvColorShock.color(),
                ActorValue::ResistMagic => InvColor::OCF_InvColorBlue.color(),
                ActorValue::ResistDisease => InvColor::OCF_InvColorPoison.color(),
                _ => Color::default(),
            },
            School::Illusion => Color::default(),
            School::Restoration => Color::default(),
            School::None => Color::default(),
        }
    }

    fn icon_file(&self) -> String {
        match self.school {
            School::Alteration => "alteration.svg".to_string(),
            School::Conjuration => "conjuration.svg".to_string(),
            School::Destruction => "destruction.svg".to_string(),
            School::Illusion => "illusion.svg".to_string(),
            School::Restoration => "restoration.svg".to_string(),
            School::None => "icon_default.svg".to_string(),
        }
    }

    fn icon_fallback(&self) -> String {
        format!("{}.svg", self.school)
    }
}

#[derive(
    Decode, Encode, Deserialize, Serialize, Debug, Default, Clone, Hash, Display, PartialEq, Eq,
)]
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
            20 => School::Conjuration,
            21 => School::Illusion,
            22 => School::Restoration,
            _ => School::None,
        }
    }
}

#[derive(
    Decode, Encode, Deserialize, Serialize, Debug, Default, Clone, Hash, Display, PartialEq, Eq,
)]
#[strum(serialize_all = "lowercase")]
pub enum MagicSpellLevel {
    #[default]
    Novice,
    Apprentice,
    Adept,
    Master,
    Expert,
}

#[derive(Decode, Encode, Clone, Debug, Deserialize, Display, Hash, Eq, PartialEq, Serialize)]
pub enum MagicDamageType {
    None,
    Disease,
    Fire,
    Frost,
    Magic,
    Poison,
    Shock,
    Sun,
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

// Some magic overhauls move spells from one school to another, so this
// classification should be used for all schools even if you reasonably think
// that healing spells will never be destruction spells. Also, this is as
// ad-hoc as the game spell types themselves.
#[derive(Decode, Encode, Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum SpellVariant {
    #[default]
    Unknown,
    BoundWeapon,
    Burden,
    Cure,
    Damage(MagicDamageType),
    Demoralize,
    Detect,
    CarryWeight, // feather
    Guide,
    Heal,
    Light,
    // Muffle,
    Reanimate,
    Reflect,
    Rune,
    SoulTrap,
    Summon,
    Teleport,
    // Transmute,
    Ward,
    // Waterbreathing,
    // Waterwalking,
}

/*
alteration.svg
alteration_detect.svg
alteration_feather.svg
alteration_light.svg
alteration_wind.svg
conjuration.svg
conjuration_skeleton.svg
conjuration_soultrap.svg
spell_reanimate.svg
destruction.svg
destruction_fire.svg
destruction_fire_adept.svg
destruction_fire_apprentice.svg
destruction_fire_expert.svg
destruction_fire_master.svg
destruction_frost.svg
destruction_frost_adept.svg
destruction_frost_apprentice.svg
destruction_frost_expert.svg
destruction_frost_master.svg
destruction_shock.svg
destruction_shock_adept.svg
destruction_shock_apprentice.svg
destruction_shock_expert.svg
destruction_shock_master.svg
illusion.svg
illusion_demoralize.svg
restoration.svg
restoration_cure.svg
restoration_heal.svg
restoration_poison.svg
restoration_sundamage.svg
restoration_ward.svg
spell_rune.svg
spell_reflect.svg
spell_teleport.svg

*/

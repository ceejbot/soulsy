#![allow(non_snake_case, non_camel_case_types)]

use strum::Display;

use super::color::InvColor;
use super::game_enums::{ActorValue, SpellArchetype};
use super::icons::Icon;
use super::HasIcon;
use crate::plugin::Color;

// Spells must be classified by querying game data about actor values, resist types,
// and spell archetypes. SpellData holds Rust expressions of the C++ enum values.
// In all cases, we choose the primary actor value from the most expensive effect
// of a spell or potion.

#[derive(Default, Clone, Debug, Eq, Hash, PartialEq)]
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
            SpellArchetype::CurePoison => Some(SpellVariant::Cure),
            SpellArchetype::CureParalysis => Some(SpellVariant::Cure),
            // SpellArchetype::Calm => SpellVariant::Calm, //do I have one?
            SpellArchetype::Demoralize => Some(SpellVariant::Demoralize),
            // SpellArchetype::Invisibility => todo!(),
            SpellArchetype::Light => Some(SpellVariant::Light),
            // SpellArchetype::NightEye => todo!(),
            SpellArchetype::BoundWeapon => Some(SpellVariant::BoundWeapon(BoundType::Unknown)),
            SpellArchetype::SummonCreature => Some(SpellVariant::Summon),
            SpellArchetype::DetectLife => Some(SpellVariant::Detect),
            //SpellArchetype::Paralysis => todo!(),
            SpellArchetype::Reanimate => Some(SpellVariant::Reanimate),
            SpellArchetype::SoulTrap => Some(SpellVariant::SoulTrap),
            SpellArchetype::Guide => Some(SpellVariant::Guide),
            //SpellArchetype::Etherealize => todo!(),
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
        match &self.variant {
            SpellVariant::Unknown => Color::default(),
            SpellVariant::BoundWeapon(_) => InvColor::OCF_InvColorEldritch.color(),
            SpellVariant::Burden => Color::default(),
            SpellVariant::Cure => InvColor::OCF_InvColorGreen.color(),
            SpellVariant::Damage(t) => match t {
                MagicDamageType::None => Color::default(),
                MagicDamageType::Disease => InvColor::OCF_InvColorGreen.color(),
                MagicDamageType::Fire => InvColor::OCF_InvColorFire.color(),
                MagicDamageType::Frost => InvColor::OCF_InvColorFrost.color(),
                MagicDamageType::Magic => InvColor::OCF_InvColorBlue.color(),
                MagicDamageType::Poison => InvColor::OCF_InvColorPoison.color(),
                MagicDamageType::Shock => InvColor::OCF_InvColorShock.color(),
                MagicDamageType::Sun => InvColor::OCF_InvColorSun.color(),
            },
            SpellVariant::Demoralize => Color::default(),
            SpellVariant::Detect => Color::default(),
            SpellVariant::CarryWeight => Color::default(),
            SpellVariant::Guide => InvColor::OCF_InvColorEldritch.color(),
            SpellVariant::Heal => InvColor::OCF_InvColorGreen.color(),
            SpellVariant::Light => InvColor::OCF_InvColorEldritch.color(),
            SpellVariant::Reanimate => Color::default(),
            SpellVariant::Reflect => Color::default(),
            SpellVariant::Rune => Color::default(),
            SpellVariant::SoulTrap => InvColor::OCF_InvColorEldritch.color(),
            SpellVariant::Summon => Color::default(),
            SpellVariant::Teleport => Color::default(),
            SpellVariant::Ward => Color::default(),
            _ => Color::default(),
        }
    }

    fn icon_file(&self) -> String {
        match &self.variant {
            SpellVariant::Unknown => self.icon_fallback(),
            SpellVariant::BoundWeapon(w) => match w {
                BoundType::BattleAxe => Icon::WeaponAxeTwoHanded.icon_file(),
                BoundType::Bow => Icon::WeaponBow.icon_file(),
                _ => Icon::WeaponSwordOneHanded.icon_file(),
            }, // TODO give this variations
            SpellVariant::Burden => self.icon_fallback(),
            SpellVariant::Cure => Icon::SpellCure.icon_file(),
            SpellVariant::Damage(t) => match t {
                MagicDamageType::None => self.icon_fallback(),
                MagicDamageType::Disease => self.icon_fallback(),
                MagicDamageType::Fire => match &self.level {
                    MagicSpellLevel::Novice => Icon::SpellFire.icon_file(),
                    MagicSpellLevel::Apprentice => Icon::SpellFire.icon_file(),
                    MagicSpellLevel::Adept => Icon::SpellFire.icon_file(),
                    MagicSpellLevel::Master => Icon::SpellFire.icon_file(),
                    MagicSpellLevel::Expert => Icon::SpellFire.icon_file(),
                },
                MagicDamageType::Frost => match &self.level {
                    MagicSpellLevel::Novice => Icon::SpellFrost.icon_file(),
                    MagicSpellLevel::Apprentice => Icon::SpellFrost.icon_file(),
                    MagicSpellLevel::Adept => Icon::SpellFrost.icon_file(),
                    MagicSpellLevel::Master => Icon::SpellFrost.icon_file(),
                    MagicSpellLevel::Expert => Icon::SpellFrost.icon_file(),
                },
                MagicDamageType::Magic => Icon::IconDefault.icon_file(),
                MagicDamageType::Poison => Icon::SpellPoison.icon_file(),
                MagicDamageType::Shock => match &self.level {
                    MagicSpellLevel::Novice => Icon::SpellShock.icon_file(),
                    MagicSpellLevel::Apprentice => Icon::SpellShockStrong.icon_file(),
                    MagicSpellLevel::Adept => Icon::SpellChainLightning.icon_file(),
                    MagicSpellLevel::Master => Icon::SpellLightning.icon_file(),
                    MagicSpellLevel::Expert => Icon::SpellLightningBlast.icon_file(),
                },
                MagicDamageType::Sun => Icon::SpellHoly.icon_file(),
            },
            SpellVariant::Banish => todo!(),
            SpellVariant::Blizzard => todo!(),
            SpellVariant::Calm => todo!(),
            SpellVariant::CarryWeight => Icon::SpellFeather.icon_file(),
            SpellVariant::Cloak(_) => todo!(),
            SpellVariant::Demoralize => Icon::SpellFear.icon_file(),
            SpellVariant::Detect => Icon::SpellDetect.icon_file(),
            SpellVariant::Fear => todo!(),
            SpellVariant::Fireball => todo!(),
            SpellVariant::Firebolt => todo!(),
            SpellVariant::FireboltStorm => todo!(),
            SpellVariant::FireWall => todo!(),
            SpellVariant::Frost => todo!(),
            SpellVariant::FrostWall => todo!(),
            SpellVariant::Guide => Icon::SpellWisp.icon_file(),
            SpellVariant::Heal => Icon::SpellHeal.icon_file(),
            SpellVariant::IceSpike => todo!(),
            SpellVariant::IceStorm => todo!(),
            SpellVariant::IcySpear => todo!(),
            SpellVariant::Invisibility => todo!(),
            SpellVariant::Light => Icon::SpellLight.icon_file(),
            SpellVariant::LightningBolt => todo!(),
            SpellVariant::LightningStorm => todo!(),
            SpellVariant::Mayhem => todo!(),
            SpellVariant::Pacify => todo!(),
            SpellVariant::Paralyze => todo!(),
            SpellVariant::Rally => todo!(),
            SpellVariant::Reanimate => Icon::SpellReanimate.icon_file(),
            SpellVariant::Reflect => Icon::SpellReflect.icon_file(),
            SpellVariant::Rout => todo!(),
            SpellVariant::Rune => Icon::SpellRune.icon_file(),
            SpellVariant::Shock => todo!(),
            SpellVariant::SoulTrap => Icon::SpellSoultrap.icon_file(),
            SpellVariant::Sparks => todo!(),
            SpellVariant::StormWall => todo!(),
            SpellVariant::Summon => Icon::SpellSummon.icon_file(),
            SpellVariant::Teleport => Icon::SpellTeleport.icon_file(),
            SpellVariant::Thunderbolt => todo!(),
            SpellVariant::TurnUndead => todo!(),
            SpellVariant::Ward => Icon::SpellWard.icon_file(),
        }
    }

    fn icon_fallback(&self) -> String {
        match &self.school {
            School::Alteration => Icon::Alteration.icon_file(),
            School::Conjuration => Icon::Conjuration.icon_file(),
            School::Destruction => Icon::Destruction.icon_file(),
            School::Illusion => Icon::Illusion.icon_file(),
            School::Restoration => Icon::Restoration.icon_file(),
            School::None => Icon::IconDefault.icon_file(),
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
            20 => School::Conjuration,
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

#[derive(Clone, Debug, Display, Hash, Eq, PartialEq)]
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

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub enum BoundType {
    BattleAxe,
    Bow,
    Dagger,
    Greatsword,
    Hammer,
    Mace,
    Shield,
    Sword,
    WarAxe,
    #[default]
    Unknown,
}

// Some magic overhauls move spells from one school to another, so this
// classification should be used for all schools even if you reasonably think
// that healing spells will never be destruction spells. Also, this is as
// ad-hoc as the game spell types themselves.
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub enum SpellVariant {
    #[default]
    Unknown,
    Banish,
    Blizzard,
    BoundWeapon(BoundType),
    Burden,
    Calm,                   // effects will include av calm
    CarryWeight,            // feather
    Cloak(MagicDamageType), // might need to be more general than damage? also resists
    // CorrodeArmor, DisintegrateWeapon
    Cure,
    Damage(MagicDamageType),
    Demoralize,
    Detect,
    // Drain,
    Fear,
    Fireball,
    Firebolt,
    FireWall,
    FireboltStorm,
    // Font (Life, Strength, Wisdom)
    Frost,
    FrostWall,
    Guide,
    Heal,
    IceSpike,
    IceStorm,
    IcySpear,
    Invisibility,
    Light,
    LightningBolt,
    LightningStorm,
    Mayhem,
    // Muffle,
    Pacify,
    Paralyze,
    Rally, // CallToArms
    Reanimate,
    Reflect,
    Rout,
    Rune,
    Shock,
    Sparks,

    SoulTrap,
    StormWall,
    Summon,
    Teleport,
    Thunderbolt,
    // Transmute,
    TurnUndead,
    Ward,
    // Waterbreathing,
    // Waterwalking,
}

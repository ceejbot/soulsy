use enumset::{enum_set, EnumSet, EnumSetType};
use strum::{Display, EnumString};

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
    ColdFire,
    Disease,
    Earth,
    Fire,
    Frost,
    Lunar,
    Magic,
    Necrotic,
    Poison,
    Shadow,
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
            MagicDamageType::ColdFire => InvColor::Frost.color(),
            MagicDamageType::Disease => InvColor::Green.color(),
            MagicDamageType::Earth => InvColor::Brown.color(),
            MagicDamageType::Fire => InvColor::Fire.color(),
            MagicDamageType::Frost => InvColor::Frost.color(),
            MagicDamageType::Lunar => InvColor::Silver.color(),
            MagicDamageType::Magic => InvColor::Blue.color(),
            MagicDamageType::Necrotic => InvColor::Eldritch.color(),
            MagicDamageType::Poison => InvColor::Poison.color(),
            MagicDamageType::Shadow => InvColor::Purple.color(),
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
            MagicDamageType::ColdFire => Icon::SpellFire.icon_file(),
            MagicDamageType::Disease => self.icon_fallback(),
            MagicDamageType::Earth => Icon::SpellEarth.icon_file(),
            MagicDamageType::Fire => Icon::SpellFire.icon_file(),
            MagicDamageType::Frost => Icon::SpellFrost.icon_file(),
            MagicDamageType::Lunar => Icon::SpellMoon.icon_file(),
            MagicDamageType::Magic => self.icon_fallback(),
            MagicDamageType::Necrotic => self.icon_fallback(),
            MagicDamageType::Poison => Icon::SpellPoison.icon_file(),
            MagicDamageType::Shadow => self.icon_fallback(),
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

// ---------- keywords

pub const DAR_ABYSS: EnumSet<SpellKeyword> = enum_set!(
    SpellKeyword::ABY_ShadowDamageResist
        | SpellKeyword::ABY_ShadowInvisAddonSpell
        | SpellKeyword::ABY_ShadowMantleSpell
        | SpellKeyword::ABY_ShadowWeaponSpell
);

pub const DAR_ASTRAL: EnumSet<SpellKeyword> = enum_set!(
    SpellKeyword::DAR_AstralBodySpell
        | SpellKeyword::DAR_AstralRestoreSpell
        | SpellKeyword::DAR_AstralStarSpell
);

pub const DAR_BLOOD: EnumSet<SpellKeyword> = enum_set!(
    SpellKeyword::BLO_BloodFFSpell
        | SpellKeyword::BLO_BloodLustSpell
        | SpellKeyword::BLO_BloodSiphonSpell
        | SpellKeyword::BLO_BloodStanceSpell
        | SpellKeyword::BLO_BloodStealHealthSpell
);

pub const DAR_ELDRITCH: EnumSet<SpellKeyword> = enum_set!(
    SpellKeyword::DAR_EldritchFrenzySpell
        | SpellKeyword::DAR_EldritchHardnessSpell
        | SpellKeyword::DAR_EldritchInfectionSpell
        | SpellKeyword::DAR_EldritchWeaveSpell
);

pub const BOUND_WEAPON: EnumSet<SpellKeyword> =
    enum_set!(SpellKeyword::MAG_MagicSummonWeapon | SpellKeyword::MAG_WeapTypeBound);

pub fn enumset_contains_any(set: &EnumSet<SpellKeyword>, keywords: &Vec<SpellKeyword>) -> bool {
    keywords.iter().any(|xs| set.contains(*xs))
}

// I collected all of the SimonMagus and most of the Darenii keywords here.
// I can't use all of them, but what the heck.
#[derive(Debug, EnumSetType, EnumString, Hash)]
pub enum SpellKeyword {
    // vanilla keywords
    MagicArmorSpell,
    MagicCloak,
    MagicDamageFire,
    MagicDamageFrost,
    MagicDamageResist,
    MagicDamageShock,
    MagicInfluence,
    MagicInfluenceCharm,
    MagicInfluenceFear,
    MagicInfluenceFrenzy,
    MagicInvisibility,
    MagicNightEye,
    MagicParalysis,
    MagicRestoreHealth,
    MagicRune,
    MagicSlow,
    MagicSummonFamiliar,
    MagicSummonFire,
    MagicSummonFrost,
    MagicSummonShock,
    MagicSummonUndead,
    MagicTelekinesis,
    MagicTurnUndead,
    MagicWard,
    MagicWeaponSpeed,
    MQClearSkiesFogTrigger,
    RitualSpellEffect,

    // shouts
    MagicShout,
    MagicVoiceChangeWeather,
    ShoutFireBreath,

    // SimonMagus keywords
    MAG_AnimalAllyKeyword,
    MAG_IsEthereal,
    MAG_MagicDamageBleed,
    MAG_MagicDamageMagicka,
    MAG_MagicDamagePoison,
    MAG_MagicDamageStamina,
    MAG_MagicDamageSun,
    MAG_MagicEffectLight,
    MAG_MagicFortifySpeed,
    MAG_MagicInfluenceCommand,
    MAG_MagicInfluenceCourage,
    MAG_MagicInfluenceParalysis,
    MAG_MagicInfluenceSilence,
    MAG_MagicJumpSpell,
    MAG_MagicShieldFire,
    MAG_MagicShieldFrost,
    MAG_MagicShieldPoison,
    MAG_MagicShieldSpell,
    MAG_MagicShoutAuraWhisper,
    MAG_MagicShoutBecomeEthereal,
    MAG_MagicSlowfallSpell,
    MAG_MagicSoulTrap,
    MAG_MagicStaffEnchantment,
    MAG_MagicStealthSpell,
    MAG_MagicSummonReanimate,
    MAG_MagicSummonWeapon,
    MAG_MagicUnarmedSpell,
    MAG_MagicWeaponEnchantment,
    MAG_MagicWeightSpell,
    MAG_PoisonCloakSpell,
    MAG_WeapTypeBound,

    // Darenii keywords
    ABY_ShadowDamageResist,
    ABY_ShadowInvisAddonSpell,
    ABY_ShadowMantleSpell,
    ABY_ShadowWeaponSpell,
    BLO_BloodFFSpell,
    BLO_BloodLustSpell,
    BLO_BloodSiphonSpell,
    BLO_BloodStanceSpell,
    BLO_BloodStealHealthSpell,
    DAR_ArcaneDamageMagickaRegen,
    DAR_ArcaneDamageRelease,
    DAR_ArcanePullSpell,
    DAR_ArcaneWeaponSpell,
    DAR_ArclightBodySpell,
    DAR_AstralBodySpell,
    DAR_AstralRestoreSpell,
    DAR_AstralStarSpell,
    DAR_EldritchFrenzySpell,
    DAR_EldritchHardnessSpell,
    DAR_EldritchInfectionSpell,
    DAR_EldritchWeaveSpell,
    DAR_MagicAspectSpell,
    DAR_MagicMeleeProcSpell,
    DAR_MagicSkoriaSlow,
    DAR_MagicWeaponSlow,
    DAR_MoltenBodySpell,
    DAR_NecroticDamageBlocker,
    DAR_UnspecificMagicDamage,
    DAR_WeakenWeapons,
    IconMagicEarth,
    IconMagicWater,
    IconMagicWind,
    LUN_LunarBodySpell,
    MagicDamageLunar,
    MagicFlameCloak,
    NAT_MagicAttunementSpell,    // fortify stamina & stam regen
    NAT_MagicNatureHealingSpell, // all healing
    NAT_MagicReflectSpell,       // used by all the reflect spells
    NAT_MagicRejuvenateSpell,
    NAT_MagicRevitalizingGrowthSpell,
    NAT_MagicRoot,
    NAT_MagicSpikeSpell,
    NAT_MagicStrengthSpell,
    NAT_MagicTreeBarkSpell,
}

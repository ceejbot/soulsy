//! Soulsy distributes some keywords to spells and shouts in vanilla and
//! in various spell packs to identify them for iconnification.
//! Mostly it relies on OCF's new-ish magic effect keywords.

use enumset::{enum_set, EnumSet, EnumSetType};
use strum::{Display, EnumIter, IntoEnumIterator};

impl TryFrom<&str> for SpellEffectKeywords {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let keystr = value
            .to_lowercase()
            .replace("soulsy_", "")
            .replace("ocf_mgef", "");
        let keywd = SpellEffectKeywords::iter().find(|xs| keystr == xs.to_string());
        if let Some(k) = keywd {
            Ok(k)
        } else {
            Err(anyhow::anyhow!("not a valid soulsy keyword"))
        }
    }
}

#[derive(Debug, Hash, Display, EnumIter, EnumSetType)]
#[strum(serialize_all = "lowercase")]
pub enum SpellEffectKeywords {
    // Some vanilla and mod spell archetypes to mark with keywords
    Archetype_CarryWeight,
    Archetype_Cure,
    Archetype_Detect,
    Archetype_Guide,
    Archetype_Light,
    Archetype_NightEye,
    Archetype_Protect,
    Archetype_Reflect,
    Archetype_Resist,
    Archetype_Root,
    Archetype_Silence,
    Archetype_Teleport,
    Archetype_Waterbreathing,
    Archetype_Waterwalking,
    Archetype_WeaponBuff,

    // Hints about which art to use.
    ArtBall,
    ArtBlast,
    ArtBolt,
    ArtBreath,
    ArtChainLightning,
    ArtFlame,
    ArtLightning,
    ArtProjectile,
    ArtSpike,
    ArtStorm,
    ArtTornado,
    ArtWall,

    // Bound weapon types
    BoundWarAxe,
    BoundBattleAxe,
    BoundBow,
    BoundDagger,
    BoundHammer,
    BoundMace,
    BoundShield,
    BoundSword,
    BoundGreatsword,

    // vanilla magic keywords
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

    // from OCF and others
    MAG_MagicDamageSun,
    IconWind,
    IconMagicWind,
    IconWater,
    IconMagicWater,

    // vanilla shouts
    Shout_AnimalAllegiance,
    Shout_AuraWhisper,
    Shout_BattleFury,
    Shout_BecomeEthereal,
    Shout_BendWill,
    Shout_CallDragon,
    Shout_CallOfValor,
    Shout_ClearSkies,
    Shout_Disarm,
    Shout_Dismay,
    Shout_DragonAspect,
    Shout_Dragonrend,
    Shout_DrainVitality,
    Shout_ElementalFury,
    Shout_FireBreath,
    Shout_FrostBreath,
    Shout_IceForm,
    Shout_KynesPeace,
    Shout_MarkedForDeath,
    Shout_Slowtime,
    Shout_SoulTear,
    Shout_StormCall,
    Shout_SummonDurnehviir,
    Shout_ThrowVoice,
    Shout_UnrelentingForce,
    Shout_WhirlwindSprint,

    // From here on it's OCF keywords minus the prefix
    ClassArcane,
    ClassArtificer,
    ClassAsh,
    ClassAstral,
    ClassBard,
    ClassBlood,
    ClassDruid,
    ClassEarth,
    ClassEldritch,
    ClassFire,
    ClassFrost,
    ClassHoly,
    ClassNecromancy,
    ClassPoison,
    ClassRace_Altmer,
    ClassRace_Argonian,
    ClassRace_Breton,
    ClassRace_Orsimer,
    ClassRace_Redguard,
    ClassRace_Vampire,
    ClassRace_Werebeast,
    ClassShadow,
    ClassShock,
    ClassSurvival,
    ClassSurvival_Needs,
    ClassSurvival_Wilderness,
    ClassUtility,
    ClassWater,
    ClassWind,
    DeliverTouch,
    PowerAction,
    PowerAction_Bag,
    PowerAction_Bard,
    PowerAction_Bathe,
    PowerAction_Bless,
    PowerAction_Broom,
    PowerAction_BuryCorpse,
    PowerAction_Campfire,
    PowerAction_Coin,
    PowerAction_CommandFollower,
    PowerAction_Craft,
    PowerAction_FillWater,
    PowerAction_Goggles,
    PowerAction_GogglesSight,
    PowerAction_HarvestCorpse,
    PowerAction_HarvestGather,
    PowerAction_HarvestWood,
    PowerAction_Horse,
    PowerAction_HorseHorn,
    PowerAction_HorseWhistle,
    PowerAction_Influence,
    PowerAction_InfluenceCancelRetreat,
    PowerAction_InfluenceEngage,
    PowerAction_Instincts,
    PowerAction_Lantern,
    PowerAction_PeekKeyhole,
    PowerAction_PitchTent,
    PowerAction_Potion,
    PowerAction_Pray,
    PowerAction_Relax,
    PowerAction_Speech,
    PowerAction_StatusFrostfall,
    PowerAction_StatusSunhelm,
    PowerAction_TameAnimal,
    PowerAction_Train,
    PowerAction_WeaponGrip,
    PowerCheat,
    PowerConfig,
    PowerGrand,
    SpellAbsorb_Magicka,
    SpellAbsorb_MagickaCloak,
    SpellAbsorb_Stamina,
    SpellAbsorb_StaminaCloak,
    SpellAssist_DamageDruid,
    SpellAssist_MovementSpeedDruid,
    SpellBound,
    SpellBound_Ammo,
    SpellBound_Armor,
    SpellBound_Weapon,
    SpellControl,
    SpellCounter_Astral,
    SpellCounter_BloodDruid,
    SpellCounter_Druid,
    SpellCounter_DruidHeal,
    SpellCounter_Fire,
    SpellCurse_Deconstruct,
    SpellCurse_DruidRoot,
    SpellCurse_Shadow,
    SpellDamage_Arcane,
    SpellDamage_ArcaneCloak,
    SpellDamage_ArcaneFire,
    SpellDamage_ArcaneFireCloak,
    SpellDamage_AshFire,
    SpellDamage_AshFireCloak,
    SpellDamage_Blood,
    SpellDamage_BloodCloak,
    SpellDamage_Deconstruct,
    SpellDamage_Earth,
    SpellDamage_EarthCloak,
    SpellDamage_Fire,
    SpellDamage_FireCloak,
    SpellDamage_FireCloakDunmer,
    SpellDamage_FireCold,
    SpellDamage_Force,
    SpellDamage_Frost,
    SpellDamage_FrostFire,
    SpellDamage_FrostFireCloak,
    SpellDamage_HolyAstral,
    SpellDamage_HolyAstralCloak,
    SpellDamage_HolyLunar,
    SpellDamage_HolyLunarCloak,
    SpellDamage_Light,
    SpellDamage_NecroticFire,
    SpellDamage_NecroticFireCloak,
    SpellDamage_Poison,
    SpellDamage_PoisonBug,
    SpellDamage_PoisonBugCloak,
    SpellDamage_PoisonCloak,
    SpellDamage_PoisonDoomstone,
    SpellDamage_PoisonEldritch,
    SpellDamage_PoisonEldritchCloak,
    SpellDamage_Shadow,
    SpellDamage_ShadowCloak,
    SpellDamage_ShockArc,
    SpellDamage_ShockArcCloak,
    SpellDamage_Sonic,
    SpellDamage_Steam,
    SpellDamage_Water,
    SpellDamage_WaterCloak,
    SpellDamage_Wind,
    SpellDamage_WindCloak,
    SpellDispel,
    SpellDivination,
    SpellEnchant,
    SpellEnhance_Attack,
    SpellEnhance_AttackEldritch,
    SpellEnhance_Casting,
    SpellEnhance_CastingDruid,
    SpellEnhance_CastingEldritch,
    SpellEnhance_CritShadowInvis,
    SpellEnhance_Damage,
    SpellEnhance_DamageArcane,
    SpellEnhance_DamageAshFire,
    SpellEnhance_DamageBlood,
    SpellEnhance_DamageBloodDruid,
    SpellEnhance_DamageDruidHunter,
    SpellEnhance_DamageFire,
    SpellEnhance_DamageFrost,
    SpellEnhance_DamageHolyAstral,
    SpellEnhance_DamageHolyLunar,
    SpellEnhance_DamagePoison,
    SpellEnhance_DamagePoisonEldritch,
    SpellEnhance_DamageShadow,
    SpellEnhance_DamageShock,
    SpellEnhance_DamageShockArc,
    SpellEnhance_Dodge,
    SpellEnhance_EldritchTome,
    SpellEnhance_EvasionDruid,
    SpellEnhance_Fall,
    SpellEnhance_Health,
    SpellEnhance_Jump,
    SpellEnhance_MovementSpeedDruid,
    SpellEnhance_Regen,
    SpellEnhance_RegenShadowInvis,
    SpellEnhance_Sight,
    SpellEnhance_SightKhajiit,
    SpellEnhance_SightVampire,
    SpellEnhance_SightVampireBlood,
    SpellEnhance_SightVampireShadow,
    SpellEnhance_SightWerebeast,
    SpellEnhance_StaminaDruid,
    SpellEnhance_Swim,
    SpellEnhance_WaterWalk,
    SpellEthereal,
    SpellForce,
    SpellHarvest,
    SpellHeal_Daedra,
    SpellHeal_Living,
    SpellHeal_LivingWater,
    SpellHeal_Self,
    SpellHeal_SelfCloak,
    SpellHeal_Undead,
    SpellLight,
    SpellMind_Charm,
    SpellMind_CharmImperial,
    SpellMind_Control,
    SpellMind_ControlBosmer,
    SpellMind_ControlVampire,
    SpellMind_Fear,
    SpellMind_FearNord,
    SpellMind_FrenzyShadow,
    SpellMind_Paralysis,
    SpellParalysis,
    SpellParalysis_Druid,
    SpellProtect_Damage,
    SpellProtect_DamageEldritch,
    SpellProtect_Magic,
    SpellReanimate,
    SpellReanimateDoomstone,
    SpellReflect_Druid,
    SpellRestore_Exposure,
    SpellRestore_Magicka,
    SpellRestore_MagickaCircle,
    SpellRestore_MagickaWater,
    SpellRestore_Stamina,
    SpellRestore_StaminaCircle,
    SpellRestore_StaminaDruid,
    SpellRestore_Warmth,
    SpellSacrifice,
    SpellSacrifice_Blood,
    SpellShapechange,
    SpellShapechange_Creature,
    SpellShapechange_Vampire,
    SpellShapechange_Werebeast,
    SpellShield_Warmth,
    SpellSoulTrap,
    SpellSpace,
    SpellSpace_Teleport,
    SpellStealth,
    SpellStealth_Invisibility,
    SpellStealth_InvisibilityDoomstone,
    SpellStealth_InvisibilityDruid,
    SpellSummon_Construct,
    SpellSummon_Creature,
    SpellSummon_Daedra,
    SpellSummon_DaedraEldritch,
    SpellSummon_Object,
    SpellSummon_Spirit,
    SpellSummon_SpiritFrost,
    SpellSummon_SpiritShadow,
    SpellSummon_Undead,
    SpellTime,
    SpellTransmute,
    SpellTurnUndeadCircle,
    SpellUnlock,
}

// ----------- spell archetypes

pub const BUFF_SPELLS: EnumSet<SpellEffectKeywords> = enum_set!(
    SpellEffectKeywords::SpellEnhance_Attack
        | SpellEffectKeywords::SpellEnhance_AttackEldritch
        | SpellEffectKeywords::SpellEnhance_Casting
        | SpellEffectKeywords::SpellEnhance_CastingDruid
        | SpellEffectKeywords::SpellEnhance_CastingEldritch
        | SpellEffectKeywords::SpellEnhance_CritShadowInvis
        | SpellEffectKeywords::SpellEnhance_Damage
        | SpellEffectKeywords::SpellEnhance_DamageArcane
        | SpellEffectKeywords::SpellEnhance_DamageAshFire
        | SpellEffectKeywords::SpellEnhance_DamageBlood
        | SpellEffectKeywords::SpellEnhance_DamageBloodDruid
        | SpellEffectKeywords::SpellEnhance_DamageDruidHunter
        | SpellEffectKeywords::SpellEnhance_DamageFire
        | SpellEffectKeywords::SpellEnhance_DamageFrost
        | SpellEffectKeywords::SpellEnhance_DamageHolyAstral
        | SpellEffectKeywords::SpellEnhance_DamageHolyLunar
        | SpellEffectKeywords::SpellEnhance_DamagePoison
        | SpellEffectKeywords::SpellEnhance_DamagePoisonEldritch
        | SpellEffectKeywords::SpellEnhance_DamageShadow
        | SpellEffectKeywords::SpellEnhance_DamageShock
        | SpellEffectKeywords::SpellEnhance_DamageShockArc
        | SpellEffectKeywords::SpellEnhance_Dodge
        | SpellEffectKeywords::SpellEnhance_EldritchTome
        | SpellEffectKeywords::SpellEnhance_EvasionDruid
        | SpellEffectKeywords::SpellEnhance_Fall
        | SpellEffectKeywords::SpellEnhance_Health
        | SpellEffectKeywords::SpellEnhance_Jump
        | SpellEffectKeywords::SpellEnhance_MovementSpeedDruid
        | SpellEffectKeywords::SpellEnhance_Regen
        | SpellEffectKeywords::SpellEnhance_RegenShadowInvis
        | SpellEffectKeywords::SpellEnhance_StaminaDruid
        | SpellEffectKeywords::SpellEnhance_Swim
        | SpellEffectKeywords::SpellEnhance_WaterWalk
);

pub const CLOAK_SPELLS: EnumSet<SpellEffectKeywords> = enum_set!(
    SpellEffectKeywords::MagicCloak
        | SpellEffectKeywords::SpellAbsorb_MagickaCloak
        | SpellEffectKeywords::SpellAbsorb_StaminaCloak
        | SpellEffectKeywords::SpellDamage_ArcaneCloak
        | SpellEffectKeywords::SpellDamage_ArcaneFireCloak
        | SpellEffectKeywords::SpellDamage_AshFireCloak
        | SpellEffectKeywords::SpellDamage_BloodCloak
        | SpellEffectKeywords::SpellDamage_EarthCloak
        | SpellEffectKeywords::SpellDamage_FireCloak
        | SpellEffectKeywords::SpellDamage_FireCloakDunmer
        | SpellEffectKeywords::SpellDamage_FrostFireCloak
        | SpellEffectKeywords::SpellDamage_HolyAstralCloak
        | SpellEffectKeywords::SpellDamage_HolyLunarCloak
        | SpellEffectKeywords::SpellDamage_NecroticFireCloak
        | SpellEffectKeywords::SpellDamage_PoisonBugCloak
        | SpellEffectKeywords::SpellDamage_PoisonCloak
        | SpellEffectKeywords::SpellDamage_PoisonEldritchCloak
        | SpellEffectKeywords::SpellDamage_ShadowCloak
        | SpellEffectKeywords::SpellDamage_ShockArcCloak
        | SpellEffectKeywords::SpellDamage_WaterCloak
        | SpellEffectKeywords::SpellDamage_WindCloak
);

pub const CONTROL_SPELLS: EnumSet<SpellEffectKeywords> = enum_set!(
    SpellEffectKeywords::MagicInfluence
        | SpellEffectKeywords::MagicInfluenceCharm
        | SpellEffectKeywords::SpellControl
        | SpellEffectKeywords::SpellMind_Charm
        | SpellEffectKeywords::SpellMind_CharmImperial
        | SpellEffectKeywords::SpellMind_Control
        | SpellEffectKeywords::SpellMind_ControlBosmer
        | SpellEffectKeywords::SpellMind_ControlVampire
);

pub const _COUNTER_SPELLS: EnumSet<SpellEffectKeywords> = enum_set!(
    SpellEffectKeywords::SpellCounter_Astral
        | SpellEffectKeywords::SpellCounter_BloodDruid
        | SpellEffectKeywords::SpellCounter_Druid
        | SpellEffectKeywords::SpellCounter_DruidHeal
        | SpellEffectKeywords::SpellCounter_Fire
);

pub const _CURSES: EnumSet<SpellEffectKeywords> = enum_set!(
    SpellEffectKeywords::SpellCurse_Deconstruct
        | SpellEffectKeywords::SpellCurse_DruidRoot
        | SpellEffectKeywords::SpellCurse_Shadow
);

pub const FEAR_SPELLS: EnumSet<SpellEffectKeywords> = enum_set!(
    SpellEffectKeywords::MagicInfluenceFear
        | SpellEffectKeywords::SpellMind_Fear
        | SpellEffectKeywords::SpellMind_FearNord
);

pub const _FRENZY_SPELLS: EnumSet<SpellEffectKeywords> = enum_set!(
    SpellEffectKeywords::MagicInfluenceFrenzy | SpellEffectKeywords::SpellMind_FrenzyShadow
);

pub const HEALING_SPELLS: EnumSet<SpellEffectKeywords> = enum_set!(
    SpellEffectKeywords::MagicRestoreHealth
        | SpellEffectKeywords::SpellHeal_Daedra
        | SpellEffectKeywords::SpellHeal_Living
        | SpellEffectKeywords::SpellHeal_LivingWater
        | SpellEffectKeywords::SpellHeal_Self
        | SpellEffectKeywords::SpellHeal_SelfCloak
        | SpellEffectKeywords::SpellHeal_Undead
);

pub const PARALYZE_SPELLS: EnumSet<SpellEffectKeywords> = enum_set!(
    SpellEffectKeywords::MagicParalysis
        | SpellEffectKeywords::SpellMind_Paralysis
        | SpellEffectKeywords::SpellParalysis
        | SpellEffectKeywords::SpellParalysis_Druid
);

pub const SUMMON_SPELLS: EnumSet<SpellEffectKeywords> = enum_set!(
    SpellEffectKeywords::MagicSummonFamiliar
        | SpellEffectKeywords::MagicSummonFire
        | SpellEffectKeywords::MagicSummonFrost
        | SpellEffectKeywords::MagicSummonShock
        | SpellEffectKeywords::MagicSummonUndead
);

pub const VISION_SPELLS: EnumSet<SpellEffectKeywords> = enum_set!(
    SpellEffectKeywords::SpellEnhance_Sight
        | SpellEffectKeywords::SpellEnhance_SightKhajiit
        | SpellEffectKeywords::SpellEnhance_SightVampire
        | SpellEffectKeywords::SpellEnhance_SightVampireBlood
        | SpellEffectKeywords::SpellEnhance_SightVampireShadow
        | SpellEffectKeywords::SpellEnhance_SightWerebeast
);

// ----------- damage types

pub const DAMAGE_ARCANE: EnumSet<SpellEffectKeywords> =
    enum_set!(SpellEffectKeywords::ClassArcane | SpellEffectKeywords::SpellDamage_Arcane);

pub const DAMAGE_ARCANEFIRE: EnumSet<SpellEffectKeywords> = enum_set!(
    SpellEffectKeywords::ClassArcane
        | SpellEffectKeywords::SpellDamage_ArcaneFire
        | SpellEffectKeywords::SpellDamage_ArcaneFireCloak
);

pub const DAMAGE_ASHFIRE: EnumSet<SpellEffectKeywords> = enum_set!(
    SpellEffectKeywords::ClassAsh
        | SpellEffectKeywords::SpellDamage_AshFire
        | SpellEffectKeywords::SpellDamage_AshFireCloak
);

pub const DAMAGE_ASTRAL: EnumSet<SpellEffectKeywords> = enum_set!(
    SpellEffectKeywords::ClassAstral
        | SpellEffectKeywords::SpellDamage_HolyAstral
        | SpellEffectKeywords::SpellDamage_HolyAstralCloak
);

pub const DAMAGE_BLOOD: EnumSet<SpellEffectKeywords> = enum_set!(
    SpellEffectKeywords::ClassBlood
        | SpellEffectKeywords::SpellDamage_Blood
        | SpellEffectKeywords::SpellDamage_BloodCloak
);

pub const DAMAGE_EARTH: EnumSet<SpellEffectKeywords> = enum_set!(
    SpellEffectKeywords::ClassEarth
        | SpellEffectKeywords::SpellDamage_Earth
        | SpellEffectKeywords::SpellDamage_EarthCloak
);

pub const DAMAGE_FIRE: EnumSet<SpellEffectKeywords> = enum_set!(
    SpellEffectKeywords::ClassFire
        | SpellEffectKeywords::MagicDamageFire
        | SpellEffectKeywords::SpellDamage_Fire
        | SpellEffectKeywords::SpellDamage_FireCloak
        | SpellEffectKeywords::SpellDamage_FireCloakDunmer
);

pub const DAMAGE_FROST: EnumSet<SpellEffectKeywords> = enum_set!(
    SpellEffectKeywords::ClassFrost
        | SpellEffectKeywords::SpellDamage_Frost
        | SpellEffectKeywords::MagicDamageFrost
);

pub const DAMAGE_FROSTFIRE: EnumSet<SpellEffectKeywords> = enum_set!(
    SpellEffectKeywords::SpellDamage_FrostFire
        | SpellEffectKeywords::SpellDamage_FrostFire
        | SpellEffectKeywords::SpellDamage_FrostFireCloak
);

pub const DAMAGE_LUNAR: EnumSet<SpellEffectKeywords> = enum_set!(
    SpellEffectKeywords::SpellDamage_HolyLunar | SpellEffectKeywords::SpellDamage_HolyLunarCloak
);

pub const DAMAGE_NECROTIC: EnumSet<SpellEffectKeywords> = enum_set!(
    SpellEffectKeywords::ClassNecromancy
        | SpellEffectKeywords::SpellDamage_NecroticFire
        | SpellEffectKeywords::SpellDamage_NecroticFireCloak
);

pub const DAMAGE_POISON: EnumSet<SpellEffectKeywords> = enum_set!(
    SpellEffectKeywords::ClassPoison
        | SpellEffectKeywords::SpellDamage_Poison
        | SpellEffectKeywords::SpellDamage_PoisonBug
        | SpellEffectKeywords::SpellDamage_PoisonBugCloak
        | SpellEffectKeywords::SpellDamage_PoisonCloak
        | SpellEffectKeywords::SpellDamage_PoisonDoomstone
        | SpellEffectKeywords::SpellDamage_PoisonEldritch
        | SpellEffectKeywords::SpellDamage_PoisonEldritchCloak
);

pub const DAMAGE_SHADOW: EnumSet<SpellEffectKeywords> = enum_set!(
    SpellEffectKeywords::ClassShadow
        | SpellEffectKeywords::SpellDamage_Shadow
        | SpellEffectKeywords::SpellDamage_ShadowCloak
);

pub const DAMAGE_SHOCK: EnumSet<SpellEffectKeywords> =
    enum_set!(SpellEffectKeywords::MagicDamageShock | SpellEffectKeywords::ClassShock);

pub const DAMAGE_SHOCKARC: EnumSet<SpellEffectKeywords> = enum_set!(
    SpellEffectKeywords::SpellDamage_ShockArc | SpellEffectKeywords::SpellDamage_ShockArcCloak
);

pub const DAMAGE_SUN: EnumSet<SpellEffectKeywords> = enum_set!(
    SpellEffectKeywords::ClassHoly
        | SpellEffectKeywords::MAG_MagicDamageSun
        | SpellEffectKeywords::SpellDamage_Light
);

pub const DAMAGE_WATER: EnumSet<SpellEffectKeywords> = enum_set!(
    SpellEffectKeywords::ClassWater
        | SpellEffectKeywords::IconWater
        | SpellEffectKeywords::SpellDamage_Steam
        | SpellEffectKeywords::SpellDamage_Water
        | SpellEffectKeywords::SpellDamage_WaterCloak
);

pub const DAMAGE_WIND: EnumSet<SpellEffectKeywords> = enum_set!(
    SpellEffectKeywords::ClassWind
        | SpellEffectKeywords::IconWind
        | SpellEffectKeywords::SpellDamage_Wind
        | SpellEffectKeywords::SpellDamage_WindCloak
        | SpellEffectKeywords::SpellDamage_Sonic
);

//! Magic effect keywords from OCF, the base game, and some spell packs.
//!
//! Soulsy distributes some keywords to spells and shouts in vanilla and
//! in various spell packs to identify them for iconnification.
//! Mostly it relies on OCF's new-ish magic effect keywords.

use enumset::{enum_set, EnumSet, EnumSetType};
use strum::{Display, EnumIter, IntoEnumIterator};

impl TryFrom<&str> for SpellKeywords {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let keystr = value
            .to_lowercase()
            .replace("soulsy_", "")
            .replace("ocf_mgef", "");
        let keywd = SpellKeywords::iter().find(|xs| keystr == xs.to_string());
        if let Some(k) = keywd {
            Ok(k)
        } else {
            Err(anyhow::anyhow!("not a valid soulsy magic keyword"))
        }
    }
}

#[derive(Debug, Hash, Display, EnumIter, EnumSetType)]
#[strum(serialize_all = "lowercase")]
pub enum SpellKeywords {
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
    // ArtBall,
    // ArtBlast,
    // ArtBolt,
    // ArtBreath,
    // ArtChainLightning,
    // ArtFlame,
    // ArtLightning,
    // ArtProjectile,
    // ArtSpike,
    // ArtStorm,
    // ArtTornado,
    // ArtWall,

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
    DAR_SummonAstralWyrm,

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
    ClassDunmer,
    ClassEarth,
    ClassEldritch,
    ClassFire,
    ClassFrost,
    ClassHoly,
    ClassMind,
    ClassNecromancy,
    ClassPoison,
    ClassRace_Altmer,
    ClassRace_Argonian,
    ClassRace_Bosmer,
    ClassRace_Breton,
    ClassRace_Dunmer,
    ClassRace_Imperial,
    ClassRace_Khajiit,
    ClassRace_Nord,
    ClassRace_Orsimer,
    ClassRace_Other,
    ClassRace_Redguard,
    ClassRace_Vampire,
    ClassRace_Werebeast,
    ClassShadow,
    ClassShock,
    ClassSurvival_Needs,
    ClassSurvival_Wilderness,
    ClassSurvival,
    ClassUtility,
    ClassVampire,
    ClassWater,
    ClassWind,
    ClassWitcher,
    DeliverTouch,
    PowerAction_Bag,
    PowerAction_Bard,
    PowerAction_Bathe,
    PowerAction_Bless,
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
    PowerAction_Influence,
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
    PowerAction,
    PowerAlteration,
    PowerCheat,
    PowerConfig,
    PowerConfigWeatherChanger,
    PowerGrand,
    Spell_Enchant,
    SpellAbsorb_Magicka,
    SpellAbsorb_MagickaCircle,
    SpellAbsorb_MagickaCloak,
    SpellAbsorb_Stamina,
    SpellAbsorb_StaminaCircle,
    SpellAbsorb_StaminaCloak,
    SpellAssist_DamageDruid,
    SpellAssist_MovementSpeedDruid,
    SpellAssist,
    SpellBound_Ammo,
    SpellBound_Armor,
    SpellBound_MiscItem,
    SpellBound_Weapon,
    SpellControl,
    SpellCounter_Astral,
    SpellCounter_BloodDruid,
    SpellCounter_Druid,
    SpellCounter_DruidHeal,
    SpellCounter_Fire,
    SpellCure,
    SpellCurse_Deconstruct,
    SpellCurse_DruidRoot,
    SpellCurse_Shadow,
    SpellCurse,
    SpellDamage_Arcane,
    SpellDamage_ArcaneCloak,
    SpellDamage_ArcaneFire,
    SpellDamage_ArcaneFireCloak,
    SpellDamage_Ash,
    SpellDamage_AshCloak,
    SpellDamage_AshFire,
    SpellDamage_AshFireCloak,
    SpellDamage_Astral,
    SpellDamage_AstralCloak,
    SpellDamage_Blood,
    SpellDamage_BloodCloak,
    SpellDamage_BloodShock,
    SpellDamage_BloodShockCloak,
    SpellDamage_Construct,
    SpellDamage_Deconstruct,
    SpellDamage_DeconstructCloak,
    SpellDamage_Disease,
    SpellDamage_DiseaseCloak,
    SpellDamage_Earth,
    SpellDamage_EarthCloak,
    SpellDamage_Fire,
    SpellDamage_FireArcane,
    SpellDamage_FireArcaneCloak,
    SpellDamage_FireCloak,
    SpellDamage_FireCloakDunmer,
    SpellDamage_FireCold,
    SpellDamage_FireColdCloak,
    SpellDamage_FireShock,
    SpellDamage_FireShockCloak,
    SpellDamage_FireShockFrost,
    SpellDamage_FireShockFrostCloak,
    SpellDamage_Force,
    SpellDamage_ForceCloak,
    SpellDamage_Frost,
    SpellDamage_FrostCloak,
    SpellDamage_FrostFire,
    SpellDamage_FrostFireCloak,
    SpellDamage_Holy,
    SpellDamage_HolyAstral,
    SpellDamage_HolyAstralCloak,
    SpellDamage_HolyCloak,
    SpellDamage_HolyLunar,
    SpellDamage_HolyLunarCloak,
    SpellDamage_Light,
    SpellDamage_LightCloak,
    SpellDamage_Necrotic,
    SpellDamage_NecroticCloak,
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
    SpellDamage_Shock,
    SpellDamage_ShockArc,
    SpellDamage_ShockArcCloak,
    SpellDamage_ShockCloak,
    SpellDamage_ShockStorm,
    SpellDamage_ShockStormCloak,
    SpellDamage_Sonic,
    SpellDamage_SonicCloak,
    SpellDamage_Steam,
    SpellDamage_SteamCloak,
    SpellDamage_Water,
    SpellDamage_WaterCloak,
    SpellDamage_Wind,
    SpellDamage_WindCloak,
    SpellDispel,
    SpellDivination,
    SpellEnchant,
    SpellEnhance_Attack,
    SpellEnhance_Blood,
    SpellEnhance_CarryWeight,
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
    SpellEnhance_Eldritch,
    SpellEnhance_EldritchTome,
    SpellEnhance_Evasion,
    SpellEnhance_EvasionDruid,
    SpellEnhance_Fall,
    SpellEnhance_Flight,
    SpellEnhance_Health,
    SpellEnhance_Jump,
    SpellEnhance_MovementSpeed,
    SpellEnhance_MovementSpeedDruid,
    SpellEnhance_Regen,
    SpellEnhance_RegenShadowInvis,
    SpellEnhance_Sight,
    SpellEnhance_SightKhajiit,
    SpellEnhance_SightVampireBlood,
    SpellEnhance_SightVampireShadow,
    SpellEnhance_SightWerebeast,
    SpellEnhance_SpellCost,
    SpellEnhance_StaminaDruid,
    SpellEnhance_Swim,
    SpellEnhance_WaterBreath,
    SpellEnhance_WaterWalk,
    SpellEthereal,
    SpellForce,
    SpellHarvest,
    SpellHeal_Construct,
    SpellHeal_Daedra,
    SpellHeal_Living,
    SpellHeal_LivingCircle,
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
    SpellMind_Courage,
    SpellMind_Fear,
    SpellMind_FearNord,
    SpellMind_FearVampire,
    SpellMind_Frenzy,
    SpellMind_FrenzyShadow,
    SpellMind_Paralysis,
    SpellMind_Rally,
    SpellParalysis_Ash,
    SpellParalysis_AshCloak,
    SpellParalysis_Druid,
    SpellParalysis,
    SpellProject,
    SpellProtect_Damage,
    SpellProtect_ElementFire,
    SpellProtect_ElementFrost,
    SpellProtect_ElementPoison,
    SpellProtect_ElementShock,
    SpellProtect_Magic,
    SpellProtect_Warmth,
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
    SpellSacrifice_Blood,
    SpellSacrifice,
    SpellShapechange_Vampire,
    SpellShapechange_Werebeast,
    SpellShapechange,
    SpellShield_Druid,
    SpellShield_Warmth,
    SpellSilence,
    SpellSoulTrap,
    SpellSoulTrapCloak,
    SpellSpace_Banish,
    SpellSpace_Teleport,
    SpellSpace,
    SpellStealth_Invisibility,
    SpellStealth_InvisibilityDoomstone,
    SpellStealth_InvisibilityDruid,
    SpellStealth_InvisibilityVampire,
    SpellStealth,
    SpellSummon_Construct,
    SpellSummon_Creature,
    SpellSummon_Daedra,
    SpellSummon_Object,
    SpellSummon_Spirit,
    SpellSummon_Undead,
    SpellTeleport,
    SpellTime,
    SpellTransmute,
    SpellTurnUndeadCircle,
    SpellUnlock,
    SpellWard,
}

// ----------- spell archetypes

pub const ICON_BUFF: EnumSet<SpellKeywords> = enum_set!(
    SpellKeywords::SpellEnhance_Attack
        | SpellKeywords::SpellEnhance_Casting
        | SpellKeywords::SpellEnhance_CastingDruid
        | SpellKeywords::SpellEnhance_CastingEldritch
        | SpellKeywords::SpellEnhance_CritShadowInvis
        | SpellKeywords::SpellEnhance_Damage
        | SpellKeywords::SpellEnhance_DamageArcane
        | SpellKeywords::SpellEnhance_DamageAshFire
        | SpellKeywords::SpellEnhance_DamageBlood
        | SpellKeywords::SpellEnhance_DamageBloodDruid
        | SpellKeywords::SpellEnhance_DamageDruidHunter
        | SpellKeywords::SpellEnhance_DamageFire
        | SpellKeywords::SpellEnhance_DamageFrost
        | SpellKeywords::SpellEnhance_DamageHolyAstral
        | SpellKeywords::SpellEnhance_DamageHolyLunar
        | SpellKeywords::SpellEnhance_DamagePoison
        | SpellKeywords::SpellEnhance_DamagePoisonEldritch
        | SpellKeywords::SpellEnhance_DamageShadow
        | SpellKeywords::SpellEnhance_DamageShock
        | SpellKeywords::SpellEnhance_DamageShockArc
        | SpellKeywords::SpellEnhance_Dodge
        | SpellKeywords::SpellEnhance_EldritchTome
        | SpellKeywords::SpellEnhance_EvasionDruid
        | SpellKeywords::SpellEnhance_Fall
        | SpellKeywords::SpellEnhance_Health
        | SpellKeywords::SpellEnhance_Jump
        | SpellKeywords::SpellEnhance_MovementSpeedDruid
        | SpellKeywords::SpellEnhance_Regen
        | SpellKeywords::SpellEnhance_RegenShadowInvis
        | SpellKeywords::SpellEnhance_StaminaDruid
        | SpellKeywords::SpellEnhance_Swim
        | SpellKeywords::SpellEnhance_WaterWalk
);

pub const ICON_CLOAK: EnumSet<SpellKeywords> = enum_set!(
    SpellKeywords::MagicCloak
        | SpellKeywords::SpellAbsorb_MagickaCloak
        | SpellKeywords::SpellAbsorb_StaminaCloak
        | SpellKeywords::SpellDamage_ArcaneCloak
        | SpellKeywords::SpellDamage_ArcaneFireCloak
        | SpellKeywords::SpellDamage_AshFireCloak
        | SpellKeywords::SpellDamage_BloodCloak
        | SpellKeywords::SpellDamage_BloodShockCloak
        | SpellKeywords::SpellDamage_EarthCloak
        | SpellKeywords::SpellDamage_FireCloak
        | SpellKeywords::SpellDamage_FireCloakDunmer
        | SpellKeywords::SpellDamage_FireShockFrostCloak
        | SpellKeywords::SpellDamage_FrostCloak
        | SpellKeywords::SpellDamage_FrostFireCloak
        | SpellKeywords::SpellDamage_HolyAstralCloak
        | SpellKeywords::SpellDamage_HolyCloak
        | SpellKeywords::SpellDamage_HolyLunarCloak
        | SpellKeywords::SpellDamage_LightCloak
        | SpellKeywords::SpellDamage_NecroticCloak
        | SpellKeywords::SpellDamage_NecroticFireCloak
        | SpellKeywords::SpellDamage_PoisonBugCloak
        | SpellKeywords::SpellDamage_PoisonCloak
        | SpellKeywords::SpellDamage_PoisonEldritchCloak
        | SpellKeywords::SpellDamage_ShadowCloak
        | SpellKeywords::SpellDamage_ShockArcCloak
        | SpellKeywords::SpellDamage_ShockCloak
        | SpellKeywords::SpellDamage_ShockStormCloak
        | SpellKeywords::SpellDamage_SonicCloak
        | SpellKeywords::SpellDamage_SteamCloak
        | SpellKeywords::SpellDamage_WaterCloak
        | SpellKeywords::SpellDamage_WindCloak
);

pub const ICON_FIRE: EnumSet<SpellKeywords> = enum_set!(
    SpellKeywords::SpellDamage_Fire
        | SpellKeywords::SpellDamage_FireCold
        | SpellKeywords::SpellDamage_FireArcane
        | SpellKeywords::SpellDamage_FireShock
        | SpellKeywords::SpellDamage_FrostFire
);

pub const ICON_CIRCLE: EnumSet<SpellKeywords> = enum_set!(
    SpellKeywords::SpellHeal_LivingCircle
        | SpellKeywords::SpellAbsorb_MagickaCircle
        | SpellKeywords::SpellAbsorb_StaminaCircle
        | SpellKeywords::SpellRestore_MagickaCircle
        | SpellKeywords::SpellRestore_StaminaCircle
);

pub const ICON_DRUID: EnumSet<SpellKeywords> =
    enum_set!(SpellKeywords::ClassDruid | SpellKeywords::SpellCounter_DruidHeal);

pub const ICON_CONTROL: EnumSet<SpellKeywords> = enum_set!(
    SpellKeywords::MagicInfluence
        | SpellKeywords::MagicInfluenceCharm
        | SpellKeywords::SpellControl
        | SpellKeywords::SpellMind_Charm
        | SpellKeywords::SpellMind_CharmImperial
        | SpellKeywords::SpellMind_Control
        | SpellKeywords::SpellMind_ControlBosmer
        | SpellKeywords::SpellMind_ControlVampire
);

pub const ICON_EARTH: EnumSet<SpellKeywords> =
    enum_set!(SpellKeywords::ClassEarth | SpellKeywords::SpellDamage_Earth);

pub const ICON_FEAR: EnumSet<SpellKeywords> = enum_set!(
    SpellKeywords::MagicInfluenceFear
        | SpellKeywords::SpellMind_Fear
        | SpellKeywords::SpellMind_FearNord
        | SpellKeywords::SpellMind_FearVampire
);

pub const ICON_FROST: EnumSet<SpellKeywords> = enum_set!(
    SpellKeywords::MagicDamageFrost | SpellKeywords::ClassFrost | SpellKeywords::SpellDamage_Frost
);

pub const ICON_HEALING: EnumSet<SpellKeywords> = enum_set!(
    SpellKeywords::MagicRestoreHealth
        | SpellKeywords::SpellHeal_Daedra
        | SpellKeywords::SpellHeal_Living
        | SpellKeywords::SpellHeal_LivingWater
        | SpellKeywords::SpellHeal_Self
        | SpellKeywords::SpellHeal_SelfCloak
        | SpellKeywords::SpellHeal_Undead
        | SpellKeywords::SpellCure
);

pub const ICON_HOLY: EnumSet<SpellKeywords> = enum_set!(
    SpellKeywords::SpellDamage_Holy | SpellKeywords::ClassHoly | SpellKeywords::MagicTurnUndead
);

pub const ICON_INVISIBILITY: EnumSet<SpellKeywords> = enum_set!(
    SpellKeywords::MagicInvisibility
        | SpellKeywords::SpellStealth_Invisibility
        | SpellKeywords::SpellStealth_InvisibilityDruid
        | SpellKeywords::SpellStealth_InvisibilityDoomstone
        | SpellKeywords::SpellStealth_InvisibilityVampire
);

pub const ICON_LIGHT: EnumSet<SpellKeywords> =
    enum_set!(SpellKeywords::SpellLight | SpellKeywords::Archetype_Light);

pub const ICON_PARALYZE: EnumSet<SpellKeywords> = enum_set!(
    SpellKeywords::MagicParalysis
        | SpellKeywords::SpellMind_Paralysis
        | SpellKeywords::SpellParalysis
        | SpellKeywords::SpellParalysis_Druid
);

pub const ICON_ROOT: EnumSet<SpellKeywords> =
    enum_set!(SpellKeywords::Archetype_Root | SpellKeywords::SpellCurse_DruidRoot);

// pub const RUNE_SPELLS: EnumSet<SpellKeywords> = enum_set!(SpellKeywords::MagicRune);

pub const ICON_SHOCK: EnumSet<SpellKeywords> = enum_set!(
    SpellKeywords::MagicDamageShock
        | SpellKeywords::ClassShock
        | SpellKeywords::SpellDamage_Shock
        | SpellKeywords::SpellDamage_BloodShock
);

pub const ICON_STORM: EnumSet<SpellKeywords> = enum_set!(SpellKeywords::SpellDamage_ShockStorm);

pub const ICON_SUMMON: EnumSet<SpellKeywords> = enum_set!(
    SpellKeywords::MagicSummonFamiliar
        | SpellKeywords::MagicSummonFire
        | SpellKeywords::MagicSummonFrost
        | SpellKeywords::MagicSummonShock
        | SpellKeywords::MagicSummonUndead
        | SpellKeywords::SpellSummon_Construct
        | SpellKeywords::SpellSummon_Creature
        | SpellKeywords::SpellSummon_Daedra
        | SpellKeywords::SpellSummon_Object
        | SpellKeywords::SpellSummon_Spirit
);

pub const ICON_VAMPIRE: EnumSet<SpellKeywords> = enum_set!(SpellKeywords::ClassVampire);

pub const ICON_VISION: EnumSet<SpellKeywords> = enum_set!(
    SpellKeywords::SpellEnhance_Sight
        | SpellKeywords::SpellEnhance_SightKhajiit
        | SpellKeywords::SpellEnhance_SightVampireBlood
        | SpellKeywords::SpellEnhance_SightVampireShadow
        | SpellKeywords::SpellEnhance_SightWerebeast
);

// ----------- spell packs

// Natura
// ClassDruid is critter summons

pub const DARENII_ABYSS: EnumSet<SpellKeywords> = enum_set!(
    SpellKeywords::ClassShadow
        | SpellKeywords::SpellCurse_Shadow
        | SpellKeywords::SpellDamage_Shadow
        | SpellKeywords::SpellDamage_ShadowCloak
);
pub const DARENII_ARCLIGHT: EnumSet<SpellKeywords> = enum_set!(
    SpellKeywords::SpellDamage_ShockArc
        | SpellKeywords::SpellEnhance_DamageShockArc
        | SpellKeywords::SpellDamage_ShockArcCloak
);
pub const DARENII_COLDHARBOUR: EnumSet<SpellKeywords> =
    enum_set!(SpellKeywords::SpellDamage_FireCold);
pub const DARENII_DESECRATION: EnumSet<SpellKeywords> = enum_set!(
    SpellKeywords::SpellDamage_Necrotic
        | SpellKeywords::SpellDamage_Necrotic
        | SpellKeywords::SpellDamage_NecroticFire
);

pub const DARENII_INQUISITION: EnumSet<SpellKeywords> = enum_set!(SpellKeywords::ClassHoly);
pub const DARENII_LUNARIS: EnumSet<SpellKeywords> = enum_set!(
    SpellKeywords::SpellDamage_HolyLunar
        | SpellKeywords::SpellDamage_HolyLunarCloak
        | SpellKeywords::SpellEnhance_DamageHolyLunar
);
// necrom should use tentacles
pub const DARENII_NECROM: EnumSet<SpellKeywords> =
    enum_set!(SpellKeywords::SpellEnhance_Eldritch | SpellKeywords::SpellDamage_PoisonEldritch);

// A Darenii pack & a Kittytail pack both use this.
pub const DARENII_STELLARIS: EnumSet<SpellKeywords> = enum_set!(
    SpellKeywords::ClassAstral
        | SpellKeywords::SpellDamage_HolyAstral
        | SpellKeywords::SpellDamage_HolyAstralCloak
        | SpellKeywords::DAR_SummonAstralWyrm
);
// Kittytail's constellation pack.
pub const CONSTELLATION_SPELLS: EnumSet<SpellKeywords> = enum_set!();

// ----------- color categories

pub const COLOR_ASH: EnumSet<SpellKeywords> = enum_set!(
    SpellKeywords::ClassAsh
        | SpellKeywords::SpellDamage_Ash
        | SpellKeywords::SpellDamage_AshCloak
        | SpellKeywords::SpellDamage_AshFire
        | SpellKeywords::SpellDamage_AshFireCloak
);

pub const COLOR_BLOOD: EnumSet<SpellKeywords> = enum_set!(
    SpellKeywords::ClassBlood
        | SpellKeywords::SpellDamage_Blood
        | SpellKeywords::SpellDamage_BloodCloak
        | SpellKeywords::SpellDamage_BloodShock
        | SpellKeywords::SpellDamage_BloodShockCloak
);

pub const COLOR_BOUND_ITEMS: EnumSet<SpellKeywords> = enum_set!(
    SpellKeywords::SpellBound_Ammo
        | SpellKeywords::SpellBound_Armor
        | SpellKeywords::SpellBound_MiscItem
        | SpellKeywords::SpellBound_Weapon
);

pub const COLOR_EARTH: EnumSet<SpellKeywords> = enum_set!(
    SpellKeywords::ClassEarth
        | SpellKeywords::SpellDamage_Earth
        | SpellKeywords::SpellDamage_EarthCloak
);

pub const COLOR_ELDRITCH: EnumSet<SpellKeywords> = enum_set!(
    SpellKeywords::SpellBound_Weapon
        | SpellKeywords::SpellBound_Armor
        | SpellKeywords::ClassArcane
        | SpellKeywords::SpellDamage_Arcane
        | SpellKeywords::SpellDamage_ArcaneFire
        | SpellKeywords::SpellDamage_ArcaneFireCloak
        | SpellKeywords::Archetype_Guide
);

pub const COLOR_FIRE: EnumSet<SpellKeywords> = enum_set!(
    SpellKeywords::ClassFire
        | SpellKeywords::MagicDamageFire
        | SpellKeywords::SpellDamage_Fire
        | SpellKeywords::SpellDamage_FireCloak
        | SpellKeywords::SpellDamage_FireCloakDunmer
);

pub const COLOR_FROST: EnumSet<SpellKeywords> = enum_set!(
    SpellKeywords::ClassFrost
        | SpellKeywords::MagicDamageFrost
        | SpellKeywords::MagicSummonFrost
        | SpellKeywords::SpellDamage_Frost
        | SpellKeywords::SpellDamage_FrostCloak
        | SpellKeywords::SpellDamage_FrostFire
        | SpellKeywords::SpellDamage_FrostFireCloak
        | SpellKeywords::SpellDamage_FireCold
        | SpellKeywords::SpellDamage_FireColdCloak
        | SpellKeywords::SpellEnhance_DamageFrost
);

pub const COLOR_HOLY: EnumSet<SpellKeywords> = enum_set!(
    SpellKeywords::ClassHoly
        | SpellKeywords::SpellDamage_Holy
        | SpellKeywords::SpellDamage_HolyAstral
        | SpellKeywords::SpellDamage_HolyAstralCloak
);

pub const COLOR_NECROTIC: EnumSet<SpellKeywords> = enum_set!(
    SpellKeywords::SpellEnhance_Eldritch
        | SpellKeywords::SpellDamage_Necrotic
        | SpellKeywords::SpellDamage_NecroticCloak
        | SpellKeywords::SpellDamage_NecroticFire
        | SpellKeywords::SpellDamage_NecroticFireCloak
);

pub const COLOR_POISON: EnumSet<SpellKeywords> = enum_set!(
    SpellKeywords::ClassPoison
        | SpellKeywords::SpellDamage_Poison
        | SpellKeywords::SpellDamage_PoisonBug
        | SpellKeywords::SpellDamage_PoisonBugCloak
        | SpellKeywords::SpellDamage_PoisonCloak
        | SpellKeywords::SpellDamage_PoisonDoomstone
        | SpellKeywords::SpellDamage_PoisonEldritch
        | SpellKeywords::SpellDamage_PoisonEldritchCloak
);

pub const COLOR_SHADOW: EnumSet<SpellKeywords> = enum_set!(
    SpellKeywords::ClassNecromancy
        | SpellKeywords::ClassShadow
        | SpellKeywords::SpellCurse_Shadow
        | SpellKeywords::SpellDamage_Shadow
        | SpellKeywords::SpellDamage_ShadowCloak
);

pub const COLOR_SHOCK: EnumSet<SpellKeywords> = enum_set!(
    SpellKeywords::MagicDamageShock
        | SpellKeywords::ClassShock
        | SpellKeywords::SpellDamage_Shock
        | SpellKeywords::SpellDamage_ShockCloak
        | SpellKeywords::SpellDamage_ShockStorm
        | SpellKeywords::SpellDamage_ShockStormCloak
);

pub const COLOR_SUN: EnumSet<SpellKeywords> = enum_set!(
    SpellKeywords::ClassHoly | SpellKeywords::MAG_MagicDamageSun | SpellKeywords::SpellDamage_Light
);

pub const COLOR_WATER: EnumSet<SpellKeywords> = enum_set!(
    SpellKeywords::ClassWater
        | SpellKeywords::IconWater
        | SpellKeywords::SpellDamage_Steam
        | SpellKeywords::SpellDamage_Water
        | SpellKeywords::SpellDamage_WaterCloak
);

pub const COLOR_WIND: EnumSet<SpellKeywords> = enum_set!(
    SpellKeywords::ClassWind
        | SpellKeywords::IconWind
        | SpellKeywords::SpellDamage_Wind
        | SpellKeywords::SpellDamage_WindCloak
        | SpellKeywords::SpellDamage_Sonic
);

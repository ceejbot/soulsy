//! Soulsy distributes keywords to spells and shouts in vanilla and
//! in various spell packs to identify them for iconnification.
//! It does so because OCF is great, but it only covers objects.

use enumset::{enum_set, EnumSet, EnumSetType};
use strum::{Display, EnumIter, IntoEnumIterator};

pub fn strings_to_keywords(tags: &[String]) -> Vec<SoulsyKeywords> {
    let keywords: Vec<SoulsyKeywords> = tags
        .iter()
        .filter_map(|xs| {
            let sliced = xs.replace("Soulsy_", "");
            if let Ok(subtype) = SoulsyKeywords::try_from(sliced.as_str()) {
                Some(subtype)
            } else {
                None
            }
        })
        .collect();
    keywords
}

impl TryFrom<&str> for SoulsyKeywords {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let keystr = value.replace("Soulsy_", "").to_lowercase();
        let keywd = SoulsyKeywords::iter().find(|xs| keystr == xs.to_string());
        if let Some(k) = keywd {
            Ok(k)
        } else {
            Err(anyhow::anyhow!("not a valid soulsy keyword"))
        }
    }
}

pub const ICON_HINTS: EnumSet<SoulsyKeywords> = enum_set!(
    SoulsyKeywords::ArtBall
        | SoulsyKeywords::ArtBlast
        | SoulsyKeywords::ArtBolt
        | SoulsyKeywords::ArtBreath
        | SoulsyKeywords::ArtChainLightning
        | SoulsyKeywords::ArtFlame
        | SoulsyKeywords::ArtLightning
        | SoulsyKeywords::ArtProjectile
        | SoulsyKeywords::ArtSpike
        | SoulsyKeywords::ArtStorm
        | SoulsyKeywords::ArtTornado
        | SoulsyKeywords::ArtWall
);

#[derive(Debug, Hash, Display, EnumIter, EnumSetType)]
#[strum(serialize_all = "lowercase")]
pub enum SoulsyKeywords {
    // Some vanilla and mod spell archetypes to mark with keywords
    Archetype_BoundWeapon,
    Archetype_Buff,
    Archetype_CarryWeight,
    Archetype_Cloak,
    Archetype_Cure,
    Archetype_Damage,
    Archetype_Guide,
    Archetype_Heal,
    Archetype_Light,
    Archetype_Protect,
    Archetype_Reanimate,
    Archetype_Reflect,
    Archetype_Resist,
    Archetype_Root,
    Archetype_Silence,
    Archetype_SoulTrap,
    Archetype_Summon,
    Archetype_Time,
    Archetype_Vision,
    Archetype_Waterbreathing,
    Archetype_Waterwalking,
    Archetype_WeaponBuff,

    // damage types from spell packs
    MagicDamage_Arcane,
    MagicDamage_Arclight,
    MagicDamage_Astral,
    MagicDamage_Bleed,
    MagicDamage_ColdFire,
    MagicDamage_Disease,
    MagicDamage_Earth,
    MagicDamage_Lunar,
    MagicDamage_Necrotic,
    MagicDamage_Poison,
    MagicDamage_Shadow,
    MagicDamage_Sun,
    MagicDamage_Water,
    MagicDamage_Wind,

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
    OCF_MgefSpellDamage_FireCold,
    MAG_MagicDamageSun,
    IconWind,
    IconWater,

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

    /*
    OCF_MgefClassArcane
OCF_MgefClassArtificer
OCF_MgefClassAsh
OCF_MgefClassAstral
OCF_MgefClassBard
OCF_MgefClassBlood
OCF_MgefClassDruid
OCF_MgefClassEarth
OCF_MgefClassEldritch
OCF_MgefClassFire
OCF_MgefClassFrost
OCF_MgefClassHoly
OCF_MgefClassNecromancy
OCF_MgefClassPoison
OCF_MgefClassRace_Altmer
OCF_MgefClassRace_Argonian
OCF_MgefClassRace_Breton
OCF_MgefClassRace_Orsimer
OCF_MgefClassRace_Redguard
OCF_MgefClassRace_Vampire
OCF_MgefClassRace_Werebeast
OCF_MgefClassShadow
OCF_MgefClassShock
OCF_MgefClassSurvival
OCF_MgefClassSurvival_Needs
OCF_MgefClassSurvival_Wilderness
OCF_MgefClassUtility
OCF_MgefClassWater
OCF_MgefClassWind
OCF_MgefDeliverTouch
OCF_MgefPowerAction
OCF_MgefPowerAction_Bag
OCF_MgefPowerAction_Bard
OCF_MgefPowerAction_Bathe
OCF_MgefPowerAction_Bless
OCF_MgefPowerAction_Broom
OCF_MgefPowerAction_BuryCorpse
OCF_MgefPowerAction_Campfire
OCF_MgefPowerAction_Coin
OCF_MgefPowerAction_CommandFollower
OCF_MgefPowerAction_Craft
OCF_MgefPowerAction_FillWater
OCF_MgefPowerAction_Goggles
OCF_MgefPowerAction_GogglesSight
OCF_MgefPowerAction_HarvestCorpse
OCF_MgefPowerAction_HarvestGather
OCF_MgefPowerAction_HarvestWood
OCF_MgefPowerAction_Horse
OCF_MgefPowerAction_HorseHorn
OCF_MgefPowerAction_HorseWhistle
OCF_MgefPowerAction_Influence
OCF_MgefPowerAction_InfluenceCancelRetreat
OCF_MgefPowerAction_InfluenceEngage
OCF_MgefPowerAction_Instincts
OCF_MgefPowerAction_Lantern
OCF_MgefPowerAction_PeekKeyhole
OCF_MgefPowerAction_PitchTent
OCF_MgefPowerAction_Potion
OCF_MgefPowerAction_Pray
OCF_MgefPowerAction_Relax
OCF_MgefPowerAction_Speech
OCF_MgefPowerAction_StatusFrostfall
OCF_MgefPowerAction_StatusSunhelm
OCF_MgefPowerAction_TameAnimal
OCF_MgefPowerAction_Train
OCF_MgefPowerAction_WeaponGrip
OCF_MgefPowerCheat
OCF_MgefPowerConfig
OCF_MgefPowerGrand
OCF_MgefSpellAbsorb_Magicka
OCF_MgefSpellAbsorb_MagickaCloak
OCF_MgefSpellAbsorb_Stamina
OCF_MgefSpellAbsorb_StaminaCloak
OCF_MgefSpellAssist_DamageDruid
OCF_MgefSpellAssist_MovementSpeedDruid
OCF_MgefSpellBound
OCF_MgefSpellBound_Ammo
OCF_MgefSpellBound_Armor
OCF_MgefSpellBound_Weapon
OCF_MgefSpellControl
OCF_MgefSpellCounter_Astral
OCF_MgefSpellCounter_BloodDruid
OCF_MgefSpellCounter_Druid
OCF_MgefSpellCounter_DruidHeal
OCF_MgefSpellCounter_Fire
OCF_MgefSpellCurse_Deconstruct
OCF_MgefSpellCurse_DruidRoot
OCF_MgefSpellCurse_Shadow
OCF_MgefSpellDamage_Arcane
OCF_MgefSpellDamage_ArcaneCloak
OCF_MgefSpellDamage_ArcaneFire
OCF_MgefSpellDamage_ArcaneFireCloak
OCF_MgefSpellDamage_AshFire
OCF_MgefSpellDamage_AshFireCloak
OCF_MgefSpellDamage_Blood
OCF_MgefSpellDamage_BloodCloak
OCF_MgefSpellDamage_Deconstruct
OCF_MgefSpellDamage_Earth
OCF_MgefSpellDamage_EarthCloak
OCF_MgefSpellDamage_Fire
OCF_MgefSpellDamage_FireCloak
OCF_MgefSpellDamage_FireCloakDunmer
OCF_MgefSpellDamage_Force
OCF_MgefSpellDamage_Frost
OCF_MgefSpellDamage_FrostFire
OCF_MgefSpellDamage_FrostFireCloak
OCF_MgefSpellDamage_HolyAstral
OCF_MgefSpellDamage_HolyAstralCloak
OCF_MgefSpellDamage_HolyLunar
OCF_MgefSpellDamage_HolyLunarCloak
OCF_MgefSpellDamage_Light
OCF_MgefSpellDamage_NecroticFire
OCF_MgefSpellDamage_NecroticFireCloak
OCF_MgefSpellDamage_Poison
OCF_MgefSpellDamage_PoisonBug
OCF_MgefSpellDamage_PoisonBugCloak
OCF_MgefSpellDamage_PoisonCloak
OCF_MgefSpellDamage_PoisonDoomstone
OCF_MgefSpellDamage_PoisonEldritch
OCF_MgefSpellDamage_PoisonEldritchCloak
OCF_MgefSpellDamage_Shadow
OCF_MgefSpellDamage_ShadowCloak
OCF_MgefSpellDamage_ShockArc
OCF_MgefSpellDamage_ShockArcCloak
OCF_MgefSpellDamage_Sonic
OCF_MgefSpellDamage_Steam
OCF_MgefSpellDamage_Water
OCF_MgefSpellDamage_WaterCloak
OCF_MgefSpellDamage_Wind
OCF_MgefSpellDamage_WindCloak
OCF_MgefSpellDispel
OCF_MgefSpellDivination
OCF_MgefSpellEnchant
OCF_MgefSpellEnhance_Attack
OCF_MgefSpellEnhance_AttackEldritch
OCF_MgefSpellEnhance_Casting
OCF_MgefSpellEnhance_CastingDruid
OCF_MgefSpellEnhance_CastingEldritch
OCF_MgefSpellEnhance_CritShadowInvis
OCF_MgefSpellEnhance_Damage
OCF_MgefSpellEnhance_DamageArcane
OCF_MgefSpellEnhance_DamageAshFire
OCF_MgefSpellEnhance_DamageBlood
OCF_MgefSpellEnhance_DamageBloodDruid
OCF_MgefSpellEnhance_DamageDruidHunter
OCF_MgefSpellEnhance_DamageFire
OCF_MgefSpellEnhance_DamageFrost
OCF_MgefSpellEnhance_DamageHolyAstral
OCF_MgefSpellEnhance_DamageHolyLunar
OCF_MgefSpellEnhance_DamagePoison
OCF_MgefSpellEnhance_DamagePoisonEldritch
OCF_MgefSpellEnhance_DamageShadow
OCF_MgefSpellEnhance_DamageShock
OCF_MgefSpellEnhance_DamageShockArc
OCF_MgefSpellEnhance_Dodge
OCF_MgefSpellEnhance_EldritchTome
OCF_MgefSpellEnhance_EvasionDruid
OCF_MgefSpellEnhance_Fall
OCF_MgefSpellEnhance_Health
OCF_MgefSpellEnhance_Jump
OCF_MgefSpellEnhance_MovementSpeedDruid
OCF_MgefSpellEnhance_Regen
OCF_MgefSpellEnhance_RegenShadowInvis
OCF_MgefSpellEnhance_Sight
OCF_MgefSpellEnhance_SightKhajiit
OCF_MgefSpellEnhance_SightVampire
OCF_MgefSpellEnhance_SightVampireBlood
OCF_MgefSpellEnhance_SightVampireShadow
OCF_MgefSpellEnhance_SightWerebeast
OCF_MgefSpellEnhance_StaminaDruid
OCF_MgefSpellEnhance_Swim
OCF_MgefSpellEnhance_WaterWalk
OCF_MgefSpellEthereal
OCF_MgefSpellForce
OCF_MgefSpellHarvest
OCF_MgefSpellHeal_Daedra
OCF_MgefSpellHeal_Living
OCF_MgefSpellHeal_LivingWater
OCF_MgefSpellHeal_Self
OCF_MgefSpellHeal_SelfCloak
OCF_MgefSpellHeal_Undead
OCF_MgefSpellLight
OCF_MgefSpellMind_Charm
OCF_MgefSpellMind_CharmImperial
OCF_MgefSpellMind_Control
OCF_MgefSpellMind_ControlBosmer
OCF_MgefSpellMind_ControlVampire
OCF_MgefSpellMind_Fear
OCF_MgefSpellMind_FearNord
OCF_MgefSpellMind_FrenzyShadow
OCF_MgefSpellMind_Paralysis
OCF_MgefSpellParalysis
OCF_MgefSpellParalysis_Druid
OCF_MgefSpellProtect_Damage
OCF_MgefSpellProtect_DamageEldritch
OCF_MgefSpellProtect_Magic
OCF_MgefSpellReanimate
OCF_MgefSpellReanimateDoomstone
OCF_MgefSpellReflect_Druid
OCF_MgefSpellRestore_Exposure
OCF_MgefSpellRestore_Magicka
OCF_MgefSpellRestore_MagickaCircle
OCF_MgefSpellRestore_MagickaWater
OCF_MgefSpellRestore_Stamina
OCF_MgefSpellRestore_StaminaCircle
OCF_MgefSpellRestore_StaminaDruid
OCF_MgefSpellRestore_Warmth
OCF_MgefSpellSacrifice
OCF_MgefSpellSacrifice_Blood
OCF_MgefSpellShapechange
OCF_MgefSpellShapechange_Creature
OCF_MgefSpellShapechange_Vampire
OCF_MgefSpellShapechange_Werebeast
OCF_MgefSpellShield_Warmth
OCF_MgefSpellSoulTrap
OCF_MgefSpellSpace
OCF_MgefSpellSpace_Teleport
OCF_MgefSpellStealth
OCF_MgefSpellStealth_Invisibility
OCF_MgefSpellStealth_InvisibilityDoomstone
OCF_MgefSpellStealth_InvisibilityDruid
OCF_MgefSpellSummon_Construct
OCF_MgefSpellSummon_Creature
OCF_MgefSpellSummon_Daedra
OCF_MgefSpellSummon_DaedraEldritch
OCF_MgefSpellSummon_Object
OCF_MgefSpellSummon_Spirit
OCF_MgefSpellSummon_SpiritFrost
OCF_MgefSpellSummon_SpiritShadow
OCF_MgefSpellSummon_Undead
OCF_MgefSpellTime
OCF_MgefSpellTransmute
OCF_MgefSpellTurnUndeadCircle
OCF_MgefSpellUnlock
 */
}

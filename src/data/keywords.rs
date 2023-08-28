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
}

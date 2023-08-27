//! Soulsy distributes keywords to spells and shouts in vanilla and
//! in various spell packs to identify them for iconnification.
//! It does so because OCF is great, but it only covers objects.

use strum::{Display, EnumIter, IntoEnumIterator};

pub fn strings_to_keywords(tags: Vec<String>) -> Vec<SoulsyKeywords> {
    let keywords: Vec<SoulsyKeywords> = tags
        .iter()
        .filter_map(|xs| {
            if let Ok(subtype) = SoulsyKeywords::try_from(xs.as_str()) {
                Some(subtype)
            } else {
                None
            }
        })
        .collect();
    keywords
}

#[derive(Debug, Clone, Hash, Display, EnumIter, Eq, PartialEq)]
pub enum SoulsyKeywords {
    // Some vanilla and mod spell archetypes to mark with keywords
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
    Archetype_Time,
    Archetype_Vision,
    Archetype_Waterbreathing,
    Archetype_Waterwalking,
    Archetype_WeaponBuff,

    // damage types from spell packs
    MagicDamage_Bleed,
    MagicDamage_ColdFire,
    MagicDamage_Disease,
    MagicDamage_Earth,
    MagicDamage_Lunar,
    MagicDamage_Magic,
    MagicDamage_Necrotic,
    MagicDamage_Poison,
    MagicDamage_Shadow,
    MagicDamage_Sun,
    MagicDamage_Water,
    MagicDamage_Wind,

    // Hints about which art to use.
    ArtBall,
    ArtFlame,
    ArtLightning,
    ArtProjectile,
    ArtSpike,
    ArtStorm,
    ArtTornado,
    ArtWall,

    // Bound weapon types
    BoundAxeOneHanded,
    BoundAxeTwoHanded,
    BoundBow,
    BoundDagger,
    BoundHammer,
    BoundMace,
    BoundShield,
    BoundSwordOneHanded,
    BoundSwordTwoHanded,

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

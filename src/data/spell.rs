#![allow(non_snake_case, non_camel_case_types)]

use cxx::let_cxx_string;

use super::base::{self, BaseType};
use super::color::InvColor;
use super::game_enums::{ActorValue};
use super::icons::Icon;
use super::keywords::*;
use super::magic::{MagicDamageType, SpellData};
use super::weapon::WeaponType;
use super::HasIcon;
use crate::plugin::{formSpecToHudItem, Color};

// Spells must be classified by querying game data about actor values, resist types,
// and spell archetypes. SpellData holds Rust expressions of the C++ enum values plus
// a collection of other data about the spell to help us categorize it.
// In most cases, we choose the primary actor value from the most expensive effect
// of a spell or potion. For some we have to consider the secondary effect.
// If we have useful keywords, that's great, but we can't count on them.

#[derive(Default, Clone, Debug, Eq, Hash, PartialEq)]
pub struct SpellType {
    pub data: SpellData,
    pub variant: SpellVariant,
}

impl SpellType {
    pub fn new(mut data: SpellData, tags: Vec<String>) -> Self {
        // well, this will be fun™

        let _color = base::color_from_keywords(&tags);
        let keywords = strings_to_keywords(&tags);
        log::info!("{keywords:?}");

        let damage = if keywords.contains(&SoulsyKeywords::MagicDamageFire) {
            MagicDamageType::Fire
        } else if keywords.contains(&SoulsyKeywords::MagicDamageFrost) {
            MagicDamageType::Frost
        } else if keywords.contains(&SoulsyKeywords::MagicDamageShock) {
            MagicDamageType::Shock
        } else if keywords.contains(&SoulsyKeywords::MagicDamage_Arcane) {
            MagicDamageType::Arcane
        } else if keywords.contains(&SoulsyKeywords::MagicDamage_Arclight) {
            MagicDamageType::Arclight
        } else if keywords.contains(&SoulsyKeywords::MagicDamage_Astral) {
            MagicDamageType::Astral
        } else if keywords.contains(&SoulsyKeywords::MagicDamage_Bleed) {
            MagicDamageType::Bleed
        } else if keywords.contains(&SoulsyKeywords::MagicDamage_ColdFire) {
            MagicDamageType::ColdFire
        } else if keywords.contains(&SoulsyKeywords::MagicDamage_Disease) {
            MagicDamageType::Disease
        } else if keywords.contains(&SoulsyKeywords::MagicDamage_Earth) {
            MagicDamageType::Earth
        } else if keywords.contains(&SoulsyKeywords::MagicDamage_Lunar) {
            MagicDamageType::Lunar
        } else if keywords.contains(&SoulsyKeywords::MagicDamage_Necrotic) {
            MagicDamageType::Necrotic
        } else if keywords.contains(&SoulsyKeywords::MagicDamage_Poison) {
            MagicDamageType::Poison
        } else if keywords.contains(&SoulsyKeywords::MagicDamage_Shadow) {
            MagicDamageType::Shadow
        } else if keywords.contains(&SoulsyKeywords::MagicDamage_Sun) {
            MagicDamageType::Sun
        } else if keywords.contains(&SoulsyKeywords::MagicDamage_Water) {
            MagicDamageType::Water
        } else if keywords.contains(&SoulsyKeywords::IconWater) {
            MagicDamageType::Water
        } else if keywords.contains(&SoulsyKeywords::MagicDamage_Wind) {
            MagicDamageType::Wind
        } else if keywords.contains(&SoulsyKeywords::IconWind) {
            MagicDamageType::Wind
        } else {
            data.damage
        };
        data.damage = damage;

        let variant = if keywords.contains(&SoulsyKeywords::Archetype_Buff) {
            SpellVariant::Buff
        } else if keywords.contains(&SoulsyKeywords::Archetype_CarryWeight) {
            SpellVariant::CarryWeight
        } else if keywords.contains(&SoulsyKeywords::Archetype_Cure) {
            SpellVariant::Cure
        } else if keywords.contains(&SoulsyKeywords::Archetype_Damage) {
            SpellVariant::Damage(data.damage.clone())
        } else if keywords.contains(&SoulsyKeywords::Archetype_Guide) {
            SpellVariant::Guide
        } else if keywords.contains(&SoulsyKeywords::Archetype_Heal) {
            SpellVariant::Heal
        } else if keywords.contains(&SoulsyKeywords::Archetype_Light) {
            SpellVariant::Light
        } else if keywords.contains(&SoulsyKeywords::Archetype_Protect) {
            SpellVariant::Unknown
        } else if keywords.contains(&SoulsyKeywords::Archetype_Reanimate) {
            SpellVariant::Reanimate
        } else if keywords.contains(&SoulsyKeywords::Archetype_Reflect) {
            SpellVariant::Reflect
        } else if keywords.contains(&SoulsyKeywords::Archetype_Resist) {
            SpellVariant::Unknown
        } else if keywords.contains(&SoulsyKeywords::Archetype_Root) {
            SpellVariant::Root
        } else if keywords.contains(&SoulsyKeywords::Archetype_Silence) {
            SpellVariant::Silence
        } else if keywords.contains(&SoulsyKeywords::Archetype_SoulTrap) {
            SpellVariant::SoulTrap
        } else if keywords.contains(&SoulsyKeywords::Archetype_Time) {
            SpellVariant::SlowTime
        } else if keywords.contains(&SoulsyKeywords::Archetype_Vision) {
            SpellVariant::Detect
        } else if keywords.contains(&SoulsyKeywords::Archetype_Waterbreathing) {
            SpellVariant::Waterbreathing
        } else if keywords.contains(&SoulsyKeywords::Archetype_Waterwalking) {
            SpellVariant::Waterwalking
        } else if keywords.contains(&SoulsyKeywords::Archetype_WeaponBuff) {
            SpellVariant::EnhanceWeapon
        } else {
            SpellVariant::Unknown
        };


        if matches!(variant,  SpellVariant::Unknown) {
            log::debug!("Falling back to default spell variant; data: {data:?}");
            log::debug!("    keywords: {tags:?}");
        };

        Self { data, variant }
    }
}

impl HasIcon for SpellType {
    fn color(&self) -> Color {
        match &self.variant {
            SpellVariant::Unknown => Color::default(),
            SpellVariant::BoundWeapon(_) => InvColor::Eldritch.color(),
            SpellVariant::Burden => Color::default(),
            SpellVariant::Cure => InvColor::Green.color(),
            SpellVariant::Damage(t) => t.color(),
            SpellVariant::Demoralize => Color::default(),
            SpellVariant::Detect => Color::default(),
            SpellVariant::CarryWeight => Color::default(),
            SpellVariant::Guide => InvColor::Eldritch.color(),
            SpellVariant::Heal => InvColor::Green.color(),
            SpellVariant::Light => InvColor::Eldritch.color(),
            SpellVariant::Reanimate => Color::default(),
            SpellVariant::Reflect => Color::default(),
            SpellVariant::Rune => Color::default(),
            SpellVariant::SoulTrap => InvColor::Eldritch.color(),
            SpellVariant::Summon => Color::default(),
            SpellVariant::Teleport => Color::default(),
            SpellVariant::TurnUndead => InvColor::Sun.color(),
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
                BoundType::Dagger => Icon::WeaponDagger.icon_file(),
                BoundType::Greatsword => Icon::WeaponSwordOneHanded.icon_file(),
                BoundType::Hammer => Icon::WeaponHammer.icon_file(),
                BoundType::Mace => Icon::WeaponMace.icon_file(),
                BoundType::Shield => Icon::ArmorShieldHeavy.icon_file(),
                BoundType::Sword => Icon::WeaponSwordOneHanded.icon_file(),
                BoundType::WarAxe => Icon::WeaponAxeOneHanded.icon_file(),
                BoundType::Unknown => Icon::WeaponSwordOneHanded.icon_file(),
            },
            SpellVariant::Burden => self.icon_fallback(),
            SpellVariant::Cure => Icon::SpellCure.icon_file(),
            SpellVariant::Damage(t) => t.icon_file(),
            SpellVariant::Banish => self.icon_fallback(),
            SpellVariant::Blizzard => self.icon_fallback(),
            SpellVariant::Calm => self.icon_fallback(),
            SpellVariant::CarryWeight => Icon::SpellFeather.icon_file(),
            SpellVariant::Cloak(_) => Icon::ArmorCloak.icon_file(),
            SpellVariant::Demoralize => Icon::SpellFear.icon_file(),
            SpellVariant::Detect => Icon::SpellDetect.icon_file(),
            SpellVariant::EnhanceWeapon => Icon::SpellSharpen.icon_file(),
            SpellVariant::Fear => Icon::SpellFear.icon_file(),
            SpellVariant::Fireball => Icon::SpellFireball.icon_file(),
            SpellVariant::Firebolt => Icon::SpellFireDual.icon_file(),
            SpellVariant::FireboltStorm => Icon::SpellMeteor.icon_file(),
            SpellVariant::FireWall => Icon::SpellFireWall.icon_file(),
            SpellVariant::Frost => Icon::SpellFrost.icon_file(),
            SpellVariant::FrostWall => Icon::SpellFrostWall.icon_file(),
            SpellVariant::Guide => Icon::SpellWisp.icon_file(),
            SpellVariant::Heal => Icon::SpellHeal.icon_file(),
            SpellVariant::IceSpike => Icon::SpellIceShard.icon_file(),
            SpellVariant::IceStorm => self.icon_fallback(),
            SpellVariant::IcySpear => Icon::SpellIceShard.icon_file(),
            SpellVariant::Invisibility => self.icon_fallback(),
            SpellVariant::Light => Icon::SpellLight.icon_file(),
            SpellVariant::LightningBolt => self.icon_fallback(), // SpellStormblast
            SpellVariant::LightningStorm => Icon::SpellChainLightning.icon_file(),
            SpellVariant::Mayhem => self.icon_fallback(),
            SpellVariant::Pacify => self.icon_fallback(),
            SpellVariant::Paralyze => self.icon_fallback(),
            SpellVariant::Rally => self.icon_fallback(),
            SpellVariant::Reanimate => Icon::SpellReanimate.icon_file(),
            SpellVariant::Reflect => Icon::SpellReflect.icon_file(),
            SpellVariant::Root => Icon::SpellRoot.icon_file(),
            SpellVariant::Rune => Icon::SpellRune.icon_file(),
            SpellVariant::Shock => Icon::SpellShockStrong.icon_file(),
            SpellVariant::Silence => Icon::SpellSilence.icon_file(),
            SpellVariant::SlowTime => Icon::SpellTime.icon_file(),
            SpellVariant::SoulTrap => Icon::SpellSoultrap.icon_file(),
            SpellVariant::Sparks => Icon::SpellShock.icon_file(),
            SpellVariant::StormWall => self.icon_fallback(),
            SpellVariant::Summon => Icon::SpellSummon.icon_file(),
            SpellVariant::Teleport => Icon::SpellTeleport.icon_file(),
            SpellVariant::Tornado => Icon::SpellTornado.icon_file(),
            SpellVariant::Thorns => self.icon_fallback(),
            SpellVariant::Thunderbolt => Icon::SpellLightningBlast.icon_file(),
            SpellVariant::TurnUndead => Icon::SpellHoly.icon_file(),
            SpellVariant::Ward => Icon::SpellWard.icon_file(),
            SpellVariant::Waterbreathing => self.icon_fallback(),
            _ => self.icon_fallback()
        }
    }

    fn icon_fallback(&self) -> String {
        self.data.school.icon_file()
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
    Buff,
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
    EnhanceWeapon,
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
    Root,
    Rune,
    Shock,
    Silence,
    SlowTime,
    Sparks,
    SoulTrap,
    StormWall,
    Summon,
    Teleport,
    Thorns,
    Thunderbolt,
    Tornado,
    Transmute,
    TurnUndead,
    Ward,
    Waterbreathing,
    Waterwalking,
}

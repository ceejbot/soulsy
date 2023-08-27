#![allow(non_snake_case, non_camel_case_types)]

use cxx::let_cxx_string;

use super::base::{self, BaseType};
use super::color::InvColor;
use super::game_enums::{ActorValue, SpellArchetype};
use super::icons::Icon;
use super::magic::{enumset_contains_any, MagicDamageType, SpellData, SpellKeyword};
use super::weapon::WeaponType;
use super::HasIcon;
use crate::data::magic;
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
    pub fn new(mut data: SpellData, keywords: Vec<String>) -> Self {
        // well, this will be fun™

        let _color = base::color_from_keywords(&keywords);

        log::info!("{keywords:?}");

        let tags: Vec<SpellKeyword> = keywords
            .iter()
            .filter_map(|xs| {
                if let Ok(subtype) = SpellKeyword::try_from(xs.as_str()) {
                    Some(subtype)
                } else {
                    None
                }
            })
            .collect();

        // Override the damage field if we have better info.
        data.damage = if tags.contains(&SpellKeyword::MAG_MagicDamageBleed) {
            MagicDamageType::Bleed
        } else if tags.contains(&SpellKeyword::IconMagicEarth) {
            MagicDamageType::Earth
        } else if tags.contains(&SpellKeyword::MagicDamageLunar) {
            MagicDamageType::Lunar
        } else if tags.contains(&SpellKeyword::MAG_MagicDamagePoison) {
            MagicDamageType::Poison
        } else if tags.contains(&SpellKeyword::MAG_MagicDamageStamina) {
            MagicDamageType::Stamina
        } else if tags.contains(&SpellKeyword::MAG_MagicDamageSun) {
            MagicDamageType::Sun
        } else if tags.contains(&SpellKeyword::IconMagicWater) {
            MagicDamageType::Water // natura spell pack
        } else if tags.contains(&SpellKeyword::IconMagicWind) {
            MagicDamageType::Wind // natura
        } else if enumset_contains_any(&magic::DAR_ELDRITCH, &tags) {
            MagicDamageType::Necrotic // necrom spell pack
        } else if enumset_contains_any(&magic::DAR_ABYSS, &tags) {
            MagicDamageType::Shadow // abyss spell pack
        } else {
            data.damage
        };

        // Use keywords to classify if we have them. If we fail to classify,
        // we dig into the spell data struct.
        let variant = if tags.contains(&SpellKeyword::MAG_MagicEffectLight) {
            Some(SpellVariant::Light)
        } else if tags.contains(&SpellKeyword::MAG_MagicInfluenceParalysis) {
            Some(SpellVariant::Paralyze)
        } else if tags.contains(&SpellKeyword::MAG_MagicInfluenceSilence) {
            Some(SpellVariant::Silence)
        } else if tags.contains(&SpellKeyword::MAG_MagicSoulTrap) {
            Some(SpellVariant::SoulTrap)
        } else if tags.contains(&SpellKeyword::MAG_MagicSummonReanimate) {
            Some(SpellVariant::Reanimate)
        } else if enumset_contains_any(&magic::BOUND_WEAPON, &tags) {
            let weaptype = bound_weapon_type(data.associated.clone());
            Some(SpellVariant::BoundWeapon(weaptype))
        } else if tags.contains(&SpellKeyword::MagicWeaponSpeed) {
            Some(SpellVariant::EnhanceWeapon)
        } else if tags.contains(&SpellKeyword::MAG_MagicWeaponEnchantment) {
            Some(SpellVariant::EnhanceWeapon)
        } else if tags.contains(&SpellKeyword::MAG_MagicWeightSpell) {
            Some(SpellVariant::CarryWeight)
        } else if tags.contains(&SpellKeyword::MagicInfluence) {
            Some(SpellVariant::Calm)
        } else if tags.contains(&SpellKeyword::MagicInfluenceFear) {
            Some(SpellVariant::Fear)
        } else if tags.contains(&SpellKeyword::MagicInfluenceCharm) {
            Some(SpellVariant::Calm)
        } else if tags.contains(&SpellKeyword::MagicInfluenceFear) {
            Some(SpellVariant::Fear)
        // } else if tags.contains(&SpellKeyword::MagicInfluenceFrenzy) {
        //  Some(SpellVariant::Frenzy)
        } else if tags.contains(&SpellKeyword::MagicInvisibility) {
            Some(SpellVariant::Invisibility)
        } else if tags.contains(&SpellKeyword::MagicNightEye) {
            Some(SpellVariant::Detect)
        } else if tags.contains(&SpellKeyword::MagicParalysis) {
            Some(SpellVariant::Paralyze)
        } else if tags.contains(&SpellKeyword::MagicRune) {
            Some(SpellVariant::Rune)
        } else if tags.contains(&SpellKeyword::MagicWard) {
            Some(SpellVariant::Ward)
        } else if tags.contains(&SpellKeyword::MAG_PoisonCloakSpell) {
            Some(SpellVariant::Cloak(MagicDamageType::Poison))
        } else if tags.contains(&SpellKeyword::MagicFlameCloak) {
            Some(SpellVariant::Cloak(MagicDamageType::Fire))
        } else if tags.contains(&SpellKeyword::NAT_MagicRoot) {
            Some(SpellVariant::Root)
        } else {
            match data.archetype {
                SpellArchetype::ValueModifier => classify_value_mod_archetype(&data),
                SpellArchetype::PeakValueModifier => classify_value_mod_archetype(&data),
                SpellArchetype::DualValueModifier => classify_value_mod_archetype(&data),
                //SpellArchetype::Absorb => todo!(),
                //SpellArchetype::Banish => todo!(),
                SpellArchetype::Calm => Some(SpellVariant::Calm), //do I have one?
                SpellArchetype::BoundWeapon => {
                    let weaptype = bound_weapon_type(data.associated.clone());
                    Some(SpellVariant::BoundWeapon(weaptype))
                }
                SpellArchetype::CureDisease => Some(SpellVariant::Cure),
                SpellArchetype::CurePoison => Some(SpellVariant::Cure),
                SpellArchetype::CureParalysis => Some(SpellVariant::Cure),
                SpellArchetype::Demoralize => Some(SpellVariant::Demoralize),
                SpellArchetype::DetectLife => Some(SpellVariant::Detect),
                SpellArchetype::Guide => Some(SpellVariant::Guide),
                SpellArchetype::Light => Some(SpellVariant::Light),
                SpellArchetype::Reanimate => Some(SpellVariant::Reanimate),
                SpellArchetype::SoulTrap => Some(SpellVariant::SoulTrap),
                SpellArchetype::SummonCreature => Some(SpellVariant::Summon),
                SpellArchetype::Cloak => Some(SpellVariant::Cloak(data.damage.clone())),
                //SpellArchetype::CommandSummoned => todo!(),
                //SpellArchetype::Darkness => todo!(),
                // SpellArchetype::Disarm => Some(SpellVariant::Disarm),
                //SpellArchetype::Disguise => todo!(),
                //SpellArchetype::Dispel => todo!(),
                SpellArchetype::EnhanceWeapon => Some(SpellVariant::EnhanceWeapon),
                //SpellArchetype::Etherealize => todo!(),
                //SpellArchetype::Frenzy => todo!(),
                //SpellArchetype::GrabActor => todo!(),
                //SpellArchetype::Invisibility => todo!(),
                //SpellArchetype::Lock => todo!(),
                SpellArchetype::NightEye => Some(SpellVariant::Detect),
                //SpellArchetype::Open => todo!(),
                //SpellArchetype::Paralysis => todo!(),
                //SpellArchetype::Rally => todo!(),
                SpellArchetype::SlowTime => Some(SpellVariant::SlowTime),
                SpellArchetype::SpawnHazard => {
                    // frostwall and firewall here?
                    log::debug!("spawn hazard here");
                    match data.damage {
                        MagicDamageType::Earth => todo!(),
                        MagicDamageType::Fire => Some(SpellVariant::FireWall),
                        MagicDamageType::Frost => Some(SpellVariant::FrostWall),
                        MagicDamageType::Shock => Some(SpellVariant::StormWall),
                        MagicDamageType::Wind => Some(SpellVariant::Tornado),
                        _ => None,
                    }
                }
                //SpellArchetype::Telekinesis => todo!(),
                SpellArchetype::TurnUndead => Some(SpellVariant::TurnUndead),
                _ => None,
            }
        };

        let variant = if let Some(v) = variant {
            v
        } else {
            log::debug!("Falling back to default spell variant; data: {data:?}");
            SpellVariant::Unknown
        };

        Self { data, variant }
    }
}

fn classify_value_mod_archetype(data: &SpellData) -> Option<SpellVariant> {
    if data.hostile {
        // can repeat same match as below for stats if necessary
        return Some(SpellVariant::Damage(data.damage.clone()));
    }

    // positive effects
    match data.effect {
        ActorValue::Health => Some(SpellVariant::Heal),
        ActorValue::DamageResist => {
            // various oak bark spells etc
            todo!()
        }
        ActorValue::Stamina => {
            // respite etc
            todo!()
        }
        ActorValue::WaterBreathing => Some(SpellVariant::Waterbreathing),
        _ => None,
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
        }
    }

    fn icon_fallback(&self) -> String {
        self.data.school.icon_file()
    }
}

fn bound_weapon_type(assoc: String) -> BoundType {
    if assoc.is_empty() {
        return BoundType::Unknown;
    }

    let_cxx_string!(form_spec = assoc);
    let assoc = formSpecToHudItem(&form_spec);
    match assoc.kind() {
        BaseType::Weapon(w) => match w {
            WeaponType::AxeOneHanded(_, _) => BoundType::WarAxe,
            WeaponType::AxeTwoHanded(_, _) => BoundType::BattleAxe,
            WeaponType::BowShort(_, _) => BoundType::Bow,
            WeaponType::Bow(_, _) => BoundType::Bow,
            WeaponType::Crossbow(_, _) => BoundType::Bow,
            WeaponType::Dagger(_, _) => BoundType::Dagger,
            WeaponType::Hammer(_, _) => BoundType::Hammer,
            WeaponType::Mace(_, _) => BoundType::Mace,
            WeaponType::SwordOneHanded(_, _) => BoundType::Sword,
            WeaponType::SwordTwoHanded(_, _) => BoundType::Greatsword,
            _ => BoundType::Unknown,
        },
        _ => BoundType::Unknown,
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
    // Transmute,
    TurnUndead,
    Ward,
    Waterbreathing,
    // Waterwalking,
}

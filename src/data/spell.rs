//! Spells have two traits that we extract from keywords: the icon used to depict
//! them, and the color used to draw the icon. Color is determined by damage type
//! and spell "class" for spells that come from themed mods. The icon is
//! determined by the spell archetype or in-game art. E.e., the wall spells are
//! use wall-like icons. So we're pretty reductive. SpellType is a struct with
//! two enums, one for each trait.

#![allow(non_snake_case, non_camel_case_types)]

use super::color::InvColor;
use super::icons::Icon;
use super::keywords::*;
use super::magic::{MagicColor, School, SpellData};
use super::HasIcon;
use crate::plugin::Color;

#[derive(Default, Clone, Debug, Eq, Hash, PartialEq)]
pub struct SpellType {
    pub icon: Icon,
    pub color: InvColor,
    pub data: SpellData,
}

impl SpellType {
    pub fn new(data: SpellData, tags: Vec<String>) -> Self {
        let keywords = strings_to_keywords(&tags);

        let damage_category = keywords.iter().find_map(|xs| {
            if DAMAGE_FIRE.contains(*xs) {
                Some(MagicColor::Fire)
            } else if DAMAGE_SHOCK.contains(*xs) {
                Some(MagicColor::Shock)
            } else if DAMAGE_FROST.contains(*xs) {
                Some(MagicColor::Frost)
            } else if DAMAGE_POISON.contains(*xs) {
                Some(MagicColor::Poison)
            } else if DAMAGE_SUN.contains(*xs) {
                Some(MagicColor::Sun)
            } else if DAMAGE_ARCANE.contains(*xs) {
                Some(MagicColor::Arcane)
            } else if DAMAGE_ARCANEFIRE.contains(*xs) {
                Some(MagicColor::ArcaneFire)
            } else if DAMAGE_ASHFIRE.contains(*xs) {
                Some(MagicColor::Ashfire)
            } else if DAMAGE_ASTRAL.contains(*xs) {
                Some(MagicColor::Astral)
            } else if DAMAGE_BLOOD.contains(*xs) {
                Some(MagicColor::Bleed)
            } else if DAMAGE_EARTH.contains(*xs) {
                Some(MagicColor::Earth)
            } else if DAMAGE_FROSTFIRE.contains(*xs) {
                Some(MagicColor::FrostFire)
            } else if DAMAGE_LUNAR.contains(*xs) {
                Some(MagicColor::Lunar)
            } else if DAMAGE_NECROTIC.contains(*xs) {
                Some(MagicColor::Necrotic)
            } else if DAMAGE_SHADOW.contains(*xs) {
                Some(MagicColor::Shadow)
            } else if DAMAGE_SHOCKARC.contains(*xs) {
                Some(MagicColor::ShockArc)
            } else if DAMAGE_WATER.contains(*xs) {
                Some(MagicColor::Water)
            } else if DAMAGE_WIND.contains(*xs) {
                Some(MagicColor::Wind)
            } else {
                None
            }
        });
        let damage = damage_category.map_or(data.damage.clone(), |xs| xs);
        let mut color = damage.color(); // we might override this

        let icon = if let Some(v) = keywords.iter().find_map(|xs| {
            if CLOAK_SPELLS.contains(*xs) {
                Some(Icon::ArmorCloak)
            } else if HEALING_SPELLS.contains(*xs) {
                color = InvColor::Green;
                Some(Icon::SpellHeal)
            } else if SUMMON_SPELLS.contains(*xs) {
                Some(Icon::SpellSummon)
            } else if BUFF_SPELLS.contains(*xs) {
                Some(Icon::SpellStamina)
            } else if CONTROL_SPELLS.contains(*xs) {
                Some(Icon::SpellControl)
            } else if COUNTER_SPELLS.contains(*xs) {
                None
            } else if CURSES.contains(*xs) {
                None
            } else if FRENZY_SPELLS.contains(*xs) {
                None
            } else if FEAR_SPELLS.contains(*xs) {
                Some(Icon::SpellFear)
            } else if PARALYZE_SPELLS.contains(*xs) {
                Some(Icon::SpellParalyze)
            } else if VISION_SPELLS.contains(*xs) {
                Some(Icon::SpellEagleEye)
            } else if matches!(xs, SpellEffectKeywords::SpellEthereal) {
                color = InvColor::Silver;
                // Some(Icon::SpellEthereal)
                None
            } else if matches!(xs, SpellEffectKeywords::Archetype_Teleport) {
                Some(Icon::SpellTeleport)
            } else if matches!(xs, SpellEffectKeywords::SpellTime) {
                Some(Icon::SpellTime)
            } else if matches!(xs, SpellEffectKeywords::Archetype_Detect) {
                Some(Icon::SpellDetect)
            } else if matches!(xs, SpellEffectKeywords::Archetype_Waterbreathing) {
                // can I find an icon?
                None
            } else if matches!(xs, SpellEffectKeywords::Archetype_Waterwalking) {
                // can I find an icon?
                None
            } else if matches!(xs, SpellEffectKeywords::Archetype_WeaponBuff) {
                Some(Icon::SpellSharpen)
            } else if matches!(xs, SpellEffectKeywords::Archetype_Guide) {
                Some(Icon::SpellWisp)
            } else if matches!(xs, SpellEffectKeywords::SpellLight) {
                Some(Icon::SpellLight)
            } else if matches!(xs, SpellEffectKeywords::Archetype_CarryWeight) {
                Some(Icon::SpellFeather)
            } else if matches!(xs, SpellEffectKeywords::Archetype_Cure) {
                Some(Icon::SpellCure)
            } else if matches!(xs, SpellEffectKeywords::Archetype_Cure) {
                Some(Icon::SpellCure)
            } else if matches!(xs, SpellEffectKeywords::SpellReanimate) {
                Some(Icon::SpellReanimate)
            } else if matches!(xs, SpellEffectKeywords::Archetype_Reflect) {
                Some(Icon::SpellReflect)
            } else if matches!(xs, SpellEffectKeywords::Archetype_Resist) {
                None
            } else if matches!(xs, SpellEffectKeywords::Archetype_Root) {
                Some(Icon::SpellRoot)
            } else if matches!(xs, SpellEffectKeywords::MagicRune) {
                Some(Icon::SpellRune)
            } else if matches!(xs, SpellEffectKeywords::Archetype_Silence) {
                Some(Icon::SpellSilence)
            } else if matches!(xs, SpellEffectKeywords::SpellSoulTrap) {
                Some(Icon::SpellSoultrap)
            } else if matches!(xs, SpellEffectKeywords::MagicArmorSpell) {
                // defense up
                None
            } else if matches!(xs, SpellEffectKeywords::MagicInvisibility) {
                // Some(Icon::SpellInvisibility)
                None
            } else if matches!(xs, SpellEffectKeywords::MagicSlow) {
                None
            } else if matches!(xs, SpellEffectKeywords::MagicNightEye) {
                Some(Icon::SpellDetect)
            } else if matches!(xs, SpellEffectKeywords::MagicTelekinesis) {
                None
            } else if matches!(xs, SpellEffectKeywords::MagicTurnUndead) {
                Some(Icon::SpellSun)
            } else if matches!(xs, SpellEffectKeywords::MagicWard) {
                Some(Icon::SpellWard)
            } else if matches!(xs, SpellEffectKeywords::MagicWeaponSpeed) {
                Some(Icon::SpellElementalFury)
            } else if matches!(xs, SpellEffectKeywords::MagicSummonFamiliar) {
                Some(Icon::SpellSummon)
            } else if matches!(xs, SpellEffectKeywords::MagicSummonFire) {
                Some(Icon::SpellSummon)
            } else if matches!(xs, SpellEffectKeywords::MagicSummonFrost) {
                Some(Icon::SpellSummon)
            } else if matches!(xs, SpellEffectKeywords::MagicSummonShock) {
                Some(Icon::SpellSummon)
            } else if matches!(xs, SpellEffectKeywords::MagicSummonUndead) {
                Some(Icon::SpellReanimate) // gets the zombie icon
            } else if matches!(xs, SpellEffectKeywords::SpellBound_Weapon) {
                let b = if keywords.contains(&SpellEffectKeywords::BoundBattleAxe) {
                    Icon::WeaponAxeTwoHanded
                } else if keywords.contains(&SpellEffectKeywords::BoundBow) {
                    Icon::WeaponBow
                } else if keywords.contains(&SpellEffectKeywords::BoundDagger) {
                    Icon::WeaponDagger
                } else if keywords.contains(&SpellEffectKeywords::BoundGreatsword) {
                    Icon::WeaponSwordTwoHanded
                } else if keywords.contains(&SpellEffectKeywords::BoundHammer) {
                    Icon::WeaponHammer
                } else if keywords.contains(&SpellEffectKeywords::BoundMace) {
                    Icon::WeaponMace
                } else if keywords.contains(&SpellEffectKeywords::BoundShield) {
                    Icon::ArmorShieldHeavy
                } else if keywords.contains(&SpellEffectKeywords::BoundSword) {
                    Icon::WeaponSwordOneHanded
                } else if keywords.contains(&SpellEffectKeywords::BoundWarAxe) {
                    Icon::WeaponAxeOneHanded
                } else {
                    Icon::WeaponSwordOneHanded
                };
                Some(b)
            } else if matches!(xs, SpellEffectKeywords::SpellBound_Armor) {
                Some(Icon::ArmorShieldHeavy)
            } else if matches!(xs, SpellEffectKeywords::ArtBall) {
                if matches!(data.damage, MagicColor::Fire) {
                    Some(Icon::SpellFireball)
                } else if matches!(data.damage, MagicColor::Shock) {
                    Some(Icon::SpellLightningBall)
                } else {
                    Some(Icon::SpellStormblast)
                }
            } else if matches!(xs, SpellEffectKeywords::ArtBlast) {
                if matches!(data.damage, MagicColor::Fire) {
                    Some(Icon::SpellMeteor)
                } else if matches!(data.damage, MagicColor::Shock) {
                    Some(Icon::SpellLightningBlast)
                } else if matches!(data.damage, MagicColor::Wind | MagicColor::Water) {
                    Some(Icon::SpellStormblast)
                } else {
                    Some(Icon::SpellBlast)
                }
            } else if matches!(xs, SpellEffectKeywords::ArtBolt) {
                if matches!(data.damage, MagicColor::Fire) {
                    Some(Icon::SpellBolt)
                } else if matches!(data.damage, MagicColor::Shock) {
                    Some(Icon::SpellShockStrong)
                } else {
                    None
                }
            } else if matches!(xs, SpellEffectKeywords::ArtBreath) {
                Some(Icon::SpellBreathAttack)
            } else if matches!(xs, SpellEffectKeywords::ArtChainLightning) {
                Some(Icon::SpellChainLightning)
            } else if matches!(xs, SpellEffectKeywords::ArtFlame) {
                Some(Icon::SpellFire)
            } else if matches!(xs, SpellEffectKeywords::ArtLightning) {
                Some(Icon::SpellLightning)
            } else if matches!(xs, SpellEffectKeywords::ArtProjectile) {
                Some(Icon::SpellBolt)
            } else if matches!(xs, SpellEffectKeywords::ArtSpike) {
                match damage {
                    MagicColor::Arcane => todo!(),
                    MagicColor::ArcaneFire => todo!(),
                    MagicColor::Ashfire => todo!(),
                    MagicColor::Astral => todo!(),
                    MagicColor::Bleed => todo!(),
                    MagicColor::Disease => todo!(),
                    MagicColor::Earth => todo!(),
                    MagicColor::Fire => todo!(),
                    MagicColor::Frost => Some(Icon::SpellIceShard),
                    MagicColor::FrostFire => Some(Icon::SpellIceShard),
                    MagicColor::Lunar => todo!(),
                    MagicColor::Magic => todo!(),
                    MagicColor::Necrotic => todo!(),
                    MagicColor::Poison => todo!(),
                    MagicColor::Shadow => Some(Icon::SpellIceShard),
                    MagicColor::Shock => Some(Icon::SpellShockStrong),
                    MagicColor::ShockArc => Some(Icon::SpellShockStrong),
                    _ => None,
                }
            } else if matches!(xs, SpellEffectKeywords::ArtStorm) {
                Some(Icon::SpellStormblast)
            } else if matches!(xs, SpellEffectKeywords::ArtTornado) {
                Some(Icon::SpellTornado)
            } else if matches!(xs, SpellEffectKeywords::ArtWall) {
                if matches!(data.damage, MagicColor::Fire) {
                    Some(Icon::SpellFireWall)
                } else if matches!(data.damage, MagicColor::Frost) {
                    Some(Icon::SpellFrostWall)
                } else if matches!(data.damage, MagicColor::Shock) {
                    Some(Icon::SpellStormblast)
                } else {
                    None
                }
            } else if matches!(xs, SpellEffectKeywords::SpellShapechange_Werebeast) {
                Some(Icon::SpellWerewolf)
            } else if matches!(
                xs,
                SpellEffectKeywords::SpellShapechange_Creature
                    | SpellEffectKeywords::SpellShapechange_Werebeast
            ) {
                Some(Icon::SpellBear)
            } else if matches!(xs, SpellEffectKeywords::SpellShapechange_Vampire) {
                // need vampire icon
                None
            } else {
                None
            }
        }) {
            v
        } else {
            if let Some(icon) = match damage {
                MagicColor::Arcane => Some(Icon::SpellAstral),
                MagicColor::ArcaneFire => Some(Icon::SpellFire),
                MagicColor::Ashfire => Some(Icon::SpellFire),
                MagicColor::Astral => Some(Icon::SpellAstral),
                MagicColor::Bleed => Some(Icon::SpellBleed),
                MagicColor::Earth => Some(Icon::SpellEarth),
                MagicColor::Fire => Some(Icon::SpellFire),
                MagicColor::Frost => Some(Icon::SpellFrost),
                MagicColor::FrostFire => Some(Icon::SpellFire),
                MagicColor::Lunar => Some(Icon::SpellMoon),
                MagicColor::Necrotic => Some(Icon::SpellNecrotic),
                MagicColor::Poison => Some(Icon::SpellPoison),
                MagicColor::Shadow => Some(Icon::SpellShadow),
                MagicColor::Shock => Some(Icon::SpellShock),
                MagicColor::ShockArc => Some(Icon::SpellArclight),
                MagicColor::Sun => Some(Icon::SpellHoly),
                MagicColor::Water => Some(Icon::SpellWater),
                MagicColor::Wind => Some(Icon::SpellWind),
                _ => None,
            } {
                icon
            } else {
                match data.school {
                    School::Alteration => Icon::Alteration,
                    School::Conjuration => Icon::Conjuration,
                    School::Destruction => Icon::Destruction,
                    School::Illusion => Icon::Illusion,
                    School::Restoration => Icon::Restoration,
                    School::None => Icon::IconDefault,
                }
            }
        };

        if matches!(icon, Icon::IconDefault) {
            log::debug!("Falling back to default spell variant; data: {data:?}");
            log::debug!("    keywords: {tags:?}");
        };

        Self { icon, color, data }
    }
}

impl HasIcon for SpellType {
    fn icon_file(&self) -> String {
        self.icon.icon_file()
    }

    fn color(&self) -> Color {
        self.color.color()
    }

    /*
    fn color(&self) -> Color {
        match &self.variant {
            SpellVariant::BoundWeapon(_) => InvColor::Eldritch.color(),
            SpellVariant::Cloak(t) => t.color(),
            SpellVariant::Cure => InvColor::Green.color(),
            SpellVariant::Damage(t) => t.color(),
            SpellVariant::Flame(t) => t.color(),
            SpellVariant::Guide => InvColor::Eldritch.color(),
            SpellVariant::Heal => InvColor::Green.color(),
            SpellVariant::Light => InvColor::Eldritch.color(),
            SpellVariant::Rune(t) => t.color(),
            SpellVariant::SoulTrap => InvColor::Eldritch.color(),
            SpellVariant::Summon(t) => t.color(),
            SpellVariant::TurnUndead => InvColor::Sun.color(),
            SpellVariant::Ward => Color::default(),
            _ => Color::default(),
        }
    }
    */

    fn icon_fallback(&self) -> String {
        Icon::Scroll.icon_file()
    }
}

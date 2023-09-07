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
use super::magic::{MagicCategory, School, SpellData};
use super::HasIcon;
use crate::plugin::Color;

#[derive(Default, Clone, Debug, Eq, Hash, PartialEq)]
pub struct SpellType {
    icon: Icon,
    color: InvColor,
    data: SpellData,
}

impl SpellType {
    pub fn new(data: SpellData, tags: Vec<String>) -> Self {
        let keywords = strings_to_keywords(&tags);
        // log::info!("{keywords:?}");

        let mut damage_category = keywords.iter().find_map(|xs| {
            if DAMAGE_ARCANE.contains(*xs) {
                Some(MagicCategory::Arcane)
            } else if DAMAGE_ARCANEFIRE.contains(*xs) {
                Some(MagicCategory::ArcaneFire)
            } else if DAMAGE_ASHFIRE.contains(*xs) {
                Some(MagicCategory::Ashfire)
            } else if DAMAGE_ASTRAL.contains(*xs) {
                Some(MagicCategory::Astral)
            } else if DAMAGE_BLOOD.contains(*xs) {
                Some(MagicCategory::Bleed)
            } else if DAMAGE_EARTH.contains(*xs) {
                Some(MagicCategory::Earth)
            } else if DAMAGE_FROSTFIRE.contains(*xs) {
                Some(MagicCategory::FrostFire)
            } else if DAMAGE_LUNAR.contains(*xs) {
                Some(MagicCategory::Lunar)
            } else if DAMAGE_NECROTIC.contains(*xs) {
                Some(MagicCategory::Necrotic)
            } else if DAMAGE_SHADOW.contains(*xs) {
                Some(MagicCategory::Shadow)
            } else if DAMAGE_SHOCKARC.contains(*xs) {
                Some(MagicCategory::ShockArc)
            } else if DAMAGE_WATER.contains(*xs) {
                Some(MagicCategory::Water)
            } else if DAMAGE_WIND.contains(*xs) {
                Some(MagicCategory::Wind)
            } else if DAMAGE_POISON.contains(*xs) {
                Some(MagicCategory::Poison)
            } else if DAMAGE_SUN.contains(*xs) {
                Some(MagicCategory::Sun)
            } else {
                None
            }
        });
        // Fall back to vanilla damage types.
        if damage_category.is_none() {
            damage_category = keywords.iter().find_map(|xs| {
                if DAMAGE_FIRE.contains(*xs) {
                    Some(MagicCategory::Fire)
                } else if DAMAGE_FROST.contains(*xs) {
                    Some(MagicCategory::Frost)
                } else if DAMAGE_SHOCK.contains(*xs) {
                    Some(MagicCategory::Shock)
                } else {
                    None
                }
            });
        }
        let damage = damage_category.map_or(data.damage.clone(), |xs| xs);
        let mut color = damage.color(); // we might override this

        log::info!("magic category: {damage:?}");

        let art_hint = keywords.iter().find_map(|xs| {
            if matches!(xs, SpellEffectKeywords::ArtBall) {
                if matches!(data.damage, MagicCategory::Fire) {
                    Some(Icon::SpellFireball)
                } else if matches!(data.damage, MagicCategory::Shock) {
                    Some(Icon::SpellLightningBall)
                } else {
                    Some(Icon::SpellStormblast)
                }
            } else if matches!(xs, SpellEffectKeywords::ArtBlast) {
                if matches!(data.damage, MagicCategory::Fire) {
                    Some(Icon::SpellMeteor)
                } else if matches!(data.damage, MagicCategory::Shock) {
                    Some(Icon::SpellLightningBlast)
                } else if matches!(data.damage, MagicCategory::Wind | MagicCategory::Water) {
                    Some(Icon::SpellStormblast)
                } else {
                    Some(Icon::SpellBlast)
                }
            } else if matches!(xs, SpellEffectKeywords::ArtBolt) {
                if matches!(data.damage, MagicCategory::Fire) {
                    Some(Icon::SpellBolt)
                } else if matches!(data.damage, MagicCategory::Shock) {
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
                    MagicCategory::Frost => Some(Icon::SpellIceShard),
                    MagicCategory::FrostFire => Some(Icon::SpellIceShard),
                    MagicCategory::Shadow => Some(Icon::SpellIceShard),
                    MagicCategory::Shock => Some(Icon::SpellShockStrong),
                    MagicCategory::ShockArc => Some(Icon::SpellShockStrong),
                    _ => None,
                }
            } else if matches!(xs, SpellEffectKeywords::ArtStorm) {
                Some(Icon::SpellStormblast)
            } else if matches!(xs, SpellEffectKeywords::ArtTornado) {
                Some(Icon::SpellTornado)
            } else if matches!(xs, SpellEffectKeywords::ArtWall) {
                if matches!(data.damage, MagicCategory::Fire) {
                    Some(Icon::SpellFireWall)
                } else if matches!(data.damage, MagicCategory::Frost) {
                    Some(Icon::SpellFrostWall)
                } else if matches!(data.damage, MagicCategory::Shock) {
                    Some(Icon::SpellStormblast)
                } else {
                    None
                }
            } else {
                None
            }
        });
        // og::info!("art hint: {art_hint:?}");

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
            // } else if COUNTER_SPELLS.contains(*xs) {
            //     None
            // } else if CURSES.contains(*xs) {
            //     None
            // } else if FRENZY_SPELLS.contains(*xs) {
            //     None
            } else if FEAR_SPELLS.contains(*xs) {
                Some(Icon::SpellFear)
            } else if PARALYZE_SPELLS.contains(*xs) {
                Some(Icon::SpellParalyze)
            } else if VISION_SPELLS.contains(*xs) {
                Some(Icon::SpellEagleEye)
            } else {
                match *xs {
                    SpellEffectKeywords::SpellEthereal => {
                        color = InvColor::Silver;
                        None
                    }
                    SpellEffectKeywords::Archetype_Teleport => Some(Icon::SpellTeleport),
                    SpellEffectKeywords::SpellTime => Some(Icon::SpellTime),
                    SpellEffectKeywords::Archetype_Detect => Some(Icon::SpellDetect),
                    SpellEffectKeywords::Archetype_WeaponBuff => Some(Icon::SpellSharpen),
                    SpellEffectKeywords::Archetype_Guide => {
                        color = InvColor::Eldritch;
                        Some(Icon::SpellWisp)
                    }
                    SpellEffectKeywords::SpellLight => {
                        color = InvColor::Eldritch;
                        Some(Icon::SpellLight)
                    }
                    SpellEffectKeywords::Archetype_Light => {
                        color = InvColor::Eldritch;
                        Some(Icon::SpellLight)
                    }
                    SpellEffectKeywords::Archetype_CarryWeight => Some(Icon::SpellFeather),
                    SpellEffectKeywords::Archetype_Cure => {
                        color = InvColor::Green;
                        Some(Icon::SpellCure)
                    }
                    SpellEffectKeywords::SpellReanimate => Some(Icon::SpellReanimate),
                    SpellEffectKeywords::Archetype_Reflect => Some(Icon::SpellReflect),
                    SpellEffectKeywords::Archetype_Root => {
                        color = InvColor::Green;
                        Some(Icon::SpellRoot)
                    }
                    SpellEffectKeywords::MagicRune => Some(Icon::SpellRune),
                    SpellEffectKeywords::Archetype_Silence => Some(Icon::SpellSilence),
                    SpellEffectKeywords::SpellSoulTrap => {
                        color = InvColor::Eldritch;
                        Some(Icon::SpellSoultrap)
                    }
                    SpellEffectKeywords::MagicSlow => Some(Icon::SpellSlow),
                    SpellEffectKeywords::MagicNightEye => Some(Icon::SpellDetect),
                    SpellEffectKeywords::MagicTelekinesis => None,
                    SpellEffectKeywords::MagicTurnUndead => {
                        color = InvColor::Sun;
                        Some(Icon::SpellSun)
                    }
                    SpellEffectKeywords::MagicWard => Some(Icon::SpellWard),
                    SpellEffectKeywords::MagicWeaponSpeed => Some(Icon::SpellElementalFury),
                    SpellEffectKeywords::MagicSummonFamiliar => Some(Icon::SpellSummon),
                    SpellEffectKeywords::MagicSummonFire => {
                        color = InvColor::Fire;
                        Some(Icon::SpellSummon)
                    }
                    SpellEffectKeywords::MagicSummonFrost => {
                        color = InvColor::Frost;
                        Some(Icon::SpellSummon)
                    }
                    SpellEffectKeywords::MagicSummonShock => {
                        color = InvColor::Shock;
                        Some(Icon::SpellSummon)
                    }
                    SpellEffectKeywords::MagicSummonUndead => Some(Icon::SpellReanimate),
                    SpellEffectKeywords::SpellBound_Weapon => {
                        color = InvColor::Eldritch;
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
                    }
                    SpellEffectKeywords::SpellBound_Armor => {
                        color = InvColor::Eldritch;
                        Some(Icon::ArmorShieldHeavy)
                    }
                    SpellEffectKeywords::SpellShapechange_Werebeast => Some(Icon::SpellWerewolf),
                    SpellEffectKeywords::SpellShapechange_Creature => Some(Icon::SpellBear),
                    SpellEffectKeywords::SpellShapechange => Some(Icon::SpellBear),
                    // SpellEffectKeywords::Archetype_Waterbreathing => None,
                    // SpellEffectKeywords::Archetype_Waterwalking => None,
                    // SpellEffectKeywords::Archetype_Resist => None,
                    // SpellEffectKeywords::MagicArmorSpell => None,
                    // SpellEffectKeywords::MagicInvisibility => Some(Icon::SpellInvisibility),
                    _ => None,
                }
            }
        }) {
            v
        } else if let Some(icon) = art_hint {
            icon
        } else if let Some(icon) = match damage {
            MagicCategory::Arcane => Some(Icon::SpellAstral),
            MagicCategory::ArcaneFire => Some(Icon::SpellFire),
            MagicCategory::Ashfire => Some(Icon::SpellFire),
            MagicCategory::Astral => Some(Icon::SpellAstral),
            MagicCategory::Bleed => Some(Icon::SpellBleed),
            MagicCategory::Earth => Some(Icon::SpellEarth),
            MagicCategory::Fire => Some(Icon::SpellFire),
            MagicCategory::Frost => Some(Icon::SpellFrost),
            MagicCategory::FrostFire => Some(Icon::SpellFire),
            MagicCategory::Lunar => Some(Icon::SpellMoon),
            MagicCategory::Necrotic => Some(Icon::SpellNecrotic),
            MagicCategory::Poison => Some(Icon::SpellPoison),
            MagicCategory::Shadow => Some(Icon::SpellShadow),
            MagicCategory::Shock => Some(Icon::SpellShock),
            MagicCategory::ShockArc => Some(Icon::SpellArclight),
            MagicCategory::Sun => Some(Icon::SpellHoly),
            MagicCategory::Water => Some(Icon::SpellWater),
            MagicCategory::Wind => Some(Icon::SpellWind),
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
        };

        if matches!(icon, Icon::IconDefault) {
            log::debug!("Falling back to default spell variant; data: {data:?}");
            log::debug!("    keywords: {tags:?}");
        };

        Self { icon, color, data }
    }

    pub fn two_handed(&self) -> bool {
        self.data.twohanded
    }
}

impl HasIcon for SpellType {
    fn icon_file(&self) -> String {
        self.icon.icon_file()
    }

    fn color(&self) -> Color {
        self.color.color()
    }

    fn icon_fallback(&self) -> String {
        Icon::Scroll.icon_file()
    }
}

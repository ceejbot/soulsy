//! Classify spells and assign icons and colors.
//!
//! Spells have two traits that we extract from keywords: the icon used to depict
//! them, and the color used to draw the icon. Color is determined by damage type
//! and spell "class" for spells that come from themed mods. We use keywords for this
//! as much as possible. Spell packs use single icons where this makes sense.
//!
//! We keep the boiled-down spell data around to handle fallbacks like using the magic school
//! if other icons aren't available. 90% of the classification work is done in `keywords.rs`,
//! which creates enum sets to match keywords against.

#![allow(non_snake_case, non_camel_case_types)]

use enumset::EnumSet;

use super::color::InvColor;
use super::icons::Icon;
use super::keywords::*;
use super::magic::{School, SpellData};
use super::{strings_to_keywords, HasIcon};
use crate::plugin::Color;

#[derive(Default, Clone, Debug, Eq, Hash, PartialEq)]
pub struct SpellType {
    icon: Icon,
    color: InvColor,
    data: SpellData,
}

impl SpellType {
    pub fn new(data: SpellData, tags: Vec<String>) -> Self {
        let keywords = strings_to_keywords::<SpellKeywords>(&tags);
        let mut itemkwds: EnumSet<SpellKeywords> = EnumSet::new();
        keywords.iter().for_each(|xs| {
            itemkwds.insert(*xs);
        });

        // Icons. We look to see if the keywords contain any of the words that
        // match certain known icon art sets. If we have a specific icon for
        // a spell type, e.g. cloak spells, we use that. We then try to use an
        // icon for a mod spell pack, e.g., constellation. If all else fails,
        // we use the icon for the magic school.
        let icon = if !itemkwds.is_disjoint(CLOAK_SPELLS) {
            Icon::ArmorCloak
        // next pre-classified spells
        } else if !itemkwds.is_disjoint(USE_FIRE_ICON) {
            Icon::SpellFire
        } else if !itemkwds.is_disjoint(SUMMON_SPELLS) {
            Icon::SpellSummon
        } else if !itemkwds.is_disjoint(BUFF_SPELLS) {
            Icon::SpellStamina
        } else if !itemkwds.is_disjoint(CONTROL_SPELLS) {
            Icon::SpellControl
        } else if !itemkwds.is_disjoint(FEAR_SPELLS) {
            Icon::SpellFear
        } else if !itemkwds.is_disjoint(PARALYZE_SPELLS) {
            Icon::SpellParalyze
        } else if !itemkwds.is_disjoint(VISION_SPELLS) {
            Icon::SpellEagleEye
        } else if !itemkwds.is_disjoint(LIGHT_SPELLS) {
            Icon::SpellLight
        // bound weapons
        } else if itemkwds.contains(SpellKeywords::SpellBound_Weapon) {
            if keywords.contains(&SpellKeywords::BoundBattleAxe) {
                Icon::WeaponAxeTwoHanded
            } else if keywords.contains(&SpellKeywords::BoundBow) {
                Icon::WeaponBow
            } else if keywords.contains(&SpellKeywords::BoundDagger) {
                Icon::WeaponDagger
            } else if keywords.contains(&SpellKeywords::BoundGreatsword) {
                Icon::WeaponSwordTwoHanded
            } else if keywords.contains(&SpellKeywords::BoundHammer) {
                Icon::WeaponHammer
            } else if keywords.contains(&SpellKeywords::BoundMace) {
                Icon::WeaponMace
            } else if keywords.contains(&SpellKeywords::BoundShield) {
                Icon::ArmorShieldHeavy
            } else if keywords.contains(&SpellKeywords::BoundSword) {
                Icon::WeaponSwordOneHanded
            } else if keywords.contains(&SpellKeywords::BoundWarAxe) {
                Icon::WeaponAxeOneHanded
            } else {
                Icon::WeaponSwordOneHanded
            }
        } else if itemkwds.contains(SpellKeywords::SpellBound_Armor) {
            Icon::ArmorShieldHeavy
        } else if !itemkwds.is_disjoint(HEALING_SPELLS) {
            Icon::SpellHeal
        } else if !itemkwds.is_disjoint(STORM_SPELLS) {
            Icon::SpellLightningBlast
        } else if !itemkwds.is_disjoint(VAMPIRE_SPELLS) {
            Icon::SpellVampire
        // next icon packs
        } else if !itemkwds.is_disjoint(DARENII_DESECRATION) {
            Icon::SpellDesecration
        } else if !itemkwds.is_disjoint(DARENII_STELLARIS) {
            Icon::SpellStars
        } else if !itemkwds.is_disjoint(DARENII_LUNARIS) {
            Icon::SpellMoon
        } else if !itemkwds.is_disjoint(CONSTELLATION_SPELLS) {
            Icon::SpellConstellation
        // next one-off vanilla spells
        } else if itemkwds.contains(SpellKeywords::Archetype_Teleport) {
            Icon::SpellTeleport
        } else if itemkwds.contains(SpellKeywords::SpellTime) {
            Icon::SpellTime
        } else if itemkwds.contains(SpellKeywords::Archetype_Detect) {
            Icon::SpellDetect
        } else if itemkwds.contains(SpellKeywords::Archetype_WeaponBuff) {
            Icon::SpellSharpen
        } else if itemkwds.contains(SpellKeywords::Archetype_Guide) {
            Icon::SpellWisp
        } else if itemkwds.contains(SpellKeywords::Archetype_CarryWeight) {
            Icon::SpellFeather
        } else if itemkwds.contains(SpellKeywords::Archetype_Cure) {
            Icon::SpellCure
        } else if itemkwds.contains(SpellKeywords::SpellReanimate) {
            Icon::SpellReanimate
        } else if itemkwds.contains(SpellKeywords::Archetype_Reflect) {
            Icon::SpellReflect
        } else if itemkwds.contains(SpellKeywords::Archetype_Root) {
            Icon::SpellRoot
        } else if itemkwds.contains(SpellKeywords::MagicRune) {
            Icon::SpellRune
        } else if itemkwds.contains(SpellKeywords::Archetype_Silence) {
            Icon::SpellSilence
        } else if itemkwds.contains(SpellKeywords::SpellSoulTrap) {
            Icon::SpellSoultrap
        } else if itemkwds.contains(SpellKeywords::MagicSlow) {
            Icon::SpellSlow
        } else if itemkwds.contains(SpellKeywords::MagicNightEye) {
            Icon::SpellDetect
        } else if itemkwds.contains(SpellKeywords::MagicTurnUndead) {
            Icon::SpellSun
        } else if itemkwds.contains(SpellKeywords::MagicWard) {
            Icon::SpellWard
        } else if itemkwds.contains(SpellKeywords::MagicWeaponSpeed) {
            Icon::SpellElementalFury
        } else if itemkwds.contains(SpellKeywords::MagicSummonFamiliar) {
            Icon::SpellSummon
        } else if itemkwds.contains(SpellKeywords::MagicSummonUndead) {
            Icon::SpellReanimate
        } else if itemkwds.contains(SpellKeywords::SpellShapechange_Werebeast)
            || itemkwds.contains(SpellKeywords::SpellShapechange)
        {
            Icon::PowerWerewolf
        } else {
            match data.school {
                School::Alteration => Icon::Alteration,
                School::Conjuration => Icon::Conjuration,
                School::Destruction => Icon::Destruction,
                School::Illusion => Icon::Illusion,
                School::Restoration => Icon::Restoration,
                School::None => {
                    log::debug!("Fell back to default spell variant; data: {data:?}");
                    log::debug!("    keywords: {tags:?}");
                    Icon::IconDefault
                }
            }
        };

        // Colors. We base this on damage type, mostly, but first we look to see
        // if we have a color keyword.
        let color_kwds = strings_to_keywords::<InvColor>(&tags);
        let color = if let Some(assigned) = color_kwds.first() {
            assigned.clone()
        } else if !itemkwds.is_disjoint(COLOR_ASH) {
            InvColor::Ash
        } else if !itemkwds.is_disjoint(COLOR_BLOOD) {
            InvColor::Blood
        } else if !itemkwds.is_disjoint(COLOR_EARTH) {
            InvColor::Brown
        } else if !itemkwds.is_disjoint(COLOR_ELDRITCH) {
            InvColor::Eldritch
        } else if !itemkwds.is_disjoint(COLOR_FIRE) {
            InvColor::Fire
        } else if !itemkwds.is_disjoint(COLOR_FROST) {
            InvColor::Frost
        } else if !itemkwds.is_disjoint(HEALING_SPELLS) {
            InvColor::Green
        } else if !itemkwds.is_disjoint(COLOR_HOLY) {
            InvColor::Holy
        } else if !itemkwds.is_disjoint(COLOR_POISON) {
            InvColor::Poison
        } else if !itemkwds.is_disjoint(COLOR_SHADOW) {
            InvColor::Shadow
        } else if !itemkwds.is_disjoint(COLOR_SHOCK) {
            InvColor::Shock
        } else if !itemkwds.is_disjoint(COLOR_SUN) {
            InvColor::Sun
        } else if !itemkwds.is_disjoint(COLOR_WATER) {
            InvColor::Water
        } else if !itemkwds.is_disjoint(COLOR_WIND) {
            InvColor::Gray
        } else {
            match data.school {
                // TODO identify common colors for magical schools
                School::Alteration => InvColor::Eldritch,
                School::Conjuration => InvColor::Silver,
                School::Destruction => InvColor::Fire,
                School::Illusion => InvColor::Blue,
                School::Restoration => InvColor::Green,
                School::None => InvColor::default(),
            }
        };

        Self { icon, color, data }
    }

    pub fn two_handed(&self) -> bool {
        self.data.twohanded
    }

    pub fn icon_fallback(&self) -> Icon {
        match self.data.school {
            School::Alteration => Icon::Alteration,
            School::Conjuration => Icon::Conjuration,
            School::Destruction => Icon::Destruction,
            School::Illusion => Icon::Illusion,
            School::Restoration => Icon::Restoration,
            School::None => self.icon.fallback(),
        }
    }
}

impl HasIcon for SpellType {
    fn icon(&self) -> &Icon {
        &self.icon
    }

    fn color(&self) -> Color {
        self.color.color()
    }
}

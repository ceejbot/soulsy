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

use enumset::EnumSet;

use super::color::InvColor;
use super::keywords::*;
use super::magic::{School, SpellData};
use super::{strings_to_enumset, strings_to_keywords, HasIcon};
use crate::images::icons::Icon;
use crate::plugin::Color;

#[derive(Default, Clone, Debug, Eq, Hash, PartialEq)]
pub struct SpellType {
    icon: Icon,
    color: InvColor,
    data: SpellData,
}

impl SpellType {
    pub fn new(data: SpellData, tags: Vec<String>) -> Self {
        let tagset: EnumSet<SpellKeywords> = strings_to_enumset(&tags);

        // Icons. We look to see if the keywords contain any of the words that
        // match certain known icon art sets. If we have a specific icon for
        // a spell type, e.g. cloak spells, we use that. We then try to use an
        // icon for a mod spell pack, e.g., constellation. If all else fails,
        // we use the icon for the magic school.
        let icon = if !tagset.is_disjoint(ICON_CLOAK) {
            Icon::ArmorCloak
        } else if !tagset.is_disjoint(ICON_BUFF) {
            Icon::SpellStamina
        } else if !tagset.is_disjoint(ICON_CONTROL) {
            Icon::SpellControl
        } else if !tagset.is_disjoint(ICON_FEAR) {
            Icon::SpellFear
        } else if !tagset.is_disjoint(ICON_LIGHT) {
            Icon::SpellLight
        } else if !tagset.is_disjoint(ICON_SUMMON) {
            Icon::SpellSummon
        } else if !tagset.is_disjoint(ICON_PARALYZE) {
            Icon::SpellParalyze
        } else if !tagset.is_disjoint(ICON_VISION) {
            Icon::SpellEagleEye
            // bound weapons
        } else if tagset.contains(SpellKeywords::SpellBound_Weapon) {
            if tagset.contains(SpellKeywords::BoundBattleAxe) {
                Icon::WeaponAxeTwoHanded
            } else if tagset.contains(SpellKeywords::BoundBow) {
                Icon::WeaponBow
            } else if tagset.contains(SpellKeywords::BoundDagger) {
                Icon::WeaponDagger
            } else if tagset.contains(SpellKeywords::BoundGreatsword) {
                Icon::WeaponSwordTwoHanded
            } else if tagset.contains(SpellKeywords::BoundHammer) {
                Icon::WeaponHammer
            } else if tagset.contains(SpellKeywords::BoundMace) {
                Icon::WeaponMace
            } else if tagset.contains(SpellKeywords::BoundShield) {
                Icon::ArmorShieldHeavy
            } else if tagset.contains(SpellKeywords::BoundSword) {
                Icon::WeaponSwordOneHanded
            } else if tagset.contains(SpellKeywords::BoundWarAxe) {
                Icon::WeaponAxeOneHanded
            } else {
                Icon::WeaponSwordOneHanded
            }
        } else if tagset.contains(SpellKeywords::SpellBound_Armor) {
            Icon::ArmorShieldHeavy
        } else if !tagset.is_disjoint(ICON_HEALING) {
            Icon::SpellHeal
        } else if !tagset.is_disjoint(ICON_EARTH) {
            Icon::SpellEarth
        } else if !tagset.is_disjoint(ICON_STORM) {
            Icon::SpellLightningBlast
        } else if !tagset.is_disjoint(ICON_VAMPIRE) {
            Icon::SpellVampire
        } else if !tagset.is_disjoint(ICON_DRUID) {
            Icon::SpellLeaves
        } else if !tagset.is_disjoint(ICON_ROOT) {
            Icon::SpellRoot
        } else if !tagset.is_disjoint(ICON_CIRCLE) {
            Icon::SpellCircle
        } else if !tagset.is_disjoint(ICON_HOLY) {
            Icon::SpellSun
        // next one-off vanilla spells
        } else if tagset.contains(SpellKeywords::Archetype_Teleport) {
            Icon::SpellTeleport
        } else if tagset.contains(SpellKeywords::SpellTime) {
            Icon::SpellTime
        } else if tagset.contains(SpellKeywords::Archetype_Detect) {
            Icon::SpellDetect
        } else if tagset.contains(SpellKeywords::Archetype_WeaponBuff) {
            Icon::SpellSharpen
        } else if tagset.contains(SpellKeywords::Archetype_Guide) {
            Icon::SpellWisp
        } else if tagset.contains(SpellKeywords::Archetype_CarryWeight) {
            Icon::SpellFeather
        } else if tagset.contains(SpellKeywords::Archetype_Cure) {
            Icon::SpellCure
        } else if tagset.contains(SpellKeywords::SpellReanimate) {
            Icon::SpellReanimate
        } else if tagset.contains(SpellKeywords::Archetype_Reflect) {
            Icon::SpellReflect
        } else if tagset.contains(SpellKeywords::MagicRune) {
            Icon::SpellRune
        } else if tagset.contains(SpellKeywords::Archetype_Silence) {
            Icon::SpellSilence
        } else if tagset.contains(SpellKeywords::SpellSoulTrap) {
            Icon::SpellSoultrap
        } else if tagset.contains(SpellKeywords::MagicSlow) {
            Icon::SpellSlow
        } else if tagset.contains(SpellKeywords::MagicNightEye) {
            Icon::SpellDetect
        } else if tagset.contains(SpellKeywords::MagicTurnUndead) {
            Icon::SpellSun
        } else if tagset.contains(SpellKeywords::MagicWard) {
            Icon::SpellWard
        } else if tagset.contains(SpellKeywords::MagicWeaponSpeed) {
            Icon::ShoutElementalFury
        } else if tagset.contains(SpellKeywords::MagicSummonFamiliar) {
            Icon::SpellSummon
        } else if tagset.contains(SpellKeywords::MagicSummonUndead) {
            Icon::SpellReanimate
        } else if tagset.contains(SpellKeywords::SpellShapechange_Werebeast)
            || tagset.contains(SpellKeywords::SpellShapechange)
        {
            Icon::PowerWerewolf
            // next icon packs
        } else if !tagset.is_disjoint(DARENII_ARCLIGHT) {
            Icon::SpellArclight
        } else if !tagset.is_disjoint(DARENII_DESECRATION) {
            Icon::SpellDesecration
        } else if !tagset.is_disjoint(DARENII_STELLARIS) {
            Icon::SpellStars
        } else if !tagset.is_disjoint(DARENII_LUNARIS) {
            Icon::SpellMoon
        } else if !tagset.is_disjoint(CONSTELLATION_SPELLS) {
            Icon::SpellConstellation
        // now really generic damage spells
        } else if !tagset.is_disjoint(ICON_FIRE) {
            Icon::SpellFire
        } else if !tagset.is_disjoint(ICON_SHOCK) {
            Icon::SpellShock
        } else if !tagset.is_disjoint(ICON_FROST) {
            Icon::SpellFrost
        } else {
            log::debug!("Falling back to magic school for spell; data: {data:?}");
            log::debug!("    keywords: {tags:?}");
            match data.school {
                School::Alteration => Icon::Alteration,
                School::Conjuration => Icon::Conjuration,
                School::Destruction => Icon::Destruction,
                School::Illusion => Icon::Illusion,
                School::Restoration => Icon::Restoration,
                School::None => Icon::IconDefault,
            }
        };

        // Colors. We base this on damage type, mostly, but first we look to see
        // if we have a color keyword.
        let color_kwds = strings_to_keywords::<InvColor>(&tags);
        let color = if let Some(assigned) = color_kwds.first() {
            assigned.clone()
        } else if !tagset.is_disjoint(DARENII_ARCLIGHT) {
            InvColor::ShockArc
        } else if !tagset.is_disjoint(COLOR_ASH) {
            InvColor::Ash
        } else if !tagset.is_disjoint(COLOR_BLOOD) {
            InvColor::Blood
        } else if !tagset.is_disjoint(COLOR_BOUND_ITEMS) {
            InvColor::Bound
        } else if !tagset.is_disjoint(COLOR_EARTH) {
            InvColor::Brown
        } else if !tagset.is_disjoint(COLOR_ELDRITCH) {
            InvColor::Eldritch
        } else if !tagset.is_disjoint(COLOR_HOLY) {
            InvColor::Holy
        } else if !tagset.is_disjoint(DARENII_LUNARIS) {
            InvColor::Lunar
        } else if !tagset.is_disjoint(COLOR_NECROTIC) {
            InvColor::Necrotic
        } else if !tagset.is_disjoint(COLOR_POISON) {
            InvColor::Poison
        } else if !tagset.is_disjoint(COLOR_SHADOW) {
            InvColor::Shadow
        } else if !tagset.is_disjoint(COLOR_SUN) {
            InvColor::Sun
        } else if !tagset.is_disjoint(COLOR_WATER) {
            InvColor::Water
        } else if !tagset.is_disjoint(COLOR_WIND) {
            InvColor::Gray
        } else if !tagset.is_disjoint(ICON_HEALING) {
            InvColor::Green
        } else if !tagset.is_disjoint(COLOR_FIRE) {
            InvColor::Fire
        } else if !tagset.is_disjoint(COLOR_FROST) {
            InvColor::Frost
        } else if !tagset.is_disjoint(COLOR_SHOCK) {
            InvColor::Shock
        } else {
            match data.school {
                // TODO identify common colors for magical schools
                School::Alteration => InvColor::Eldritch,
                School::Conjuration => InvColor::Silver,
                School::Destruction => InvColor::Fire,
                School::Illusion => InvColor::Blue,
                School::Restoration => InvColor::Green,
                School::None => {
                    log::debug!("no color specified for spell; keywords={tags:?};");
                    InvColor::default()
                }
            }
        };

        Self { icon, color, data }
    }

    pub fn is_two_handed(&self) -> bool {
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

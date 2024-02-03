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

use std::fmt::Display;

use enumset::EnumSet;

use super::color::{color_from_keywords, InvColor};
use super::keywords::*;
use super::magic::{School, SpellData};
use super::{strings_to_enumset, HasIcon};
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
        let icon = if let Some(icon) = icon_for_tagset(&tagset) {
            icon
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
        let color = if let Some(c) = color_from_keywords(&tags) {
            c
        } else if let Some(c) = color_for_tagset(&tagset) {
            c
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

impl Display for SpellType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Spell: icon='{}'; color='{}'; data: {}",
            self.icon, self.color, self.data
        )
    }
}

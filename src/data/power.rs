use super::color::InvColor;
use super::icons::Icon;
use super::HasIcon;
use super::{keywords::*, strings_to_keywords};
use crate::plugin::Color;

#[derive(Default, Clone, Debug, Eq, Hash, PartialEq)]
pub struct PowerType {
    icon: Icon,
    color: InvColor,
}

impl PowerType {
    pub fn new(name: &str, tags: Vec<String>) -> Self {
        let kywds = strings_to_keywords::<SpellKeywords>(&tags);
        let icon = if kywds.contains(&SpellKeywords::SpellShapechange_Werebeast) {
            Icon::PowerWerewolf
        } else if kywds.contains(&SpellKeywords::SpellShapechange) {
            Icon::PowerWerebear
        } else if kywds.contains(&SpellKeywords::PowerAction_Campfire) {
            Icon::MiscCampfire
        } else if kywds.contains(&SpellKeywords::PowerAction_PitchTent) {
            Icon::MiscTent
        } else if kywds.contains(&SpellKeywords::PowerAction_Bathe) {
            Icon::PowerWash
        } else if kywds.contains(&SpellKeywords::PowerAction_Horse) {
            Icon::PowerHorse
        } else if kywds.contains(&SpellKeywords::PowerAction_Pray) {
            Icon::PowerPray
        } else if kywds.contains(&SpellKeywords::PowerAction_FillWater) {
            Icon::PowerFillBottles
        } else if kywds.contains(&SpellKeywords::PowerAction_TameAnimal) {
            Icon::ShoutAnimalAllegiance
            // } else if kywds.contains(&SpellKeywords::PowerAction_WeaponGrip) {
            // Icon::WeaponGrip
        } else {
            log::debug!("Falling back to default icon for power; name='{name}'; keywords={tags:?}");
            Icon::Power
        };

        PowerType {
            icon,
            color: InvColor::default(),
        }
    }
}

impl HasIcon for PowerType {
    fn color(&self) -> Color {
        self.color.color()
    }

    fn icon(&self) -> &Icon {
        &self.icon
    }
}

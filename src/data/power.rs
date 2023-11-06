use super::color::InvColor;
use super::icons::Icon;
use super::keywords::*;
use super::{strings_to_enumset, HasIcon};
use crate::plugin::Color;

#[derive(Default, Clone, Debug, Eq, Hash, PartialEq)]
pub struct PowerType {
    icon: Icon,
    color: InvColor,
}

impl PowerType {
    pub fn new(name: &str, tags: Vec<String>) -> Self {
        let kywds = strings_to_enumset::<SpellKeywords>(&tags);

        let icon = if kywds.contains(SpellKeywords::SpellShapechange_Werebeast) {
            Icon::PowerWerewolf
        } else if kywds.contains(SpellKeywords::SpellShapechange) {
            Icon::PowerWerebear
        } else if kywds.contains(SpellKeywords::PowerAction_Bag) {
            Icon::ArmorBackpack
        } else if kywds.contains(SpellKeywords::PowerAction_Bard) {
            Icon::MiscLute
        } else if kywds.contains(SpellKeywords::PowerAction_Bathe) {
            // I have no joke here; I just like saying power wash.
            Icon::PowerWash
        } else if kywds.contains(SpellKeywords::PowerAction_Bless) {
            Icon::ArmorBackpack // TODO bless icon
        } else if kywds.contains(SpellKeywords::PowerAction_BuryCorpse) {
            Icon::ToolShovel
        } else if kywds.contains(SpellKeywords::PowerAction_Campfire) {
            Icon::MiscCampfire
        // } else if kywds.contains(SpellKeywords::PowerAction_Coin) {
        // Icon::MiscCoin
        // } else if kywds.contains(SpellKeywords::PowerAction_CommandFollower) {
        // Icon::ArmorBackpack // TODO command icon
        // } else if kywds.contains(SpellKeywords::PowerAction_Craft) {
        // Icon::ArmorBackpack // TODO craft icon
        } else if kywds.contains(SpellKeywords::PowerAction_FillWater) {
            Icon::PowerFillBottles
        } else if kywds.contains(SpellKeywords::PowerAction_HarvestCorpse) {
            Icon::ToolShovel // TODO wrong!
        } else if kywds.contains(SpellKeywords::PowerAction_HarvestGather) {
            Icon::ToolSickle
        } else if kywds.contains(SpellKeywords::PowerAction_HarvestWood) {
            Icon::WeaponWoodAxe
        } else if kywds.contains(SpellKeywords::PowerAction_Horse) {
            Icon::PowerHorse
        } else if kywds.contains(SpellKeywords::PowerAction_Lantern) {
            Icon::MiscLantern
        } else if kywds.contains(SpellKeywords::PowerAction_PitchTent) {
            Icon::MiscTent
        } else if kywds.contains(SpellKeywords::PowerAction_PeekKeyhole) {
            Icon::PowerPeek
        } else if kywds.contains(SpellKeywords::PowerAction_Potion) {
            Icon::PotionDefault
        } else if kywds.contains(SpellKeywords::PowerAction_Pray) {
            Icon::PowerPray
        } else if kywds.contains(SpellKeywords::PowerAction_Relax) {
            Icon::PowerPeek
        // } else if kywds.contains(SpellKeywords::PowerAction_Speech) {
        //     Icon::PowerPeek
        // } else if kywds.contains(SpellKeywords::PowerAction_StatusFrostfall) {
        //     Icon::PowerPeek
        // } else if kywds.contains(SpellKeywords::PowerAction_StatusSunhelm) {
        //     Icon::PowerPeek
        } else if kywds.contains(SpellKeywords::PowerAction_TameAnimal) {
            Icon::ShoutAnimalAllegiance
        // } else if kywds.contains(SpellKeywords::PowerAction_Train) {
        //     Icon::PowerPeek
        // } else if kywds.contains(SpellKeywords::PowerAction_WeaponGrip) {
        //     Icon::WeaponGrip
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

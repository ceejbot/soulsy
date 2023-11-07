//! Food and drink. These items are alchemy items in the game, but they
//! get their own icons.

use enumset::{enum_set, EnumSet, EnumSetType};
use strum::EnumString;

use super::color::InvColor;
use super::{strings_to_enumset, HasIcon, HasKeywords};
use crate::images::icons::Icon;
use crate::plugin::Color;

/// Struct to hold the icon selection and the inventory color to use.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Default)]
pub struct FoodType {
    icon: Icon,
    color: InvColor,
}

/// Quack quack icon trait.
impl HasIcon for FoodType {
    fn color(&self) -> Color {
        self.color.color()
    }

    fn icon(&self) -> &Icon {
        &self.icon
    }
}

/// We select color and icon from keywords, so we implement this trait.
impl HasKeywords for FoodType {
    fn classify(name: &str, keywords: Vec<String>, _twohanded: bool) -> Self {
        let color = super::color::color_from_keywords(&keywords);
        let tags = strings_to_enumset::<FoodKeywords>(&keywords);
        let containers = strings_to_enumset::<ContainerKeywords>(&keywords);

        let icon = if !ICON_TEA.is_disjoint(tags) {
            Icon::DrinkTea
        } else if !ICON_WATER.is_disjoint(tags) {
            Icon::DrinkWater
        } else if !ICON_WINE.is_disjoint(tags) {
            Icon::DrinkWine
        } else if !ICON_BREAD.is_disjoint(tags) {
            Icon::FoodBread
        } else if !ICON_CARROT.is_disjoint(tags) {
            Icon::FoodCarrot
        } else if !ICON_CHEESE.is_disjoint(tags) {
            Icon::FoodCheese
        } else if !ICON_FISH.is_disjoint(tags) {
            Icon::FoodFish
        } else if !ICON_MEAT.is_disjoint(tags) {
            Icon::FoodMeat
        } else if !ICON_PIE.is_disjoint(tags) {
            Icon::FoodPie
        } else if !ICON_STEW.is_disjoint(tags) {
            Icon::FoodStew
        } else if !ICON_TEACUP.is_disjoint(containers) {
            Icon::DrinkTea
        } else if !ICON_WINE_BOTTLE.is_disjoint(containers) {
            Icon::DrinkWine
        } else if !ICON_MEAD.is_disjoint(containers) {
            Icon::DrinkMead
        } else if !ICON_SKOOMA.is_disjoint(containers) {
            Icon::PotionSkooma
        } else if !ICON_WATER_JUG.is_disjoint(containers) {
            Icon::DrinkWater
        } else if !ICON_STEW_BOWL.is_disjoint(containers) {
            Icon::FoodStew
        } else {
            log::debug!("Falling back to default food icon: name='{name}'; keywords={keywords:?}");
            Icon::Food
        };
        // ContainerKeywords::OCF_VesselBottlePotion => Icon::PotionDefault,

        Self { icon, color }
    }
}

const ICON_WINE_BOTTLE: EnumSet<ContainerKeywords> =
    enum_set!(ContainerKeywords::_SH_WineBottleKeyword);
const ICON_MEAD: EnumSet<ContainerKeywords> = enum_set!(
    ContainerKeywords::_SH_MeadBottleKeyword
        | ContainerKeywords::OCF_VesselBottle
        | ContainerKeywords::OCF_VesselFlagon
        | ContainerKeywords::OCF_VesselTankard
);
const ICON_SKOOMA: EnumSet<ContainerKeywords> =
    enum_set!(ContainerKeywords::OCF_VesselBottleSkooma | ContainerKeywords::OCF_VesselVial);
const ICON_STEW_BOWL: EnumSet<ContainerKeywords> = enum_set!(ContainerKeywords::OCF_VesselBowl);
const ICON_WATER_JUG: EnumSet<ContainerKeywords> = enum_set!(
    ContainerKeywords::OCF_VesselWaterskin
        | ContainerKeywords::OCF_VesselFlask
        | ContainerKeywords::OCF_VesselJug
);
const ICON_TEACUP: EnumSet<ContainerKeywords> = enum_set!(ContainerKeywords::OCF_VesselCup);

const ICON_TEA: EnumSet<FoodKeywords> = enum_set!(
    FoodKeywords::OCF_AlchDrink_Coffee // heresy!
        | FoodKeywords::OCF_AlchDrink_Tea
);

const ICON_WATER: EnumSet<FoodKeywords> = enum_set!(
    FoodKeywords::OCF_AlchDrink_Juice
        | FoodKeywords::OCF_AlchDrink_Milk
        | FoodKeywords::OCF_AlchDrink_Water
        | FoodKeywords::OCF_AlchDrink_MilkRaw
        | FoodKeywords::OCF_AlchDrink_WaterRaw
);

const ICON_WINE: EnumSet<FoodKeywords> = enum_set!(FoodKeywords::MAG_FoodTypeWine);

const ICON_BREAD: EnumSet<FoodKeywords> = enum_set!(FoodKeywords::OCF_AlchFood_Bread);

const ICON_CARROT: EnumSet<FoodKeywords> = enum_set!(FoodKeywords::OCF_AlchFood_Vegetable);

const ICON_CHEESE: EnumSet<FoodKeywords> = enum_set!(FoodKeywords::OCF_AlchFood_Cheese);

const ICON_FISH: EnumSet<FoodKeywords> = enum_set!(
    FoodKeywords::OCF_AlchFood_Fish
        | FoodKeywords::OCF_AlchFood_FishRaw
        | FoodKeywords::OCF_AlchFood_Seafood
        | FoodKeywords::OCF_AlchFood_SeafoodRaw
);

const ICON_MEAT: EnumSet<FoodKeywords> = enum_set!(
    FoodKeywords::OCF_AlchFood_Meat
        | FoodKeywords::OCF_AlchFood_MeatSmall
        | FoodKeywords::OCF_AlchFood_MeatRaw
);

const ICON_PIE: EnumSet<FoodKeywords> = enum_set!(
    FoodKeywords::OCF_AlchFood_Meal
        | FoodKeywords::OCF_AlchFood_Treat
        | FoodKeywords::MAG_FoodTypePie
        | FoodKeywords:OCF_AlchFood_Baked
);

const ICON_STEW: EnumSet<FoodKeywords> =
    enum_set!(FoodKeywords::OCF_AlchFood_Stew | FoodKeywords::OCF_AlchFood_Treat);

#[derive(Debug, EnumString, Hash, EnumSetType)]
enum ContainerKeywords {
    OCF_VesselBottle,
    OCF_VesselBottlePotion,
    OCF_VesselBottleSkooma,
    OCF_VesselBowl,
    OCF_VesselCup,
    OCF_VesselFlagon,
    OCF_VesselFlask,
    OCF_VesselJug,
    // OCF_VesselSack,
    OCF_VesselTankard,
    OCF_VesselVial,
    OCF_VesselWaterskin,
    _SH_MeadBottleKeyword,
    _SH_WineBottleKeyword,
}

#[derive(Debug, EnumString, Hash, EnumSetType)]
enum FoodKeywords {
    OCF_AlchDrink_Coffee,
    OCF_AlchDrink_Juice,
    OCF_AlchDrink_Milk,
    OCF_AlchDrink_MilkRaw,
    OCF_AlchDrink_Tea,
    OCF_AlchDrink_Water,
    OCF_AlchDrink_WaterRaw,
    OCF_AlchDrink,
    OCF_AlchDrinkAlcohol,
    OCF_AlchDrinkSoft,
    OCF_AlchFood_Baked,
    OCF_AlchFood_Bread,
    OCF_AlchFood_Cheese,
    OCF_AlchFood_Egg,
    OCF_AlchFood_EggMagic,
    OCF_AlchFood_EggRaw,
    OCF_AlchFood_Fish,
    OCF_AlchFood_FishRaw,
    OCF_AlchFood_Fruit,
    OCF_AlchFood_Ingredient,
    OCF_AlchFood_IngredientDry,
    OCF_AlchFood_IngredientRaw,
    OCF_AlchFood_IngredientWet,
    OCF_AlchFood_Meal,
    OCF_AlchFood_Meat,
    OCF_AlchFood_MeatRaw,
    OCF_AlchFood_MeatSmall,
    OCF_AlchFood_Seafood,
    OCF_AlchFood_SeafoodRaw,
    OCF_AlchFood_Stew,
    OCF_AlchFood_Treat,
    OCF_AlchFood_Vegetable,
    OCF_AlchFood,
    OCF_AlchGreenPact,
    MAG_FoodTypePie,
    MAG_FoodTypeWine,
}

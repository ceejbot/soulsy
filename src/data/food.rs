//! Food and drink. These items are alchemy items in the game, but they
//! get their own icons.

use strum::EnumString;

use super::color::InvColor;
use super::icons::Icon;
use super::{HasIcon, HasKeywords, strings_to_keywords};
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
    fn classify(_name: &str, keywords: Vec<String>, _twohanded: bool) -> Self {
        let color = super::base::color_from_keywords(&keywords);
        let tags = strings_to_keywords::<FoodKeywords>(&keywords);
        let containers = strings_to_keywords::<ContainerKeywords>(&keywords);

        let maybe_icon = tags.iter().find_map(|subtype| {
            match subtype {
                FoodKeywords::OCF_AlchDrink_Coffee => Some(Icon::DrinkTea), // heresy
                FoodKeywords::OCF_AlchDrink_Juice => Some(Icon::DrinkWater),
                FoodKeywords::OCF_AlchDrink_Milk => Some(Icon::DrinkWater),
                FoodKeywords::OCF_AlchDrink_Tea => Some(Icon::DrinkTea),
                FoodKeywords::OCF_AlchDrink_Water => Some(Icon::DrinkWater),
                FoodKeywords::OCF_AlchDrinkAlcohol => pickContainerIcon(&containers),
                FoodKeywords::OCF_AlchFood_Baked => Some(Icon::FoodPie),
                FoodKeywords::OCF_AlchFood_Bread => Some(Icon::FoodBread),
                FoodKeywords::OCF_AlchFood_Cheese => Some(Icon::FoodCheese),
                FoodKeywords::OCF_AlchFood_Fish => Some(Icon::FoodFish),
                FoodKeywords::OCF_AlchFood_Fruit => Some(Icon::Food),
                FoodKeywords::OCF_AlchFood_Meal => Some(Icon::FoodPie),
                FoodKeywords::OCF_AlchFood_Meat => Some(Icon::FoodMeat),
                FoodKeywords::OCF_AlchFood_Seafood => Some(Icon::FoodFish),
                FoodKeywords::OCF_AlchFood_Stew => Some(Icon::FoodStew),
                FoodKeywords::OCF_AlchFood_Treat => Some(Icon::FoodPie),
                FoodKeywords::OCF_AlchFood_Vegetable => Some(Icon::FoodCarrot),
                FoodKeywords::MAG_FoodTypePie => Some(Icon::FoodPie),
                FoodKeywords::MAG_FoodTypeWine => Some(Icon::DrinkWine),
            }
        });

        let icon = if let Some(icon) = maybe_icon {
            icon
        } else {
            Icon::Food
        };

        Self { icon, color }
    }
}

/// A helper function to use the container type for many drink items.
fn pickContainerIcon(containers: &[ContainerKeywords]) -> Option<Icon> {
    containers
        .iter()
        .map(|xs| match xs {
            ContainerKeywords::_SH_WineBottleKeyword => Icon::DrinkWine,
            ContainerKeywords::_SH_MeadBottleKeyword => Icon::DrinkMead,
            ContainerKeywords::OCF_VesselBottle => Icon::DrinkMead,
            ContainerKeywords::OCF_VesselBottlePotion => Icon::PotionDefault,
            ContainerKeywords::OCF_VesselBottleSkooma => Icon::PotionSkooma,
            ContainerKeywords::OCF_VesselBowl => Icon::FoodStew,
            ContainerKeywords::OCF_VesselCup => Icon::DrinkTea,
            ContainerKeywords::OCF_VesselFlagon => Icon::DrinkMead,
            ContainerKeywords::OCF_VesselFlask => Icon::DrinkWater,
            ContainerKeywords::OCF_VesselJug => Icon::DrinkWater,
            ContainerKeywords::OCF_VesselTankard => Icon::DrinkMead,
            ContainerKeywords::OCF_VesselVial => Icon::PotionSkooma,
            ContainerKeywords::OCF_VesselWaterskin => Icon::DrinkWater,
        })
        .next()
}

#[derive(Debug, EnumString, Hash)]
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

#[derive(Debug, EnumString, Hash)]
enum FoodKeywords {
    OCF_AlchDrink_Coffee,
    OCF_AlchDrink_Juice,
    OCF_AlchDrink_Milk,
    OCF_AlchDrink_Tea,
    OCF_AlchDrink_Water,
    OCF_AlchDrinkAlcohol,
    OCF_AlchFood_Baked,
    OCF_AlchFood_Bread,
    OCF_AlchFood_Cheese,
    OCF_AlchFood_Fish,
    OCF_AlchFood_Fruit,
    OCF_AlchFood_Meal,
    OCF_AlchFood_Meat,
    OCF_AlchFood_Seafood,
    OCF_AlchFood_Stew,
    OCF_AlchFood_Treat,
    OCF_AlchFood_Vegetable,
    MAG_FoodTypePie,
    MAG_FoodTypeWine,
}

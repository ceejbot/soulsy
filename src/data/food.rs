use strum::EnumString;

use super::color::InvColor;
use super::icons::Icon;
use super::{HasIcon, HasKeywords};
use crate::plugin::Color;

/// Food variations that get their own icons.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct FoodType {
    icon: Icon,
    color: InvColor,
}

impl Default for FoodType {
    fn default() -> Self {
        Self {
            icon: Icon::default(),
            color: InvColor::default(),
        }
    }
}

/// Food variations that get their own icons.

impl HasIcon for FoodType {
    fn color(&self) -> Color {
        self.color.color()
    }

    fn icon_file(&self) -> String {
        self.icon.icon_file()
    }

    fn icon_fallback(&self) -> String {
        Icon::Food.icon_file()
    }
}

impl HasKeywords for FoodType {
    fn classify(_name: &str, keywords: Vec<String>, _twohanded: bool) -> Self {
        log::info!("{keywords:?}");
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
                FoodKeywords::OCF_AlchFood_Baked => Some(Icon::FoodBread),
                FoodKeywords::OCF_AlchFood_Bread => Some(Icon::FoodBread),
                FoodKeywords::OCF_AlchFood_Cheese => Some(Icon::FoodCheese),
                FoodKeywords::OCF_AlchFood_Fish => Some(Icon::FoodFish),
                FoodKeywords::OCF_AlchFood_Fruit => Some(Icon::Food),
                FoodKeywords::OCF_AlchFood_Meal => Some(Icon::Food),
                FoodKeywords::OCF_AlchFood_Meat => Some(Icon::FoodMeat),
                FoodKeywords::OCF_AlchFood_Seafood => Some(Icon::FoodFish),
                FoodKeywords::OCF_AlchFood_Stew => Some(Icon::FoodStew),
                FoodKeywords::OCF_AlchFood_Treat => Some(Icon::FoodBread),
                FoodKeywords::OCF_AlchFood_Vegetable => Some(Icon::FoodCarrot),
                _ => None,
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

fn pickContainerIcon(containers: &[ContainerKeywords]) -> Option<Icon> {
    containers.iter().find_map(|xs| match xs {
        ContainerKeywords::_SH_WineBottleKeyword => Some(Icon::DrinkWine),
        ContainerKeywords::_SH_MeadBottleKeyword => Some(Icon::DrinkMead),
        ContainerKeywords::OCF_VesselBottle => Some(Icon::DrinkMead),
        ContainerKeywords::OCF_VesselBottlePotion => Some(Icon::PotionDefault),
        ContainerKeywords::OCF_VesselBottleSkooma => Some(Icon::PotionSkooma),
        ContainerKeywords::OCF_VesselBowl => Some(Icon::FoodStew),
        ContainerKeywords::OCF_VesselCup => Some(Icon::DrinkTea),
        ContainerKeywords::OCF_VesselFlagon => todo!(),
        ContainerKeywords::OCF_VesselFlask => todo!(),
        ContainerKeywords::OCF_VesselJug => Some(Icon::DrinkWater),
        ContainerKeywords::OCF_VesselTankard => todo!(),
        ContainerKeywords::OCF_VesselVial => todo!(),
        ContainerKeywords::OCF_VesselWaterskin => Some(Icon::DrinkWater),
    })
}

fn strings_to_keywords<T: for<'a> TryFrom<&'a str>>(tags: &[String]) -> Vec<T> {
    let keywords: Vec<T> = tags
        .iter()
        .filter_map(|xs| {
            if let Ok(subtype) = T::try_from(xs.as_str()) {
                Some(subtype)
            } else {
                None
            }
        })
        .collect();
    keywords
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

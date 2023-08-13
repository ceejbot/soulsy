//! OCF color keywords associated with specific colors.

use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

use crate::plugin::Color;

impl Color {
    pub fn rgb(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b, a: 255 }
    }
}

#[derive(
    Decode,
    Encode,
    Deserialize,
    Serialize,
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    Display,
    EnumString,
)]
pub enum InvColor {
    OCF_InvColorAedric,
    OCF_InvColorAsh,
    OCF_InvColorBlack,
    OCF_InvColorBlood,
    OCF_InvColorBlue,
    OCF_InvColorBrown,
    OCF_InvColorCopper,
    OCF_InvColorDaedric,
    OCF_InvColorDwarven,
    OCF_InvColorEldritch,
    OCF_InvColorFire,
    OCF_InvColorFrost,
    OCF_InvColorGold,
    OCF_InvColorGray,
    OCF_InvColorGreen,
    OCF_InvColorLegendary,
    OCF_InvColorOrange,
    OCF_InvColorPink,
    OCF_InvColorPoison,
    OCF_InvColorPurple,
    OCF_InvColorRed,
    OCF_InvColorShock,
    OCF_InvColorSilver,
    OCF_InvColorSun,
    OCF_InvColorWater,
    #[default]
    OCF_InvColorWhite,
    OCF_InvColorYellow,
}

impl InvColor {
    pub fn color(&self) -> Color {
        match *self {
            InvColor::OCF_InvColorAedric => Color::default(), // TODO
            InvColor::OCF_InvColorAsh => Color::rgb(64, 64, 64),
            InvColor::OCF_InvColorBlack => Color::rgb(0, 0, 0),
            InvColor::OCF_InvColorBlue => Color::rgb(59, 106, 249),
            InvColor::OCF_InvColorBlood => Color::rgb(138, 3, 3),
            InvColor::OCF_InvColorBrown => Color::rgb(165, 42, 42),
            InvColor::OCF_InvColorCopper => Color::rgb(184, 115, 51),
            InvColor::OCF_InvColorDaedric => Color::rgb(171, 35, 0),
            InvColor::OCF_InvColorDwarven => Color::rgb(255, 175, 0),
            InvColor::OCF_InvColorEldritch => Color::rgb(230, 230, 250),
            InvColor::OCF_InvColorFire => Color::rgb(255, 76, 0), // orange
            InvColor::OCF_InvColorFrost => Color::rgb(0, 237, 255),
            InvColor::OCF_InvColorGold => Color::rgb(218, 165, 32),
            InvColor::OCF_InvColorGray => Color::rgb(128, 128, 128),
            InvColor::OCF_InvColorGreen => Color::rgb(32, 223, 32),
            InvColor::OCF_InvColorLegendary => Color::rgb(255, 175, 0),
            InvColor::OCF_InvColorOrange => Color::rgb(255, 76, 0),
            InvColor::OCF_InvColorPink => Color::rgb(219, 46, 114),
            InvColor::OCF_InvColorPoison => Color::rgb(192, 128, 255), // purple
            InvColor::OCF_InvColorPurple => Color::rgb(192, 128, 255),
            InvColor::OCF_InvColorRed => Color::rgb(255, 0, 0),
            InvColor::OCF_InvColorShock => Color::rgb(255, 213, 0), // yellow
            InvColor::OCF_InvColorSilver => Color::rgb(192, 192, 192),
            InvColor::OCF_InvColorSun => Color::rgb(223, 188, 32),
            InvColor::OCF_InvColorWater => Color::rgb(212, 241, 249),
            InvColor::OCF_InvColorWhite => Color::default(),
            InvColor::OCF_InvColorYellow => Color::rgb(255, 213, 0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_classify_from_keywords() {}
}

//! OCF color keywords associated with specific colors.

use strum::{Display, EnumIter, IntoEnumIterator};

use crate::plugin::Color;

impl Color {
    pub fn rgb(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b, a: 255 }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, Display, EnumIter)]
#[strum(serialize_all = "lowercase")]
pub enum InvColor {
    Aedric,
    Ash,
    Black,
    Blood,
    Blue,
    Brown,
    Copper,
    Daedric,
    Dwarven,
    Eldritch,
    Fire,
    Frost,
    Gold,
    Gray,
    Green,
    Legendary,
    Orange,
    Pink,
    Poison,
    Purple,
    Red,
    Shock,
    Silver,
    Sun,
    Water,
    #[default]
    White,
    Yellow,
}

impl TryFrom<&str> for InvColor {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let color_name = value.replace("OCF_InvColor", "").to_lowercase();
        let color = InvColor::iter().find(|xs| color_name == xs.to_string());
        if let Some(c) = color {
            Ok(c)
        } else {
            Err(anyhow::anyhow!("not a valid color type"))
        }
    }
}

impl InvColor {
    pub fn color(&self) -> Color {
        match *self {
            InvColor::Aedric => Color::default(), // TODO
            InvColor::Ash => Color::rgb(64, 64, 64),
            InvColor::Black => Color::rgb(0, 0, 0),
            InvColor::Blue => Color::rgb(59, 106, 249),
            InvColor::Blood => Color::rgb(138, 3, 3),
            InvColor::Brown => Color::rgb(165, 42, 42),
            InvColor::Copper => Color::rgb(184, 115, 51),
            InvColor::Daedric => Color::rgb(171, 35, 0),
            InvColor::Dwarven => Color::rgb(255, 175, 0),
            InvColor::Eldritch => Color::rgb(230, 230, 250),
            InvColor::Fire => Color::rgb(255, 76, 0), // orange
            InvColor::Frost => Color::rgb(0, 237, 255),
            InvColor::Gold => Color::rgb(218, 165, 32),
            InvColor::Gray => Color::rgb(128, 128, 128),
            InvColor::Green => Color::rgb(32, 223, 32),
            InvColor::Legendary => Color::rgb(255, 175, 0),
            InvColor::Orange => Color::rgb(255, 76, 0),
            InvColor::Pink => Color::rgb(219, 46, 114),
            InvColor::Poison => Color::rgb(192, 128, 255), // purple
            InvColor::Purple => Color::rgb(192, 128, 255),
            InvColor::Red => Color::rgb(255, 0, 0),
            InvColor::Shock => Color::rgb(255, 213, 0), // yellow
            InvColor::Silver => Color::rgb(192, 192, 192),
            InvColor::Sun => Color::rgb(223, 188, 32),
            InvColor::Water => Color::rgb(212, 241, 249),
            InvColor::White => Color::default(),
            InvColor::Yellow => Color::rgb(255, 213, 0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_classify_from_keywords() {
        let color = InvColor::try_from("OCF_InvColorAedric").expect("aedric is a valid color");
        assert_eq!(color, InvColor::Aedric);
        let color = InvColor::try_from("OCF_InvColorWater").expect("aedric is a valid color");
        assert_eq!(color, InvColor::Water);
        let color = InvColor::try_from("OCF_InvColorShock").expect("aedric is a valid color");
        assert_eq!(color, InvColor::Shock);
        let color = InvColor::try_from("OCF_InvColorSun").expect("aedric is a valid color");
        assert_eq!(color, InvColor::Sun);
        let color = InvColor::try_from("OCF_InvColorDaedric").expect("aedric is a valid color");
        assert_eq!(color, InvColor::Daedric);
    }
}

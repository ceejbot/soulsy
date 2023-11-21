//! OCF color keywords associated with specific colors.

use eyre::{eyre, Result};
use strum::{Display, EnumIter, EnumVariantNames, IntoEnumIterator};

use crate::plugin::Color;

impl Color {
    pub fn rgb(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b, a: 255 }
    }
}

impl Default for Color {
    fn default() -> Self {
        Self {
            r: 255,
            g: 255,
            b: 255,
            a: 255,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, Display, EnumIter, EnumVariantNames)]
#[strum(serialize_all = "lowercase")]
pub enum InvColor {
    Aedric,
    Ash,
    Black,
    Blood,
    Blue,
    Bound,
    Brown,
    Copper,
    Daedric,
    Druid,
    Dwarven,
    Eldritch,
    Fire,
    FireVolcanic,
    Frost,
    Gold,
    Gray,
    Green,
    Holy,
    Legendary,
    Lunar,
    Magenta,
    Necrotic,
    Orange,
    Pink,
    Poison,
    Purple,
    Red,
    Shadow,
    Shock,
    ShockArc,
    Silver,
    Sun,
    Water,
    #[default]
    White,
    Yellow,
}

pub fn color_from_keywords(keywords: &[String]) -> InvColor {
    let color_keywords: Vec<InvColor> = keywords
        .iter()
        .filter_map(|xs| InvColor::try_from(xs.as_str()).ok())
        .collect();
    if let Some(c) = color_keywords.first() {
        c.clone()
    } else {
        InvColor::default()
    }
}

impl TryFrom<&str> for InvColor {
    type Error = eyre::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let color_name = value
            .replace("OCF_InvColor", "")
            .replace("OCF_IconColor", "")
            .to_lowercase();
        let color = InvColor::iter().find(|xs| color_name == xs.to_string());
        if let Some(c) = color {
            Ok(c)
        } else {
            Err(eyre!("not a valid color type"))
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
            InvColor::Bound => Color::rgb(150, 200, 244),
            InvColor::Brown => Color::rgb(165, 42, 42),
            InvColor::Copper => Color::rgb(184, 115, 51),
            InvColor::Daedric => Color::rgb(171, 35, 0),
            InvColor::Druid => Color::rgb(118, 160, 113), // olive green
            InvColor::Dwarven => Color::rgb(255, 175, 0),
            InvColor::Eldritch => Color::rgb(150, 244, 200),
            InvColor::Fire => Color::rgb(255, 40, 0), // red-orange
            InvColor::FireVolcanic => Color::rgb(255, 40, 0), // same as fire; TODO
            InvColor::Frost => Color::rgb(0, 237, 255),
            InvColor::Gold => Color::rgb(218, 165, 32),
            InvColor::Gray => Color::rgb(128, 128, 128),
            InvColor::Green => Color::rgb(32, 223, 32),
            InvColor::Holy => Color::rgb(223, 188, 32), // same as sun for now
            InvColor::Legendary => Color::rgb(255, 175, 0),
            InvColor::Lunar => Color::rgb(130, 185, 230), // light blue
            InvColor::Magenta => Color::rgb(255, 0, 255),
            InvColor::Necrotic => Color::rgb(46, 252, 183), // blue-green
            InvColor::Orange => Color::rgb(255, 76, 0),
            InvColor::Pink => Color::rgb(219, 46, 114),
            InvColor::Poison => Color::rgb(160, 240, 2), // yellow-green
            InvColor::Purple => Color::rgb(192, 128, 255),
            InvColor::Red => Color::rgb(255, 0, 0),
            InvColor::Shadow => Color::rgb(80, 0, 145), // dark purple
            InvColor::Shock => Color::rgb(255, 213, 0), // yellow
            InvColor::ShockArc => Color::rgb(255, 76, 0),
            InvColor::Silver => Color::rgb(192, 192, 192), // light gray
            InvColor::Sun => Color::rgb(223, 188, 32),
            InvColor::Water => Color::rgb(152, 233, 255), // light blue
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

#[cfg(test)]
pub fn random_color() -> InvColor {
    use rand::prelude::*;
    use strum::VariantNames;
    if let Some(variant) = InvColor::VARIANTS.choose(&mut rand::thread_rng()) {
        InvColor::try_from(*variant).unwrap_or(InvColor::Aedric)
    } else {
        InvColor::Shock
    }
}

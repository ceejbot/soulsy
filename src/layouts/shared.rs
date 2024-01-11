//! Types shared by all layout variations, including implementations on
//! types defined in lib.rs.

use std::fmt::Display;

use serde::de::{Deserializer, Error};
use serde::{Deserialize, Serialize};

use crate::plugin::{Action, Align, HudElement, MeterKind};

// ---------- Align

impl Default for Align {
    fn default() -> Self {
        Align::Left
    }
}

impl Display for Align {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Align::Left => write!(f, "left"),
            Align::Right => write!(f, "right"),
            Align::Center => write!(f, "center"),
            _ => write!(f, "left"),
        }
    }
}

impl Serialize for Align {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

pub fn deserialize_align<'de, D>(deserializer: D) -> Result<Align, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;

    match s.to_lowercase().as_str() {
        "left" => Ok(Align::Left),
        "right" => Ok(Align::Right),
        "center" => Ok(Align::Center),
        _ => Err(Error::unknown_variant(&s, &["left", "right", "center"])),
    }
}

// ---------- NamedAnchor

/// Named HUD anchor points.
#[derive(Debug, Default, Clone, Hash, PartialEq)]
pub enum NamedAnchor {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Center,
    CenterTop,    // top edge
    CenterBottom, // bottom edge
    LeftCenter,   // left edge midway down
    RightCenter,  // right edge
    #[default]
    None,
}

impl Display for NamedAnchor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            NamedAnchor::TopLeft => write!(f, "top_left"),
            NamedAnchor::TopRight => write!(f, "top_right"),
            NamedAnchor::BottomLeft => write!(f, "bottom_left"),
            NamedAnchor::BottomRight => write!(f, "bottom_right"),
            NamedAnchor::Center => write!(f, "center"),
            NamedAnchor::CenterTop => write!(f, "center_top"),
            NamedAnchor::CenterBottom => write!(f, "center_bottom"),
            NamedAnchor::LeftCenter => write!(f, "left_center"),
            NamedAnchor::RightCenter => write!(f, "right_center"),
            _ => write!(f, "none"),
        }
    }
}

impl Serialize for NamedAnchor {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

pub fn deserialize_named_anchor<'de, D>(deserializer: D) -> Result<NamedAnchor, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;

    match s.to_lowercase().as_str() {
        "top_left" => Ok(NamedAnchor::TopLeft),
        "top_right" => Ok(NamedAnchor::TopRight),
        "bottom_left" => Ok(NamedAnchor::BottomLeft),
        "bottom_right" => Ok(NamedAnchor::BottomRight),
        "center" => Ok(NamedAnchor::Center),
        "center_top" => Ok(NamedAnchor::CenterTop),
        "top_center" => Ok(NamedAnchor::CenterTop),
        "center_bottom" => Ok(NamedAnchor::CenterBottom),
        "bottom_center" => Ok(NamedAnchor::CenterBottom),
        "left_center" => Ok(NamedAnchor::LeftCenter),
        "center_left" => Ok(NamedAnchor::LeftCenter),
        "right_center" => Ok(NamedAnchor::RightCenter),
        "center_right" => Ok(NamedAnchor::RightCenter),
        "none" => Ok(NamedAnchor::None),
        _ => Err(Error::unknown_variant(
            &s,
            &[
                "top_left",
                "top_right",
                "bottom_left",
                "bottom_right",
                "center",
                "center_top",
                "center_bottom",
                "left_center",
                "right_center",
            ],
        )),
    }
}

impl From<&str> for NamedAnchor {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "top_left" => NamedAnchor::TopLeft,
            "top_right" => NamedAnchor::TopRight,
            "bottom_left" => NamedAnchor::BottomLeft,
            "bottom_right" => NamedAnchor::BottomRight,
            "center" => NamedAnchor::Center,
            "center_top" => NamedAnchor::CenterTop,
            "top_center" => NamedAnchor::CenterTop,
            "center_bottom" => NamedAnchor::CenterBottom,
            "bottom_center" => NamedAnchor::CenterBottom,
            "left_center" => NamedAnchor::LeftCenter,
            "center_left" => NamedAnchor::LeftCenter,
            "right_center" => NamedAnchor::RightCenter,
            "center_right" => NamedAnchor::RightCenter,
            "none" => NamedAnchor::None,
            _ => NamedAnchor::None,
        }
    }
}

// ---------- MeterType

// We can't derive this because it is exposed to C++.
impl Default for MeterKind {
    fn default() -> Self {
        MeterKind::None
    }
}

impl Display for MeterKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            MeterKind::CircleArc => write!(f, "circle_arc"),
            MeterKind::Rectangular => write!(f, "rectangular"),
            _ => write!(f, "none"),
        }
    }
}

impl From<&str> for MeterKind {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "circle_arc" => MeterKind::CircleArc,
            "rectangular" => MeterKind::Rectangular,
            _ => MeterKind::None,
        }
    }
}

impl Serialize for MeterKind {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

pub fn deserialize_meter_kind<'de, D>(deserializer: D) -> Result<MeterKind, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    match s.to_lowercase().as_str() {
        "circle_arc" => Ok(MeterKind::CircleArc),
        "rectangular" => Ok(MeterKind::Rectangular),
        "none" => Ok(MeterKind::None),
        _ => Err(Error::unknown_variant(
            &s,
            &["circle_arc", "rectangular", "none"],
        )),
    }
}

// ---------- HudElement

/// All this converting makes me suspect the abstraction is wrong.
impl From<Action> for HudElement {
    fn from(value: Action) -> Self {
        if value == Action::Power {
            HudElement::Power
        } else if value == Action::Utility {
            HudElement::Utility
        } else if value == Action::Left {
            HudElement::Left
        } else if value == Action::Right {
            HudElement::Right
        } else {
            HudElement::Ammo
        }
    }
}

impl Display for HudElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            HudElement::Ammo => write!(f, "Ammo"),
            HudElement::EquipSet => write!(f, "Equipset"),
            HudElement::Left => write!(f, "Left"),
            HudElement::Power => write!(f, "Power"),
            HudElement::Right => write!(f, "Right"),
            HudElement::Utility => write!(f, "Utility"),
            _ => write!(f, "unknown"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layouts::HudLayout1;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    struct TestAnchor {
        #[serde(default, deserialize_with = "deserialize_named_anchor")]
        anchor: NamedAnchor,
    }

    #[test]
    fn deserde_anchor_names() {
        let input = r#"anchor = "center""#;
        let parsed: TestAnchor = toml::from_str(input).expect("this should be parseable");
        assert_eq!(parsed.anchor, NamedAnchor::Center);

        let input = r#"anchor = "bottom_center""#;
        let parsed: TestAnchor = toml::from_str(input).expect("this should be parseable");
        assert_eq!(parsed.anchor, NamedAnchor::CenterBottom);
    }

    #[test]
    fn parses_named_anchors() {
        let data = std::fs::read_to_string("layouts/hexagons/SoulsyHUD_hexagons_lr.toml")
            .expect("file not found?");
        let hexa1: HudLayout1 = toml::from_str(data.as_str()).expect("layout should be valid toml");
        assert_eq!(hexa1.anchor_name, NamedAnchor::TopRight);
        assert_eq!(hexa1.anchor_point().x, 3290.0);
        assert_eq!(hexa1.anchor_point().y, 150.0);

        let data = std::fs::read_to_string("layouts/hexagons/SoulsyHUD_hexagons_tb.toml")
            .expect("file not found?");
        let hexa2: HudLayout1 = toml::from_str(data.as_str()).expect("layout should be valid toml");
        assert_eq!(hexa2.anchor_name, NamedAnchor::BottomRight);
        assert_eq!(hexa2.anchor_point().x, 3290.0);
        assert_eq!(hexa2.anchor_point().y, 1290.0);
    }
}

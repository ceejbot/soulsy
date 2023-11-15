use std::fmt::Display;

use serde::de::{Deserializer, Error};
use serde::{Deserialize, Serialize};

use crate::plugin::{Action, Align, HudElement};

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
    use crate::layouts::HudLayout1;

    use super::*;

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
    fn parses_anchor_points() {
        let data = include_str!("../../layouts/SoulsyHUD_topleft.toml");
        let layout: HudLayout1 =
        let layout: HudLayout = toml::from_str(data).expect("layout should be valid toml");
        assert_eq!(layout.anchor_name, NamedAnchor::None);
        assert_eq!(layout.anchor_point().x, 150.0);
        assert_eq!(layout.anchor_point().y, 150.0);
    }

    #[cfg(not(target_os = "linux"))]
    #[test]
    fn default_layout_parses() {
        let data = include_str!("../../data/SKSE/plugins/SoulsyHUD_layout.toml");
        let builtin: HudLayout = toml::from_str(data).expect("layout should be valid toml");
        assert_eq!(builtin.anchor_name, NamedAnchor::BottomLeft);
        assert_eq!(builtin.anchor_point().x, 150.0);
        assert_eq!(builtin.anchor_point().y, 1290.0);
    }

    #[test]
    fn other_layouts_parse() {
        let data = include_str!("../../layouts/SoulsyHUD_centered.toml");
        let centered: HudLayout1 =
            toml::from_str(data).expect("layout should be valid toml");
        assert_eq!(centered.anchor_name, NamedAnchor::Center);
        assert_eq!(centered.anchor_point().x, 1720.0);
        assert_eq!(centered.anchor_point().y, 720.0);

        let data = include_str!("../../layouts/hexagons/SoulsyHUD_hexagons_lr.toml");
        let hexa1: HudLayout1 = toml::from_str(data).expect("layout should be valid toml");
        assert_eq!(hexa1.anchor_name, NamedAnchor::TopRight);
        assert_eq!(hexa1.anchor_point().x, 3290.0);
        assert_eq!(hexa1.anchor_point().y, 150.0);

        let data = include_str!("../../layouts/hexagons/SoulsyHUD_hexagons_tb.toml");
        let hexa2: HudLayout1 = toml::from_str(data).expect("layout should be valid toml");
        assert_eq!(hexa2.anchor_name, NamedAnchor::BottomRight);
        assert_eq!(hexa2.anchor_point().x, 3290.0);
        assert_eq!(hexa2.anchor_point().y, 1290.0);

        let data = include_str!("../../layouts/SoulsyHUD_minimal.toml");
        let layout: HudLayout1 =
            toml::from_str(data).expect("layout should be valid toml");
        assert_eq!(layout.anchor_name, NamedAnchor::BottomLeft);
        assert_eq!(layout.anchor_point().x, 150.0);
        assert_eq!(layout.anchor_point().y, 1315.0);
    }
}

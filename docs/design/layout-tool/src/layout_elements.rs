use std::fmt::Display;
use std::path::PathBuf;

use serde::de::{Deserializer, Error};
use serde::{Deserialize, Serialize};

/// Text alignment options
#[derive(Debug, Clone, Hash, PartialEq)]
pub enum Align {
    Left,
    Right,
    Center,
}

/// Named HUD anchor points.
#[derive(Debug, Clone, Hash)]
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
    None,
}

/// An x,y coordinate used to indicate size or an offset.
#[derive(Deserialize, Serialize, Debug, Clone, Default, PartialEq)]
pub struct Point {
    /// Width or side-to-side offset. Negative values move left.
    x: f32,
    /// Height or top-to-bottom offset. Negative values move up.
    y: f32,
}

/// Color as rgba between 0 and 255. The default is white at full alpha.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct ImageElement {
    svg: PathBuf,
    size: Point,
    color: Color,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct IconElement {
    size: Point,
    offset: Point,
    color: Color,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct HotkeyElement {
    size: Point,
    offset: Point,
    color: Color,
    background: Option<ImageElement>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct TextElement {
    offset: Point,
    color: Color,
    #[serde(default, deserialize_with = "crate::deserialize_align")]
    alignment: Align,
    format: String,
    font_size: f32,
}

// TODO TODO TODO
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ProgressElement {
    offset: Point,
    color: Color,
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

impl Default for NamedAnchor {
    fn default() -> Self {
        NamedAnchor::None
    }
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

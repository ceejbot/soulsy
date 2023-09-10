//! The layout for the HUD is read from a TOML file. This data is shared between
//! languages the same way that the user settings are. The Rust side reads the
//! toml; the C++ side uses the data in layout. The majority of the implementation
//! is filing in defaults.

#![allow(non_snake_case, non_camel_case_types)]

use std::fmt::Display;
use std::fs;
use std::io::Write;
use std::sync::Mutex;

use anyhow::Result;
use once_cell::sync::Lazy;
use serde::de::{Deserializer, Error};
use serde::{Deserialize, Serialize};

use crate::plugin::{Action, Align, Color, HudElement, HudLayout, NamedAnchor, Point, SlotLayout};

static LAYOUT_PATH: &str = "./data/SKSE/Plugins/SoulsyHUD_Layout.toml";

/// There can be only one. Not public because we want access managed.
static LAYOUT: Lazy<Mutex<HudLayout>> = Lazy::new(|| Mutex::new(HudLayout::init()));

#[cfg(target_os = "windows")]
use crate::plugin::{resolutionHeight, resolutionWidth};

// mocked screen resolution numbers, because these functions are provided by
// C++ and require imgui etc.
#[cfg(any(target_os = "macos", target_os = "unix"))]
fn resolutionWidth() -> f32 {
    3440.0
}

#[cfg(any(target_os = "macos", target_os = "unix"))]
fn resolutionHeight() -> f32 {
    1440.0
}

/// Read our layout data from the file, or fall back to defaults if the file
/// is not present or is invalid TOML.
pub fn hud_layout() -> HudLayout {
    let layout = LAYOUT
        .lock()
        .expect("Unrecoverable runtime problem: cannot acquire layout lock.");
    layout.clone()
}

impl HudLayout {
    /// Read a layout object from a toml file.
    pub fn read_from_file(pathstr: &str) -> Result<Self> {
        let path = std::path::Path::new(pathstr);
        if !path.exists() {
            // No file? We write out defaults.
            let layout = HudLayout::default();
            let buf = toml::to_string_pretty(&layout)?;
            let mut fp = fs::File::create(path)?;
            write!(fp, "{buf}")?;
            Ok(HudLayout::default())
        } else if let Ok(buf) = fs::read_to_string(path) {
            match toml::from_str::<HudLayout>(&buf) {
                Ok(v) => Ok(v),
                Err(e) => {
                    // We are *not* overwriting a bad TOML file, but we are logging it.
                    // The player might be editing it and experimenting.
                    log::warn!("bad toml in hud layout; {e:?}");
                    Ok(HudLayout::default())
                }
            }
        } else {
            log::warn!(
                "unable to read any data from {}! falling back to defaults",
                LAYOUT_PATH
            );
            Ok(HudLayout::default())
        }
    }

    /// Refresh the layout from the file, to take an out-of-band update and apply it in-game.
    pub fn refresh() {
        let old = hud_layout();
        if let Ok(buf) = toml::to_string_pretty(&old) {
            let backup = format!("{LAYOUT_PATH}.bak");
            let path = std::path::Path::new(&backup);
            if let Ok(mut fp) = fs::File::create(path) {
                if write!(fp, "{buf}").is_ok() {
                    log::info!("backed up old layout to {}", path.display());
                }
            }
        }

        match HudLayout::read_from_file(LAYOUT_PATH) {
            Ok(v) => {
                log::info!(
                    "hud layout read: loc={}, {}; size={}, {}; global scale={};",
                    v.anchor.x,
                    v.anchor.y,
                    v.size.x,
                    v.size.y,
                    v.global_scale
                );
                let mut hudl = LAYOUT
                    .lock()
                    .expect("Unrecoverable runtime problem: cannot acquire layout lock.");
                *hudl = v;
            }
            Err(e) => {
                log::warn!("Failed to read layout file; continuing with previous; {e:?}");
            }
        }
    }

    pub fn init() -> HudLayout {
        match HudLayout::read_from_file(LAYOUT_PATH) {
            Ok(v) => {
                log::info!("successfully initialized HUD layout from player file");
                v
            }
            Err(e) => {
                log::warn!("Failed to read layout file; initializing from defaults; {e:?}");
                HudLayout::default()
            }
        }
    }

    pub fn anchor(&self) -> Point {
        // If we read a named anchor point, turn it into pixels.
        // The anchor point is the location of the hud CENTER, so we offset.
        let screen_width = resolutionWidth();
        let screen_height = resolutionHeight();
        let width = self.size.x * self.global_scale;
        let height = self.size.y * self.global_scale;

        match self.anchor_point {
            NamedAnchor::TopLeft => Point {
                x: width / 2.0,
                y: height / 2.0,
            },
            NamedAnchor::TopRight => Point {
                x: screen_width - width / 2.0,
                y: height / 2.0,
            },
            NamedAnchor::BottomLeft => Point {
                x: width / 2.0,
                y: screen_height - height / 2.0,
            },
            NamedAnchor::BottomRight => Point {
                x: screen_width - width / 2.0,
                y: screen_height - height / 2.0,
            },
            NamedAnchor::Center => Point {
                x: screen_width / 2.0,
                y: screen_height / 2.0,
            },
            NamedAnchor::CenterTop => Point {
                x: screen_width / 2.0,
                y: height / 2.0,
            },
            NamedAnchor::CenterBottom => Point {
                x: screen_width / 2.0,
                y: screen_height - height / 2.0,
            },
            NamedAnchor::LeftCenter => Point {
                x: width / 2.0,
                y: screen_height / 2.0,
            },
            NamedAnchor::RightCenter => Point {
                x: screen_width - width / 2.0,
                y: screen_height / 2.0,
            },
            _ => self.anchor.clone(),
        }
    }
}

impl Default for HudLayout {
    fn default() -> Self {
        // Yes, this is annoyingly complex. Obviously this solution is wrong.
        let power_default = SlotLayout {
            element: HudElement::Power,
            name: "Shouts/Powers".to_string(),
            align_text: Align::Left,
            offset: Point { x: 0.0, y: -125.0 },
            size: Point { x: 125.0, y: 125.0 },
            bg_color: Color::default(),
            icon_color: Color {
                r: 255,
                g: 255,
                b: 200,
                a: 255,
            },
            icon_size: Point { x: 120.0, y: 120.0 },
            icon_offset: Point::default(),
            hotkey_color: Color::default(),
            hotkey_offset: Point { x: 10.0, y: 0.0 },
            hotkey_size: Point { x: 30.0, y: 30.0 },
            hotkey_bg_color: Color::default(),
            count_offset: Point { x: 0.0, y: 0.0 },
            count_font_size: 20.0,
            count_color: Color::default(),
            name_color: Color::default(),
            name_offset: Point { x: 0.0, y: 20.0 },
            name_font_size: 20.0,
        };
        let utility_default = SlotLayout {
            element: HudElement::Utility,
            name: "Consumables".to_string(),
            align_text: Align::Right,
            offset: Point { x: 0.0, y: 125.0 },
            size: Point { x: 125.0, y: 125.0 },
            bg_color: Color::default(),
            icon_color: Color {
                r: 200,
                g: 255,
                b: 255,
                a: 255,
            },
            icon_size: Point { x: 120.0, y: 120.0 },
            icon_offset: Point::default(),
            hotkey_color: Color::default(),
            hotkey_offset: Point { x: 10.0, y: 0.0 },
            hotkey_size: Point { x: 30.0, y: 30.0 },
            hotkey_bg_color: Color::default(),
            count_offset: Point { x: 0.0, y: 0.0 },
            count_font_size: 20.0,
            count_color: Color::default(),
            name_color: Color::default(),
            name_offset: Point { x: 0.0, y: 20.0 },
            name_font_size: 20.0,
        };
        let left_default = SlotLayout {
            element: HudElement::Left,
            name: "Left Hand".to_string(),
            align_text: Align::Right,
            offset: Point { x: -125.0, y: 0.0 },
            size: Point { x: 125.0, y: 125.0 },
            bg_color: Color::default(),
            icon_color: Color {
                r: 255,
                g: 200,
                b: 255,
                a: 255,
            },
            icon_size: Point { x: 120.0, y: 120.0 },
            icon_offset: Point::default(),
            hotkey_color: Color::default(),
            hotkey_offset: Point { x: 10.0, y: 0.0 },
            hotkey_size: Point { x: 30.0, y: 30.0 },
            hotkey_bg_color: Color::default(),
            count_offset: Point { x: 0.0, y: 0.0 },
            count_font_size: 20.0,
            count_color: Color::default(),
            name_color: Color::default(),
            name_offset: Point { x: 0.0, y: 20.0 },
            name_font_size: 20.0,
        };
        let right_default = SlotLayout {
            element: HudElement::Right,
            name: "Right Hand".to_string(),
            align_text: Align::Left,
            offset: Point { x: 125.0, y: 0.0 },
            size: Point { x: 125.0, y: 125.0 },
            bg_color: Color::default(),
            icon_color: Color {
                r: 200,
                g: 255,
                b: 200,
                a: 255,
            },
            icon_size: Point { x: 120.0, y: 120.0 },
            icon_offset: Point::default(),
            hotkey_color: Color::default(),
            hotkey_offset: Point { x: 10.0, y: 0.0 },
            hotkey_size: Point { x: 30.0, y: 30.0 },
            hotkey_bg_color: Color::default(),
            count_offset: Point { x: 0.0, y: 0.0 },
            count_font_size: 20.0,
            count_color: Color::default(),
            name_color: Color::default(),
            name_offset: Point { x: 0.0, y: 20.0 },
            name_font_size: 20.0,
        };
        let ammo_default = SlotLayout {
            element: HudElement::Ammo,
            name: "Ammo".to_string(),
            align_text: Align::Left,
            offset: Point { x: 0.0, y: 0.0 },
            size: Point { x: 62.0, y: 62.0 },
            bg_color: Color::default(),
            icon_color: Color {
                r: 200,
                g: 200,
                b: 255,
                a: 255,
            },
            icon_size: Point { x: 50.0, y: 50.0 },
            icon_offset: Point::default(),
            hotkey_color: Color::default(),
            hotkey_offset: Point { x: 10.0, y: 0.0 },
            hotkey_size: Point { x: 30.0, y: 30.0 },
            hotkey_bg_color: Color::default(),
            count_offset: Point { x: 0.0, y: 0.0 },
            count_font_size: 20.0,
            count_color: Color::default(),
            name_color: Color::default(),
            name_offset: Point { x: 0.0, y: 20.0 },
            name_font_size: 20.0,
        };

        let layouts = vec![
            power_default,
            utility_default,
            left_default,
            right_default,
            ammo_default,
        ];
        Self {
            global_scale: 1.0,
            anchor_point: NamedAnchor::None,
            anchor: Point {
                x: 100.0,
                y: 1400.0,
            },
            size: Point { x: 300.0, y: 300.0 },
            bg_color: Color::default(),
            hide_ammo_when_irrelevant: false,
            hide_left_when_irrelevant: false,
            layouts,
            animation_alpha: 51,
            animation_duration: 0.1,
            font: "futura-book-bt.ttf".to_string(),
            font_size: 20.0,
            chinese_full_glyphs: false,
            simplified_chinese_glyphs: true,
            cyrillic_glyphs: true,
            japanese_glyphs: false,
            korean_glyphs: false,
            thai_glyphs: false,
            vietnamese_glyphs: false,
        }
    }
}

impl SlotLayout {
    pub fn default_for_element(element: HudElement, name: &str) -> Self {
        Self {
            element,
            name: name.to_string(),
            ..Default::default()
        }
    }
}

impl Default for SlotLayout {
    fn default() -> Self {
        Self {
            element: HudElement { repr: 1 },
            name: "unknown".to_string(),
            align_text: Align::Left,
            offset: Point::default(),
            size: Point { x: 125.0, y: 125.0 },
            bg_color: Color::default(),
            icon_size: Point { x: 100.0, y: 100.0 },
            icon_color: Color {
                r: 200,
                g: 200,
                b: 255,
                a: 125,
            },
            icon_offset: Point::default(),
            hotkey_color: Color::default(),
            hotkey_offset: Point { x: 20.0, y: 0.0 },
            hotkey_size: Point { x: 30.0, y: 30.0 },
            hotkey_bg_color: Color::default(),
            count_offset: Point::default(),
            count_font_size: 20.0,
            count_color: Color::default(),
            name_offset: Point::default(),
            name_color: Color::default(),
            name_font_size: 20.0,
        }
    }
}

impl Default for Align {
    fn default() -> Self {
        Align::Left
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

impl Point {
    pub fn offset_by(&self, offset: Point) -> Point {
        Point {
            x: self.x + offset.x,
            y: self.y + offset.y,
        }
    }
}

// ---------- Align

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
        "center_bottom" => Ok(NamedAnchor::CenterBottom),
        "left_center" => Ok(NamedAnchor::LeftCenter),
        "right_center" => Ok(NamedAnchor::RightCenter),
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

    #[test]
    fn parses_anchor_points() {
        let layout = HudLayout::read_from_file("layouts/SoulsyHUD_topleft.toml")
            .expect("failed to parse known-good layout toml");
        assert_eq!(layout.anchor_point, NamedAnchor::None);
        assert_eq!(layout.anchor().x, 150.0);
        assert_eq!(layout.anchor().y, 150.0);
    }

    #[test]
    fn parses_named_anchors() {
        let builtin = HudLayout::read_from_file("data/SKSE/plugins/SoulsyHUD_layout.toml")
            .expect("failed to parse known-good layout toml");
        assert_eq!(builtin.anchor_point, NamedAnchor::BottomLeft);
        assert_eq!(builtin.anchor().x, 150.0);
        assert_eq!(builtin.anchor().y, 1290.0);

        let centered = HudLayout::read_from_file("layouts/SoulsyHUD_centered.toml")
            .expect("failed to parse known-good layout toml");
        assert_eq!(centered.anchor_point, NamedAnchor::Center);
        assert_eq!(centered.anchor().x, 1720.0);
        assert_eq!(centered.anchor().y, 720.0);

        let hexa1 = HudLayout::read_from_file("layouts/hexagons/SoulsyHUD_hexagons_lr.toml")
            .expect("failed to parse known-good layout toml");
        assert_eq!(hexa1.anchor_point, NamedAnchor::TopRight);
        assert_eq!(hexa1.anchor().x, 3290.0);
        assert_eq!(hexa1.anchor().y, 150.0);

        let hexa1 = HudLayout::read_from_file("layouts/hexagons/SoulsyHUD_hexagons_tb.toml")
            .expect("failed to parse known-good layout toml");
        assert_eq!(hexa1.anchor_point, NamedAnchor::BottomRight);
        assert_eq!(hexa1.anchor().x, 3290.0);
        assert_eq!(hexa1.anchor().y, 1290.0);

        let layout = HudLayout::read_from_file("layouts/SoulsyHUD_minimal.toml")
            .expect("failed to parse known-good layout toml");
        assert_eq!(layout.anchor_point, NamedAnchor::BottomLeft);
        assert_eq!(layout.anchor().x, 150.0);
        assert_eq!(layout.anchor().y, 1315.0);
    }
}

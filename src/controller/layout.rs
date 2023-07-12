//! The layout for the HUD is read from a TOML file. This data is shared between
//! languages the same way that the user settings are. The Rust side reads the
//! toml; the C++ side uses the data in layout. The majority of the implementation
//! is filing in defaults.

use std::path::PathBuf;
use std::sync::Mutex;

use anyhow::Result;
use once_cell::sync::Lazy;

use crate::plugin::{Color, HudElement, HudLayout, Point, SlotLayout};

static LAYOUT_PATH: &str = "./data/SKSE/Plugins/SoulsyHUD_HudLayout.toml";

/// There can be only one. Not public because we want access managed.
static LAYOUT: Lazy<Mutex<HudLayout>> = Lazy::new(|| Mutex::new(HudLayout::refresh()));

/// Read our layout data from the file, or fall back to defaults if the file
/// is not present or is invalid TOML.
pub fn layout() -> HudLayout {
    let layout = LAYOUT.lock().unwrap();
    layout.clone()
}

impl HudLayout {
    /// Read a settings object from a toml file.
    pub fn read_from_file() -> Result<Self> {
        let buf = std::fs::read_to_string(PathBuf::from(LAYOUT_PATH))?;
        let layout = toml::from_str::<HudLayout>(&buf)?;
        Ok(layout)
    }

    /// Refresh the layout from the file, to take an out-of-band update and apply it in-game.
    pub fn refresh() -> HudLayout {
        match HudLayout::read_from_file() {
            Ok(v) => {
                log::info!("successfully refreshed HUD layout");
                v
            }
            Err(e) => {
                log::warn!("Failed to read layout file; continuing with defaults; {e:?}");
                HudLayout::default()
            }
        }
    }
}

impl Default for HudLayout {
    fn default() -> Self {
        let layouts = vec![
            SlotLayout::default_for_element(HudElement::Power),
            SlotLayout::default_for_element(HudElement::Utility),
            SlotLayout::default_for_element(HudElement::Left),
            SlotLayout::default_for_element(HudElement::Right),
            SlotLayout::default_for_element(HudElement::Ammo),
        ];

        Self {
            anchor: Point { x: 105.0, y: 105.0 },
            size: Point { x: 450.0, y: 450.0 },
            bg_color: Color::default(),
            layouts,
            debug: false,
            animation_alpha: 51,
            animation_duration: 0.1,
        }
    }
}

impl SlotLayout {
    pub fn default_for_element(element: HudElement) -> Self {
        Self {
            element,
            ..Default::default()
        }
    }
}

impl Default for SlotLayout {
    fn default() -> Self {
        Self {
            element: HudElement { repr: 1 },
            offset: Point::default(),
            size: Point { x: 150.0, y: 150.0 },
            bg_scale: 1.0,
            bg_color: Color::default(),
            icon_scale: 1.0,
            icon_color: Color {
                r: 255,
                g: 255,
                b: 255,
                a: 125,
            },
            hotkey_color: Color::default(),
            hotkey_offset: Point { x: 20.0, y: 0.0 },
            hotkey_scale: 1.0,
            hotkey_bg_color: Color::default(),
            text_offset: Point::default(),
            count_font_size: 20.0,
            count_color: Color::default(),
            name_offset: Point::default(),
            name_font_size: 20.0,
            name_color: Color::default(),
        }
    }
}

pub fn create_color(r: u8, g: u8, b: u8, a: u8) -> Color {
    Color { r, g, b, a }
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

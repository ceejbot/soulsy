//! The layout for the HUD is read from a TOML file. This data is shared between
//! languages the same way that the user settings are. The Rust side reads the
//! toml; the C++ side uses the data in layout. The majority of the implementation
//! is filing in defaults.

#![allow(non_snake_case, non_camel_case_types)]

use std::fs;
use std::io::Write;
use std::sync::Mutex;

use anyhow::Result;
use once_cell::sync::Lazy;

use crate::plugin::{HudLayout1, NamedAnchor, Point};

static LAYOUT_PATH: &str = "./data/SKSE/Plugins/SoulsyHUD_Layout.toml";

/// There can be only one. Not public because we want access managed.
static LAYOUT: Lazy<Mutex<HudLayout1>> = Lazy::new(|| Mutex::new(HudLayout1::init()));

/// Lazy parsing of the compile-time include of the default layout, as a fallback.
static DEFAULT_LAYOUT: Lazy<HudLayout1> = Lazy::new(HudLayout1::default);

#[cfg(not(test))]
use crate::plugin::{resolutionHeight, resolutionWidth};

// mocked screen resolution numbers, because these functions are provided by
// C++ and require imgui etc.
#[cfg(test)]
fn resolutionWidth() -> f32 {
    3440.0
}

#[cfg(test)]
fn resolutionHeight() -> f32 {
    1440.0
}

/// Read our layout data from the file, or fall back to defaults if the file
/// is not present or is invalid TOML.
pub fn hud_layout() -> HudLayout1 {
    let layout = LAYOUT
        .lock()
        .expect("Unrecoverable runtime problem: cannot acquire layout lock.");
    layout.clone()
}

impl HudLayout1 {
    /// Read a layout object from a toml file.
    pub fn read_from_file(pathstr: &str) -> Result<Self> {
        let path = std::path::Path::new(pathstr);
        if !path.exists() {
            // No file? We write out defaults.
            let layout = DEFAULT_LAYOUT.clone();
            let buf = toml::to_string_pretty(&layout)?;
            let mut fp = fs::File::create(path)?;
            write!(fp, "{buf}")?;
            Ok(layout)
        } else if let Ok(buf) = fs::read_to_string(path) {
            match toml::from_str::<HudLayout1>(&buf) {
                Ok(v) => Ok(v),
                Err(e) => {
                    // We are *not* overwriting a bad TOML file, but we are logging it.
                    // The player might be editing it and experimenting.
                    log::warn!("Bad TOML in hud layout!; {e:?}");
                    Ok(DEFAULT_LAYOUT.clone())
                }
            }
        } else {
            log::warn!(
                "Unable to read any data from {}! Falling back to defaults",
                LAYOUT_PATH
            );
            Ok(DEFAULT_LAYOUT.clone())
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
                    log::info!(
                        "Previous layout file has been backed up to {}",
                        path.display()
                    );
                }
            }
        }

        match HudLayout1::read_from_file(LAYOUT_PATH) {
            Ok(v) => {
                log::info!(
                    "hud layout read: loc={}, {}; size={}, {}; global scale={};",
                    v.anchor_point().x,
                    v.anchor_point().y,
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

    pub fn init() -> HudLayout1 {
        match HudLayout1::read_from_file(LAYOUT_PATH) {
            Ok(v) => {
                log::info!("Successfully initialized HUD layout from TOML file.");
                v
            }
            Err(e) => {
                log::warn!("Failed to read TOML layout file; initializing from defaults; {e:?}");
                DEFAULT_LAYOUT.clone()
            }
        }
    }

    pub fn anchor_point(&self) -> Point {
        // If we read a named anchor point, turn it into pixels.
        // The anchor point is the location of the hud CENTER, so we offset.
        let screen_width = resolutionWidth();
        let screen_height = resolutionHeight();
        let width = self.size.x * self.global_scale;
        let height = self.size.y * self.global_scale;

        match self.anchor_name {
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
            _ => {
                if self.anchor == Point::default() {
                    log::info!("Layout has neither a named anchor nor an anchor point. Falling back to top left.");
                    Point {
                        x: width / 2.0,
                        y: height / 2.0,
                    }
                } else {
                    self.anchor.clone()
                }
            }
        }
    }
}

impl Default for HudLayout1 {
    fn default() -> Self {
        // compile-time include of default layout toml
        let buf = include_str!("../../data/SKSE/plugins/SoulsyHUD_Layout.toml");
        toml::from_str::<HudLayout1>(buf)
            .expect("Default layout is not valid toml! Cannot proceed.")
    }
}

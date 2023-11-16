//! Layouts: two schema versions and associated machinery.

#![allow(non_snake_case, non_camel_case_types)]

use serde::{Deserialize, Serialize};

pub mod layout_v1;
pub mod layout_v2;
pub mod shared;

use std::fs;
use std::io::Write;
use std::sync::Mutex;

use anyhow::Result;
pub use layout_v1::HudLayout1;
pub use layout_v2::{HudLayout2, TextElement};
use once_cell::sync::Lazy;

use self::shared::NamedAnchor;
use crate::plugin::{LayoutFlattened, Point};

static LAYOUT_PATH: &str = "./data/SKSE/Plugins/SoulsyHUD_Layout.toml";

/// There can be only one. Not public because we want access managed.
static LAYOUT: Lazy<Mutex<LayoutFlattened>> = Lazy::new(|| Mutex::new(Layout::initialize()));

/// Lazy parsing of the compile-time include of the default layout, as a fallback.
static DEFAULT_LAYOUT: Lazy<HudLayout2> = Lazy::new(HudLayout2::default);

/// The accessor for anybody who needs to use the layout.
pub fn hud_layout() -> LayoutFlattened {
    let layout = LAYOUT
        .lock()
        .expect("Unrecoverable runtime problem: cannot acquire layout lock.");
    layout.clone()
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Layout {
    Version1(Box<HudLayout1>),
    Version2(Box<HudLayout2>),
}

impl Default for Layout {
    fn default() -> Self {
        Layout::Version2(Box::new(DEFAULT_LAYOUT.clone()))
    }
}

impl Layout {
    /// Read the layout at startup, falling back if necessary.
    pub fn initialize() -> LayoutFlattened {
        let layout = match Layout::read_from_file(LAYOUT_PATH) {
            Ok(v) => v,
            Err(e) => {
                log::warn!("Problem reading the default layout file! {e:?}");
                Layout::default()
            }
        };
        layout.flatten()
    }

    /// Read the layout from disk to pick up any changes to the file.
    pub fn refresh() {
        match Layout::read_from_file(LAYOUT_PATH) {
            Ok(v) => {
                let mut hudl = LAYOUT
                    .lock()
                    .expect("Unrecoverable runtime problem: cannot acquire layout lock.");
                *hudl = v.flatten();
            }
            Err(e) => {
                log::warn!("Problem reading the default layout file! Not updating.");
                log::warn!("{e:?}");
            }
        };
    }

    /// Read a layout object from a toml file.
    pub fn read_from_file(pathstr: &str) -> Result<Self> {
        let path = std::path::Path::new(pathstr);
        if !path.exists() {
            // No file? We write out defaults.
            let layout = DEFAULT_LAYOUT.clone();
            let buf = toml::to_string_pretty(&layout)?;
            let mut fp = fs::File::create(path)?;
            write!(fp, "{buf}")?;
            return Ok(Layout::Version2(Box::new(layout)));
        }

        let buf = fs::read_to_string(path)?;
        let parsed = toml::from_str::<Layout>(&buf)?;
        Ok(parsed)
    }

    pub fn flatten(&self) -> LayoutFlattened {
        match self {
            // *v dereference the ref-to-box, **v unbox, &**v borrow
            Layout::Version1(v) => LayoutFlattened::from(&**v),
            Layout::Version2(v) => LayoutFlattened::from(&**v),
        }
    }
}

pub fn anchor_point(
    global_scale: f32,
    size: &Point,
    anchor_name: &NamedAnchor,
    maybe_anchor: Option<&Point>,
) -> Point {
    // If we read a named anchor point, turn it into pixels.
    // The anchor point is the location of the hud CENTER, so we offset.
    let screen_width = resolutionWidth();
    let screen_height = resolutionHeight();

    let width = size.x * global_scale;
    let height = size.y * global_scale;

    match anchor_name {
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
            if let Some(anchor) = maybe_anchor {
                if *anchor == Point::default() {
                    log::info!("Layout has neither a named anchor nor an anchor point. Falling back to top left.");
                    Point {
                        x: width / 2.0,
                        y: height / 2.0,
                    }
                } else {
                    anchor.clone()
                }
            } else {
                // note the opportunity for refactoring but I am too stressed right now
                Point {
                    x: width / 2.0,
                    y: height / 2.0,
                }
            }
        }
    }
}

impl Default for LayoutFlattened {
    fn default() -> Self {
        todo!()
    }
}

impl Point {
    pub fn scale(&self, factor: f32) -> Point {
        Point {
            x: self.x * factor,
            y: self.y * factor,
        }
    }

    pub fn translate(&self, other: &Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

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

#[cfg(test)]
mod tests {
    use super::shared::NamedAnchor;
    use super::*;

    #[test]
    fn can_lazy_load_layouts() {
        let layout = hud_layout();
        assert_eq!(layout.anchor.x, 150.0);
        assert_eq!(layout.anchor.y, 1290.0);
    }

    #[test]
    fn can_load_v2_layouts() {
        let squarev1 = Layout::read_from_file("layouts/square/SoulsyHUD_Layout.toml")
            .expect("the original square layout can be loaded");
        let squarev2 = Layout::read_from_file("layouts/square/LayoutV2.toml")
            .expect("the square layout has been ported");
        let flat1 = squarev1.flatten();
        let flat2 = squarev2.flatten();
        assert_eq!(flat1.bg_size, flat2.bg_size);
        assert_eq!(flat1.anchor, flat2.anchor);
        assert_eq!(
            flat1.hide_ammo_when_irrelevant,
            flat2.hide_ammo_when_irrelevant
        );

        assert_eq!(flat1.slots.len(), flat2.slots.len());
        assert_eq!(flat1.slots.len(), 6);

        // This is fragile, because it depends on both the order of flattening &
        // the order things are in the layout file.
        let power1 = flat1.slots.first().expect("wat");
        let power2 = flat2.slots.first().expect("wat");
        assert_eq!(power1.element, power2.element);
        assert_eq!(power1.center, power2.center);
        assert_eq!(power1.icon_center, power2.icon_center);
        assert_eq!(power1.hotkey_center, power2.hotkey_center);
    }

    #[test]
    fn default_layout_exists() {
        // TODO
    }

    #[derive(Deserialize, Serialize, Debug, Clone)]
    struct TestAnchor {
        #[serde(default, deserialize_with = "super::shared::deserialize_named_anchor")]
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
        let data =
            std::fs::read_to_string("layouts/SoulsyHUD_topleft.toml").expect("file not found?");
        let layout: HudLayout1 =
            toml::from_str(data.as_str()).expect("layout should be valid toml");
        assert_eq!(layout.anchor_name, NamedAnchor::None);
        assert_eq!(layout.anchor_point().x, 150.0);
        assert_eq!(layout.anchor_point().y, 150.0);
    }

    #[test]
    fn parses_named_anchors() {
        let data = std::fs::read_to_string("data/SKSE/plugins/SoulsyHUD_layout.toml")
            .expect("file not found?");
        let builtin: HudLayout1 =
            toml::from_str(data.as_str()).expect("layout should be valid toml");
        assert_eq!(builtin.anchor_name, NamedAnchor::BottomLeft);
        assert_eq!(builtin.anchor_point().x, 150.0);
        assert_eq!(builtin.anchor_point().y, 1290.0);

        let data =
            std::fs::read_to_string("layouts/SoulsyHUD_centered.toml").expect("file not found?");
        let centered: HudLayout1 =
            toml::from_str(data.as_str()).expect("layout should be valid toml");
        assert_eq!(centered.anchor_name, NamedAnchor::Center);
        assert_eq!(centered.anchor_point().x, 1720.0);
        assert_eq!(centered.anchor_point().y, 720.0);

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

        let data =
            std::fs::read_to_string("layouts/SoulsyHUD_minimal.toml").expect("file not found?");
        let layout: HudLayout1 =
            toml::from_str(data.as_str()).expect("layout should be valid toml");
        assert_eq!(layout.anchor_name, NamedAnchor::BottomLeft);
        assert_eq!(layout.anchor_point().x, 150.0);
        assert_eq!(layout.anchor_point().y, 1315.0);
    }
}

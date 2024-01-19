//! Layouts: two schema versions and associated machinery.

pub mod layout_v1;
pub mod layout_v2;
pub mod shared;

use std::fs;
use std::io::Write;
use std::sync::Mutex;

use eyre::{eyre, Context, Result};
pub use layout_v1::HudLayout1;
pub use layout_v2::{HudLayout2, TextElement};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use self::shared::NamedAnchor;
use crate::control::notify;
use crate::controller::control::translated_key;
use crate::controller::user_settings;
use crate::plugin::{LayoutFlattened, Point};

static LAYOUT_PATH: &str = "./data/SKSE/Plugins/SoulsyHUD_Layout.toml";

/// There can be only one. Not public because we want access managed.
static LAYOUT: Lazy<Mutex<LayoutFlattened>> = Lazy::new(|| Mutex::new(Layout::initialize()));

/// Lazy parsing of the compile-time include of the default layout, as a fallback.
static DEFAULT_LAYOUT: Lazy<HudLayout2> = Lazy::new(HudLayout2::fallback);

/// The accessor for anybody who needs to use the layout.
pub fn hud_layout() -> LayoutFlattened {
    let layout = LAYOUT
        .lock()
        .expect("Unrecoverable runtime problem: cannot acquire layout lock.");
    layout.clone()
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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
                log::warn!("Problem reading the enabled layout file! {e:#}");
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
                log::warn!("{e:#}");
                log::warn!("In-game layout not updated.")
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

        let buf = fs::read_to_string(path)
            .wrap_err_with(|| format!("Unable to read the layout file: {}", pathstr))?;
        match toml::from_str::<Layout>(&buf) {
            Ok(v) => {
                // could notify here if we wanted with $SoulsyHUD_Layout_Refreshed_Msg
                Ok(v)
            }
            Err(_) => {
                let msg = translated_key("$SoulsyHUD_Layout_Failed_Msg");
                notify(&msg);
                // We know these are both errors or we wouldn't be here.
                let v1err = HudLayout1::read_from_file(pathstr)
                    .expect_err("Layout parsing failed but v1 succeeded? WAT.");
                eprintln!("{v1err:#}");
                let v2err = HudLayout2::read_from_file(pathstr)
                    .expect_err("Layout parsing failed but v2 succeeded? WAT.");
                eprintln!("{v2err:#}");
                Err(eyre!(
                    "The toml file at '{}' can't be parsed as a SoulsyHUD layout.",
                    pathstr
                ))
            }
        }
    }

    pub fn flatten(&self) -> LayoutFlattened {
        match self {
            // *v dereference the ref-to-box, **v unbox, &**v borrow
            Layout::Version1(v) => LayoutFlattened::from(&**v),
            Layout::Version2(v) => LayoutFlattened::from(&**v),
        }
    }

    pub fn anchor_point(&self) -> Point {
        match self {
            Layout::Version1(v) => v.anchor_point(),
            Layout::Version2(v) => v.anchor_point(),
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
    let config = *user_settings();
    let screen_width = displayWidth();
    let screen_height = displayHeight();

    let width = size.x * global_scale;
    let height = size.y * global_scale;

    let user_pref_anchor = config.anchor_loc();
    let anchor_to_use = if !matches!(user_pref_anchor, &NamedAnchor::None) {
        user_pref_anchor
    } else {
        anchor_name
    };

    match anchor_to_use {
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
        NamedAnchor::None => {
            if let Some(anchor) = maybe_anchor {
                anchor.clone()
            } else {
                // note the opportunity for refactoring but I am too stressed right now
                Point {
                    x: width * 0.5,
                    y: height * 0.5,
                }
            }
        }
    }
}

impl Default for LayoutFlattened {
    fn default() -> Self {
        Layout::default().flatten()
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

    pub fn origin() -> Self {
        Point { x: 0.0, y: 0.0 }
    }
}

#[cfg(not(test))]
use crate::plugin::{displayHeight, displayWidth};

// Mocked screen resolution numbers, because these functions are provided by
// C++ and require imgui etc. The names come from C++ and are not snake case.
#[cfg(test)]
#[allow(non_snake_case)]
fn displayWidth() -> f32 {
    3440.0
}

#[cfg(test)]
#[allow(non_snake_case)]
fn displayHeight() -> f32 {
    1440.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_functions_behave() {
        let point = Point { x: 10.0, y: 15.0 };
        let puncta = Point { x: -5.0, y: 2.0 };
        assert_eq!(point.translate(&puncta), puncta.translate(&point));
        assert_eq!(point.scale(6.0), Point { x: 60.0, y: 90.0 });
        assert_eq!(puncta.scale(-2.0), Point { x: 10.0, y: -4.0 });
        assert_eq!(
            puncta.scale(-2.0).translate(&puncta),
            Point { x: 5.0, y: -2.0 }
        );
    }

    #[test]
    fn can_lazy_load_layouts() {
        let layout = hud_layout();
        assert_eq!(layout.anchor.x, 150.0);
        assert_eq!(layout.anchor.y, 1290.0);
    }

    #[test]
    fn can_load_v2_layouts() {
        let squarev1 = Layout::read_from_file("tests/fixtures/layout-v1.toml")
            .expect("the original square layout can be loaded");
        let squarev2 = Layout::read_from_file(
            "installer/core/SKSE/plugins/soulsy_layouts/SoulsyHUD_square.toml",
        )
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
        // the order things are in the layout file. Consider sorting.
        let power1 = flat1.slots.first().expect("wat");
        let power2 = flat2.slots.first().expect("wat");
        assert_eq!(power1.element, power2.element);
        assert_eq!(power1.center, power2.center);
        assert_eq!(power1.icon_center, power2.icon_center);
        assert_eq!(power1.hotkey_center, power2.hotkey_center);
    }

    #[test]
    fn default_layout_exists() {
        let fpath = std::path::Path::new(
            "installer/core/SKSE/plugins/soulsy_layouts/SoulsyHUD_default.toml",
        );
        assert!(fpath.exists());
    }

    #[test]
    fn default_flattened_layout_exists() {
        let defaulted = LayoutFlattened::default();
        assert_eq!(
            defaulted.anchor,
            Point {
                x: 150.0,
                y: 1290.0
            }
        );
    }

    #[test]
    fn anchor_points_respected() {
        let buf = include_str!("../../tests/fixtures/named-anchor.toml");
        let named: HudLayout2 =
            toml::from_str(buf).expect("named-anchor.toml fixture is a valid layout");
        let buf2 = include_str!("../../tests/fixtures/anchor-point.toml");
        let pointed: HudLayout2 =
            toml::from_str(buf2).expect("named-anchor.toml fixture is a valid layout");
        assert_eq!(
            pointed.anchor_point(),
            Point {
                x: 150.0,
                y: 1290.0
            }
        );
        assert_eq!(pointed.anchor_point(), named.anchor_point());
    }

    #[test]
    fn anchor_points_respect_settings() {
        // override the defaults with what we need for this test
        let _ = crate::controller::UserSettings::refresh_with("tests/fixtures/test-settings.ini");

        let named = Layout::read_from_file("tests/fixtures/named-anchor.toml")
            .expect("this test fixture exists and is valid");
        // this layout has bottom left as an anchor point
        // the test settings override with "center"
        let relocated = named.anchor_point();
        assert_eq!(
            relocated,
            Point {
                x: 1720.0,
                y: 720.0
            }
        );

        let flattened = named.flatten();
        assert_eq!(flattened.anchor, relocated);
    }
}

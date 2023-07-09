use std::path::PathBuf;

/// The layout for the HUD is read from a TOML file. The user can't change
/// it at runtime, but they can tweak and reload. This is not a great workflow!
/// But it does allow theming and adjusting for the determined, and we can
/// improve this over time.
///
/// This data is shared between languages the same way that the user settings are.
/// The Rust side reads the toml; the C++ side uses the data in layout.
use anyhow::Result;
use once_cell::sync::OnceCell;

use crate::plugin::HudLayout;

static LAYOUT_PATH: &str = "./data/SKSE/Plugins/SoulsyHUD_HudLayout.toml";

/// There can be only one. Not public because we want access managed.
static LAYOUT: OnceCell<HudLayout> = OnceCell::new();

impl HudLayout {
    /// Read a settings object from a toml file.
    pub fn read_from_file() -> Result<Self> {
        let buf = std::fs::read_to_string(PathBuf::from(LAYOUT_PATH))?;
        let layout = toml::from_str::<HudLayout>(&buf)?;
        Ok(layout)
    }
}

/// Read our layout data from the file, or fall back to defaults if the file
/// is not present.
///
/// Lazily initialized on first request for the layout object.
pub fn layout() -> &'static HudLayout {
    if LAYOUT.get().is_none() {
        let layout = match HudLayout::read_from_file() {
            Ok(v) => v,
            Err(e) => {
                log::warn!("Failed to read settings file; continuing with defaults; {e:?}");

                HudLayout::default()
            }
        };
        LAYOUT.set(layout).unwrap();
    }

    // If this fails, the universe is in a bad state. Crashing is fine.
    LAYOUT.get().unwrap()
}

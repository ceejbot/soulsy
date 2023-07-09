use anyhow::Result;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};

use std::path::PathBuf;

static SETTINGS_PATH: &str = "./data/SKSE/Plugins/SoulsySettings.toml";

/// There can be only one. Not public because we want access managed.
static SETTINGS: OnceCell<Settings> = OnceCell::new();

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
/// User-modifiable settings for HUD behavior. Doesn't manage cycles.
pub struct Settings {
    /// The key for the left hand's cycle.
    pub left: u32,
    /// The key for the right hand's cycle.
    pub right: u32,
    /// The key for powers.
    pub power: u32,
    /// The key for utility items.
    pub utility: u32,
    /// The key to activate or use a utility item.
    pub activate: u32,
    /// Show/hide shortcut key.
    pub showhide: u32,
    /// Whether to fade out hud when not in combat.
    pub fade: bool,
    /// True if we should enable debug logging.
    pub debug: bool,
}

impl Settings {
    /// Read a settings object from a toml file.
    pub fn read_from_file() -> Result<Self> {
        let buf = std::fs::read_to_string(&PathBuf::from(SETTINGS_PATH))?;
        let settings = toml::from_str::<Settings>(&buf)?;
        Ok(settings)
    }

    /// Serialize settings to a string of valid toml and write it out.
    ///
    /// This version returns a rust result; it is called by the C++ bridge
    /// with a boolean indicating success or failure.
    fn write_to_file(&self) -> Result<()> {
        let toml = toml::to_string(self)?;
        std::fs::write(&SETTINGS_PATH, toml)?;
        Ok(())
    }

    /// Write our settings to disk. We are swallowing errors we can recover from.
    /// At least in theory.
    pub fn write_settings(&self) -> bool {
        match self.write_to_file() {
            Ok(_) => true,
            Err(e) => {
                log::warn!("Failed to serialize settings to toml! {e:?}");
                false
            }
        }
    }
}

/// Read our user settings from the file, or fall back to defaults if the file
/// is not present.
///
/// Lazily initialized on first request for the settings object. Will try to
/// write defaults if the file couldn't be found, but will gamely continue if it
/// fails.
pub fn settings() -> &'static Settings {
    if SETTINGS.get().is_none() {
        let settings = match Settings::read_from_file() {
            Ok(v) => v,
            Err(e) => {
                log::warn!("Failed to read settings file; continuing with defaults; {e:?}");
                let s = Settings::default();
                if s.write_to_file().is_err() {
                    log::warn!("Failed to write default settings to a file. Proceeding.");
                }
                s
            }
        };
        SETTINGS.set(settings).unwrap();
    }

    // If this fails, the universe is in a bad state. Crashing is fine.
    SETTINGS.get().unwrap()
}

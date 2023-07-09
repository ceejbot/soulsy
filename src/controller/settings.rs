use anyhow::Result;
use ini::Ini;
use once_cell::sync::OnceCell;

static SETTINGS_PATH: &str = "./data/MCM/Settings/SoulsyHUD.ini";

/// There can be only one. Not public because we want access managed.
static SETTINGS: OnceCell<Settings> = OnceCell::new();

/// User-modifiable settings for HUD behavior. Doesn't manage cycles.
///
/// These settings are read from an ini file managed by SkyUI's MCM, which provides
/// a UX for changing values. We are responsible for reading it, but do not need to
/// write it.
#[derive(Debug, Clone)]
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
    /// The maximum length of a cycle. Must be between 2 and 15, inclusive.
    pub maxlen: u32,
    /// The number of milliseconds to delay before equipping a selection. Max 2500, min 0.
    pub equip_delay: u32,
    /// Whether to fade out hud when not in combat.
    pub fade: bool,
    /// The number of milliseconds to delay before fading. Max 5000, min 0.
    pub fade_delay: u32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            left: 11,
            right: 13,
            power: 10,
            utility: 23,
            activate: Default::default(),
            showhide: Default::default(),
            maxlen: Default::default(),
            equip_delay: 500,
            fade: true,
            fade_delay: 1000,
        }
    }
}

/// The ini crate returns strings, which is reasonable because ini is barely a format.
/// So we have to parse them, with fallbacks if the value isn't found. We are doing our
/// very best to survive user error.
fn read_int_from(section: &ini::Properties, key: &str, default: u32) -> u32 {
    if let Some(str_val) = section.get(key) {
        str_val.parse::<u32>().unwrap_or(default)
    } else {
        default
    }
}

impl Settings {
    /// Read a settings object from a toml file.
    pub fn read_from_file() -> Result<Self> {
        let mut settings = Settings::default();
        // We'll fall back to defaults at a different level.
        let conf = Ini::load_from_file(SETTINGS_PATH)?;
        let empty = ini::Properties::new();

        // This is the sound of my brain going clonk. TODO rework.
        let controls = if let Some(s) = conf.section(Some("Controls")) {
            s
        } else {
            &empty
        };
        settings.left = read_int_from(controls, "uLeftCycleKey", settings.left);
        settings.right = read_int_from(controls, "uRightCycleKey", settings.right);
        settings.power = read_int_from(controls, "uPowerCycleKey", settings.power);
        settings.utility = read_int_from(controls, "uUtilityCycleKey", settings.utility);
        settings.activate = read_int_from(controls, "uUtilityActivateKey", settings.activate);
        settings.showhide = read_int_from(controls, "uShowHideKey", settings.showhide);

        let options = if let Some(s) = conf.section(Some("Options")) {
            s
        } else {
            &empty
        };
        settings.maxlen = read_int_from(options, "uMaxCycleLength", settings.maxlen);
        if settings.maxlen > 15 {
            settings.maxlen = 15;
        } else if settings.maxlen < 2 {
            settings.maxlen = 2;
        }
        settings.equip_delay = read_int_from(options, "uEquipDelay", settings.equip_delay);
        if settings.equip_delay > 2500 {
            settings.maxlen = 2500;
        }
        settings.fade_delay = read_int_from(options, "uFadeDelay", settings.fade_delay);
        if settings.fade_delay > 10000 {
            settings.maxlen = 10000;
        }
        settings.fade = if let Some(str_val) = options.get("bFade") {
            str_val != "0"
        } else {
            settings.fade
        };

        Ok(settings)
    }
}

/// Read our user settings from the file, or fall back to defaults if the file
/// is not present.
///
/// Lazily initialized on first request for the settings object. Will return all
/// 0s if somebody nuked the ini file.
pub fn settings() -> &'static Settings {
    if SETTINGS.get().is_none() {
        let settings = match Settings::read_from_file() {
            Ok(v) => v,
            Err(e) => {
                log::warn!("Failed to read settings file; continuing with defaults; {e:?}");
                Settings::default()
            }
        };
        SETTINGS.set(settings).unwrap();
    }

    // If this fails, the universe is in a bad state. Crashing is fine.
    SETTINGS.get().unwrap()
}

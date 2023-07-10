use std::sync::Mutex;

use anyhow::Result;
use ini::Ini;
use once_cell::sync::Lazy;

static SETTINGS_PATH: &str = "./data/MCM/Settings/SoulsyHUD.ini";

/// There can be only one. Not public because we want access managed.
static SETTINGS: Lazy<Mutex<UserSettings>> = Lazy::new(|| Mutex::new(UserSettings::default()));

/// We hand a read-only copy to C++ for use.
pub fn user_settings() -> Box<UserSettings> {
    let settings = SETTINGS.lock().unwrap();
    Box::new(settings.clone())
}

/// User-modifiable settings for HUD behavior. Doesn't manage cycles.
///
/// These settings are read from an ini file managed by SkyUI's MCM, which provides
/// a UX for changing values. We are responsible for reading it, but do not need to
/// write it.
#[derive(Debug, Clone)]
pub struct UserSettings {
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

impl Default for UserSettings {
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

impl UserSettings {
    pub fn refresh() -> Result<()> {
        let mut settings = SETTINGS.lock().unwrap();
        settings.read_from_file()
    }

    /// Refresh ourselves from the MCM-controlled file.
    pub fn read_from_file(&mut self) -> Result<()> {
        // We'll fall back to defaults at a different level.
        let conf = Ini::load_from_file(SETTINGS_PATH)?;
        let empty = ini::Properties::new();

        // This is the sound of my brain going clonk. TODO rework.
        let controls = if let Some(s) = conf.section(Some("Controls")) {
            s
        } else {
            &empty
        };
        self.left = read_int_from(controls, "uLeftCycleKey", self.left);
        self.right = read_int_from(controls, "uRightCycleKey", self.right);
        self.power = read_int_from(controls, "uPowerCycleKey", self.power);
        self.utility = read_int_from(controls, "uUtilityCycleKey", self.utility);
        self.activate = read_int_from(controls, "uUtilityActivateKey", self.activate);
        self.showhide = read_int_from(controls, "uShowHideKey", self.showhide);

        let options = if let Some(s) = conf.section(Some("Options")) {
            s
        } else {
            &empty
        };
        self.maxlen = read_int_from(options, "uMaxCycleLength", self.maxlen);
        if self.maxlen > 15 {
            self.maxlen = 15;
        } else if self.maxlen < 2 {
            self.maxlen = 2;
        }
        self.equip_delay = read_int_from(options, "uEquipDelay", self.equip_delay);
        if self.equip_delay > 2500 {
            self.maxlen = 2500;
        }
        self.fade_delay = read_int_from(options, "uFadeDelay", self.fade_delay);
        if self.fade_delay > 10000 {
            self.maxlen = 10000;
        }
        self.fade = if let Some(str_val) = options.get("bFade") {
            str_val != "0"
        } else {
            self.fade
        };

        Ok(())
    }

    pub fn is_cycle_button(&self, key: u32) -> bool {
        key == self.left || key == self.right || key == self.power || key == self.utility
    }

    pub fn left(&self) -> u32 {
        self.left
    }
    pub fn right(&self) -> u32 {
        self.right
    }
    pub fn power(&self) -> u32 {
        self.power
    }
    pub fn utility(&self) -> u32 {
        self.utility
    }
    pub fn activate(&self) -> u32 {
        self.activate
    }
    pub fn showhide(&self) -> u32 {
        self.showhide
    }
    pub fn maxlen(&self) -> u32 {
        self.maxlen
    }
    pub fn equip_delay(&self) -> u32 {
        self.equip_delay
    }
    pub fn fade(&self) -> bool {
        self.fade
    }
    pub fn fade_delay(&self) -> u32 {
        self.fade_delay
    }
}

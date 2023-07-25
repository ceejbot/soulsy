use std::sync::Mutex;

use anyhow::Result;
use ini::Ini;
use once_cell::sync::Lazy;

use crate::plugin::HudElement;

/// This is the path to players's modified settings.
static SETTINGS_PATH: &str = "./data/MCM/Settings/SoulsyHUD.ini";

/// This is the path to the mod settings definition file.
/// static INI_PATH: &str = "./data/MCM/Config/SoulsyHUD/settings.ini";

/// There can be only one. Not public because we want access managed.
static SETTINGS: Lazy<Mutex<UserSettings>> =
    Lazy::new(|| Mutex::new(UserSettings::new_from_file()));

/// We hand a read-only copy to C++ for use.
pub fn user_settings() -> Box<UserSettings> {
    let settings = SETTINGS
        .lock()
        .expect("Unrecoverable runtime problem: cannot acquire settings lock.");
    Box::new(settings.clone())
}

/// Wrapper for C++ convenience; logs errors but does no more
pub fn refresh_user_settings() {
    match UserSettings::refresh() {
        Ok(_) => {
            log::info!("refreshed user settings after MCM edits");
        }
        Err(e) => {
            log::warn!("failed to refresh user settings; using defaults; {e:?}");
        }
    }
}

/// User-modifiable settings for HUD behavior. Doesn't manage cycles.
///
/// These settings are read from an ini file managed by SkyUI's MCM, which provides
/// a UX for changing values. We are responsible for reading it, but do not need to
/// write it.
#[derive(Debug, Clone)]
pub struct UserSettings {
    /// An optional modifier key for all cycle hotkeys. E.g., shift + key.
    pub cycle_modifier: i32,
    /// An optional modifier key for unequipping a specific slot
    pub unequip_modifier: i32,
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
    /// An optional modifier key for activating the utility item.
    pub activate_modifier: i32,
    /// Show/hide shortcut key.
    pub showhide: u32,
    /// A hotkey for re-reading the layout from toml and redrawing.
    pub refresh_layout: u32,
    /// The maximum length of a cycle. Must be between 2 and 15, inclusive.
    pub maxlen: u32,
    /// The number of milliseconds to delay before equipping a selection. Max 2500, min 0.
    pub equip_delay: u32,
    /// Whether to fade out hud when not in combat.
    pub autofade: bool,
    /// The controller kind to show in the UX. Matches the controller_set enum in key_path.h
    pub controller_kind: u32, // 0 = pc, 1 = ps, 2 = xbox
    /// Whether to include unarmed as a cycle entry for each hand.
    pub include_unarmed: bool,
    /// Whether to slow down time when cycling
    pub cycling_slows_time: bool,
}

impl Default for UserSettings {
    fn default() -> Self {
        Self {
            cycle_modifier: -1,
            unequip_modifier: -1,
            // The map in key_path.h starts with numeral 1 => 2.
            left: 5,
            right: 7,
            power: 3,
            utility: 6,
            activate: 4,
            activate_modifier: -1,
            refresh_layout: 8,
            showhide: 2,
            maxlen: 10,       // this not a key code but an int
            equip_delay: 750, // in milliseconds
            autofade: true,
            controller_kind: 0, // PC
            include_unarmed: true,
            cycling_slows_time: false,
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

fn read_signed_int_from(section: &ini::Properties, key: &str, default: i32) -> i32 {
    if let Some(str_val) = section.get(key) {
        str_val.parse::<i32>().unwrap_or(default)
    } else {
        default
    }
}

impl UserSettings {
    pub fn new_from_file() -> Self {
        let mut s = UserSettings::default();
        s.read_from_file().unwrap_or_default();
        s
    }

    pub fn refresh() -> Result<()> {
        let mut settings = SETTINGS
            .lock()
            .expect("Unrecoverable runtime problem: cannot acquire settings lock.");
        settings.read_from_file()
    }

    /// Refresh ourselves from the MCM-controlled file.
    pub fn read_from_file(&mut self) -> Result<()> {
        // We'll fall back to defaults at a different level.
        let conf = Ini::load_from_file(SETTINGS_PATH)?;
        let empty = ini::Properties::new();

        // This is the sound of my brain going clonk.
        let controls = if let Some(s) = conf.section(Some("Controls")) {
            s
        } else {
            &empty
        };
        self.cycle_modifier =
            read_signed_int_from(controls, "iCycleModifierKey", self.cycle_modifier);
        self.unequip_modifier =
            read_signed_int_from(controls, "iUnequipModifierKey", self.unequip_modifier);
        self.left = read_int_from(controls, "uLeftCycleKey", self.left);
        self.right = read_int_from(controls, "uRightCycleKey", self.right);
        self.power = read_int_from(controls, "uPowerCycleKey", self.power);
        self.utility = read_int_from(controls, "uUtilityCycleKey", self.utility);
        self.activate = read_int_from(controls, "uUtilityActivateKey", self.activate);
        self.activate_modifier =
            read_signed_int_from(controls, "iUtilityActivateModifier", self.activate_modifier);
        self.showhide = read_int_from(controls, "uShowHideKey", self.showhide);
        self.refresh_layout = read_int_from(controls, "uRefreshKey", self.refresh_layout);

        let options = if let Some(s) = conf.section(Some("Options")) {
            s
        } else {
            &empty
        };
        self.maxlen = clamp(
            read_int_from(options, "uMaxCycleLength", self.maxlen),
            2,
            15,
        );
        self.equip_delay = clamp(
            read_int_from(options, "uEquipDelay", self.equip_delay),
            0,
            2500,
        );
        self.autofade = if let Some(str_val) = options.get("bAutoFade") {
            str_val != "0"
        } else {
            self.autofade
        };
        self.controller_kind = clamp(
            read_int_from(options, "uControllerKind", self.controller_kind),
            0,
            2,
        );
        self.include_unarmed = if let Some(str_val) = options.get("bIncludeUnarmed") {
            str_val != "0"
        } else {
            self.include_unarmed
        };
        self.cycling_slows_time = if let Some(str_val) = options.get("bCyclingSlowsTime") {
            str_val != "0"
        } else {
            self.cycling_slows_time
        };

        Ok(())
    }

    pub fn unequip_with_modifier(&self) -> bool {
        // hiding the implementation here, possibly pointlessly
        self.unequip_modifier > 0
    }

    pub fn is_unequip_modifier(&self, key: u32) -> bool {
        self.unequip_modifier as u32 == key
    }

    pub fn cycle_with_modifier(&self) -> bool {
        self.cycle_modifier > 0
    }

    pub fn is_cycle_modifier(&self, key: u32) -> bool {
        self.cycle_modifier as u32 == key
    }

    pub fn is_cycle_button(&self, key: u32) -> bool {
        key == self.left || key == self.right || key == self.power || key == self.utility
    }

    pub fn hotkey_for(&self, action: HudElement) -> u32 {
        match action {
            HudElement::Power => self.power,
            HudElement::Utility => self.utility,
            HudElement::Left => self.left,
            HudElement::Right => self.right,
            HudElement::Ammo => self.activate, // objectively wrong, but ignored
            _ => self.refresh_layout,          // programmer error; should be unreachable!()
        }
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
    pub fn activate_modifier(&self) -> i32 {
        self.activate_modifier
    }
    pub fn activate(&self) -> u32 {
        self.activate
    }
    pub fn showhide(&self) -> u32 {
        self.showhide
    }
    pub fn refresh_layout(&self) -> u32 {
        self.refresh_layout
    }
    pub fn maxlen(&self) -> u32 {
        clamp(self.maxlen, 2, 15)
    }
    pub fn equip_delay(&self) -> u32 {
        clamp(self.equip_delay, 100, 5000)
    }
    pub fn autofade(&self) -> bool {
        self.autofade
    }
    pub fn controller_kind(&self) -> u32 {
        clamp(self.controller_kind, 0, 2)
    }
    pub fn include_unarmed(&self) -> bool {
        self.include_unarmed
    }
    pub fn cycling_slows_time(&self) -> bool {
        self.cycling_slows_time
    }
}

fn clamp(num: u32, min: u32, max: u32) -> u32 {
    if num > max {
        max
    } else if num < min {
        min
    } else {
        num
    }
}

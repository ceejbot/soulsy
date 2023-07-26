use std::sync::Mutex;

use anyhow::Result;
use ini::Ini;
use once_cell::sync::Lazy;
use strum::Display;

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

#[derive(Debug, Clone, Display)]
pub enum ActivationMethod {
    Hotkey,
    LongPress,
    Modifier,
}

impl From<u32> for ActivationMethod {
    fn from(value: u32) -> Self {
        match value {
            0 => ActivationMethod::Hotkey,
            1 => ActivationMethod::LongPress,
            2 => ActivationMethod::Modifier,
            _ => ActivationMethod::Hotkey
        }
    }
}

#[derive(Debug, Clone, Display)]
pub enum UnarmedMethod {
    None,
    LongPress,
    Modifier,
    AddToCycles,
}

impl From<u32> for UnarmedMethod {
    fn from(value: u32) -> Self {
        match value {
            0 => UnarmedMethod::None,
            1 => UnarmedMethod::LongPress,
            2 => UnarmedMethod::Modifier,
            3 => UnarmedMethod::AddToCycles,
            _ => UnarmedMethod::None
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
    cycle_modifier: i32,
    /// When to apply the cycle modifier.
    cycle_mod_when: CycleModWhen,
    /// An optional modifier key for unequipping a specific slot
    unequip_modifier: i32,
    /// The key for the left hand's cycle.
    left: u32,
    /// The key for the right hand's cycle.
    right: u32,
    /// The key for powers.
    power: u32,
    /// The key for utility items.
    utility: u32,
    /// The key to activate or use a utility item.
    activate: u32,
    /// An optional modifier key for activating the utility item.
    activate_modifier: i32,
    /// Set to require activation with a long press
    activate_with_press: bool,
    /// Show/hide shortcut key.
    showhide: u32,
    /// A hotkey for re-reading the layout from toml and redrawing.
    refresh_layout: u32,
    /// The maximum length of a cycle. Must be between 2 and 15, inclusive.
    maxlen: u32,
    /// The number of milliseconds to delay before equipping a selection. Max 2500, min 0.
    equip_delay: u32,
    /// Whether to fade out hud when not in combat.
    autofade: bool,
    /// The time in milliseconds it takes to fade out.
    fade_time: u32,
    /// The controller kind to show in the UX. Matches the controller_set enum in key_path.h
    controller_kind: u32, // 0 = pc, 1 = ps, 2 = xbox
    /// Whether to include unarmed as a cycle entry for each hand.
    include_unarmed: bool,
    /// Whether to slow down time when cycling
    cycling_slows_time: bool,
    /// How much to slow down time.
    slow_time_factor: f32,
}

impl Default for UserSettings {
    fn default() -> Self {
        Self {
            cycle_modifier: -1,
            cycle_mod_when: CycleModWhen::Never,
            unequip_modifier: -1,
            // The map in key_path.h starts with numeral 1 => 2.
            left: 5,
            right: 7,
            power: 3,
            utility: 6,
            activate: 4,
            activate_modifier: -1,
            activate_with_press: false,
            refresh_layout: 8,
            showhide: 2,
            maxlen: 10,       // this not a key code but an int
            equip_delay: 750, // in milliseconds
            autofade: true,
            fade_time: 2000,    // in milliseconds
            controller_kind: 0, // PS5
            include_unarmed: true,
            cycling_slows_time: false,
            slow_time_factor: 0.25,
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

fn read_bool_from(section: &ini::Properties, key: &str, default: bool) -> bool {
    if let Some(str_val) = section.get(key) {
        str_val != "0"
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
        // And again, clonk.
        let options = if let Some(s) = conf.section(Some("Options")) {
            s
        } else {
            &empty
        };

        self.cycle_modifier =
            read_signed_int_from(controls, "iCycleModifierKey", self.cycle_modifier);
        self.cycle_mod_when = if let Some(str_val) = controls.get("uCycleModifierRequired") {
            CycleModWhen::from(str_val)
        } else {
            CycleModWhen::Never
        };

        self.left = read_int_from(controls, "uLeftCycleKey", self.left);
        self.right = read_int_from(controls, "uRightCycleKey", self.right);
        self.power = read_int_from(controls, "uPowerCycleKey", self.power);
        self.utility = read_int_from(controls, "uUtilityCycleKey", self.utility);

        self.activate = read_int_from(controls, "uUtilityActivateKey", self.activate);
        self.activate_modifier =
            read_signed_int_from(controls, "iUtilityActivateModifier", self.activate_modifier);
        self.activate_with_press =
            read_bool_from(controls, "bActivateLongPress", self.activate_with_press);

        self.showhide = read_int_from(controls, "uShowHideKey", self.showhide);
        self.refresh_layout = read_int_from(controls, "uRefreshKey", self.refresh_layout);

        self.unequip_modifier =
            read_signed_int_from(controls, "iUnequipModifierKey", self.unequip_modifier);


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

        self.autofade = read_bool_from(options, "bAutoFade", self.autofade);
        self.fade_time = clamp(read_int_from(options, "uFadeTime", self.fade_time), 0, 2500);

        self.controller_kind = clamp(
            read_int_from(options, "uControllerKind", self.controller_kind),
            0,
            2,
        );

        self.include_unarmed = read_bool_from(options, "bIncludeUnarmed", self.include_unarmed);

        self.cycling_slows_time =
            read_bool_from(options, "bCyclingSlowsTime", self.cycling_slows_time);
        let percentage = read_int_from(options, "uSlowTimeFactor", 25);
        self.slow_time_factor = percentage as f32 / 100.0;

        Ok(())
    }

    pub fn unequip_with_modifier(&self) -> bool {
        // hiding the implementation here, possibly pointlessly
        self.unequip_modifier > 0
    }
    pub fn is_unequip_modifier(&self, key: u32) -> bool {
        self.unequip_modifier as u32 == key
    }
    pub fn unequip_modifier(&self) -> i32 {
        self.unequip_modifier
    }

    // TODO rework
    pub fn cycle_mod_when(&self) -> &CycleModWhen {
        &self.cycle_mod_when
    }
    pub fn cycle_with_modifier(&self) -> bool {
        self.cycle_modifier > 0
    }
    pub fn cycle_modifier(&self) -> i32 {
        self.cycle_modifier
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
    pub fn activate_with_press(&self) -> bool {
        self.activate_with_press
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
    pub fn fade_time(&self) -> u32 {
        self.fade_time
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

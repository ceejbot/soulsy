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

/// User-modifiable settings for HUD behavior. Doesn't manage cycles.
///
/// These settings are read from an ini file managed by SkyUI's MCM, which provides
/// a UX for changing values. We are responsible for reading it, but do not need to
/// write it.
#[derive(Debug, Clone)]
pub struct UserSettings {
    /// The key for powers. uPowerCycleKey
    power: u32,
    /// The key for utility items. uUtilityCycleKey
    utility: u32,
    /// The key for the left hand's cycle. uLeftCycleKey
    left: u32,
    /// The key for the right hand's cycle. uRightCycleKey
    right: u32,

    /// How the player wants to use the utility item. uHowToActivate
    how_to_activate: ActivationMethod,
    /// The key to activate or use a utility item. uUtilityActivateKey
    activate: u32,
    /// An optional modifier key for activating the utility item. iUtilityActivateModifier
    activate_modifier: i32,

    /// How the player wants to advance a cycle. uHowToAdvance
    how_to_cycle: ActivationMethod,
    /// An optional modifier key for all cycle hotkeys. E.g., shift + key. iCycleModifierKey
    cycle_modifier: i32,

    /// How the player adds and removes items in menus. uHowTriggerInMenus
    how_to_toggle: ActivationMethod,
    /// Optional menu modifier key
    menu_modifier: i32,

    /// How the player wants to handle unequipping slots. uHowToUnequip
    unarmed_handling: UnarmedMethod,
    /// An optional modifier key for unequipping a specific slot. iUnequipModifierKey
    unequip_modifier: i32,

    /// Show/hide shortcut key. uShowHideKey
    showhide: u32,
    /// A hotkey for re-reading the layout from toml and redrawing. uRefreshKey
    refresh_layout: u32,
    /// The number of milliseconds to delay before equipping a selection. Max 2500, min 0.
    equip_delay_ms: u32,
    /// The number of milliseconds it takes for a press to be a long one.
    long_press_ms: u32,
    /// Whether to fade out hud when not in combat.
    autofade: bool,
    /// The time in milliseconds it takes to fade out.
    fade_time: u32,
    /// The controller kind to show in the UX. Matches the controller_set enum in key_path.h
    controller_kind: u32, // 0 = pc, 1 = ps, 2 = xbox
    /// Whether to slow down time when cycling
    cycling_slows_time: bool,
    /// How much to slow down time.
    slow_time_factor: f32,
}

impl Default for UserSettings {
    fn default() -> Self {
        Self {
            // The map in key_path.h starts with numeral 1 => 2.
            left: 5,
            right: 7,
            power: 3,
            utility: 6,
            how_to_activate: ActivationMethod::Hotkey,
            activate: 4,
            activate_modifier: -1,
            how_to_cycle: ActivationMethod::Hotkey,
            cycle_modifier: -1,
            how_to_toggle: ActivationMethod::Hotkey,
            menu_modifier: -1,
            unarmed_handling: UnarmedMethod::None,
            unequip_modifier: -1,
            refresh_layout: 8,
            showhide: 2,
            equip_delay_ms: 750, // in milliseconds
            long_press_ms: 1250, // in milliseconds
            autofade: true,
            fade_time: 2000,    // in milliseconds
            controller_kind: 0, // PS5
            cycling_slows_time: false,
            slow_time_factor: 0.25,
        }
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

        self.left = read_from_ini(self.left, "uLeftCycleKey", controls);
        self.right = read_from_ini(self.right, "uRightCycleKey", controls);
        self.power = read_from_ini(self.power, "uPowerCycleKey", controls);
        self.utility = read_from_ini(self.utility, "uUtilityCycleKey", controls);
        self.how_to_cycle = read_from_ini(self.how_to_cycle, "uHowToCycle", controls);
        self.cycle_modifier = read_from_ini(self.cycle_modifier, "iCycleModifierKey", controls);
        let old_mod_required = read_from_ini(false, "uCycleModifierRequired", controls);
        if old_mod_required && self.cycle_modifier != -1 {
            log::warn!("Using your old config option to require a mod key for cycling.");
            self.how_to_cycle = ActivationMethod::Modifier;
        }

        self.how_to_toggle = read_from_ini(self.how_to_toggle, "uHowToggleInMenus", controls);
        self.menu_modifier = read_from_ini(self.menu_modifier, "iMenuModifierKey", controls);

        self.how_to_activate = read_from_ini(self.how_to_activate, "uHowToActivate", controls);
        self.activate = read_from_ini(self.activate, "uUtilityActivateKey", controls);
        self.activate_modifier =
            read_from_ini(self.activate_modifier, "iUtilityActivateModifier", controls);

        self.showhide = read_from_ini(self.showhide, "uShowHideKey", controls);
        self.refresh_layout = read_from_ini(self.refresh_layout, "uRefreshKey", controls);

        self.unarmed_handling = read_from_ini(self.unarmed_handling, "uHowToUnequip", controls);
        self.unequip_modifier =
            read_from_ini(self.unequip_modifier, "iUnequipModifierKey", controls);
        let old_include_unarmed = read_from_ini(false, "bIncludeUnarmed", options);
        if old_include_unarmed && matches!(self.unarmed_handling, UnarmedMethod::None) {
            log::warn!("Using your old config option and adding unarmed to cycles.");
            self.unarmed_handling = UnarmedMethod::AddToCycles;
        } else if self.unequip_modifier != -1
            && matches!(self.unarmed_handling, UnarmedMethod::None)
        {
            log::warn!("Using your old config option and requiring a modifier key for un-equipping a hand.");
            self.unarmed_handling = UnarmedMethod::AddToCycles;
        }

        self.equip_delay_ms = clamp(
            read_from_ini(self.equip_delay_ms, "uEquipDelay", options),
            0,
            2500,
        );
        self.long_press_ms = clamp(
            read_from_ini(self.equip_delay_ms, "uLongPressMillis", options),
            self.equip_delay_ms + 100,
            2500,
        );

        self.autofade = read_from_ini(self.autofade, "bAutoFade", options);
        self.fade_time = clamp(read_from_ini(self.fade_time, "uFadeTime", options), 0, 2500);
        self.controller_kind = clamp(
            read_from_ini(self.controller_kind, "uControllerKind", options),
            0,
            1,
        );

        self.cycling_slows_time =
            read_from_ini(self.cycling_slows_time, "bCyclingSlowsTime", options);
        let percentage = read_from_ini(25, "uSlowTimeFactor", options);
        self.slow_time_factor = percentage as f32 / 100.0;

        Ok(())
    }

    pub fn unequip_with_modifier(&self) -> bool {
        // hiding the implementation here, possibly pointlessly
        self.unequip_modifier > 0
    }
    pub fn unarmed_handling(&self) -> &UnarmedMethod {
        &self.unarmed_handling
    }
    pub fn is_unequip_modifier(&self, key: u32) -> bool {
        self.unequip_modifier as u32 == key
    }
    pub fn unequip_modifier(&self) -> i32 {
        self.unequip_modifier
    }

    pub fn how_to_toggle(&self) -> &ActivationMethod {
        &self.how_to_toggle
    }
    pub fn menu_modifier(&self) -> i32 {
        self.menu_modifier
    }

    pub fn how_to_cycle(&self) -> &ActivationMethod {
        &self.how_to_cycle
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
            _ => self.refresh_layout,          // required because this is a C-style enum
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

    pub fn how_to_activate(&self) -> &ActivationMethod {
        &self.how_to_activate
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
        20
    }
    pub fn equip_delay_ms(&self) -> u32 {
        self.equip_delay_ms
    }
    pub fn long_press_ms(&self) -> u32 {
        self.long_press_ms
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
    pub fn cycling_slows_time(&self) -> bool {
        self.cycling_slows_time
    }
    pub fn slow_time_factor(&self) -> f32 {
        self.slow_time_factor
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

/// General-purpose enum for how to activate things.
#[derive(Debug, Clone, Display, Copy)]
pub enum ActivationMethod {
    /// Tap the hotkey.
    Hotkey,
    /// Long-press the hotkey.
    LongPress,
    /// Use a modifier plus the hotkey.
    Modifier,
}

// Trait and implementations for reading from the ini file

fn read_from_ini<'a, T: FromIniStr>(default: T, key: &str, section: &ini::Properties) -> T {
    if let Some(str_val) = section.get(key) {
        if let Some(v) = T::from_ini(&str_val) {
            v
        } else {
            default
        }
    } else {
        default
    }
}

trait FromIniStr {
    fn from_ini(value: &str) -> Option<Self>
    where
        Self: Sized;
}

impl FromIniStr for ActivationMethod {
    fn from_ini(value: &str) -> Option<Self> {
        match value {
            "0" => Some(ActivationMethod::Hotkey),
            "1" => Some(ActivationMethod::LongPress),
            "2" => Some(ActivationMethod::Modifier),
            _ => None,
        }
    }
}

/// How the player wants to handle unarmed combat.
#[derive(Debug, Clone, Display, Copy)]
pub enum UnarmedMethod {
    /// No support from the HUD.
    None,
    /// Long-press a cycle key to unequip.
    LongPress,
    /// Use a modifier plus a cycle key to unequip.
    Modifier,
    /// Add unarmed combat to the slots for left and right hand.
    AddToCycles,
}

impl FromIniStr for UnarmedMethod {
    fn from_ini(value: &str) -> Option<Self> {
        match value {
            "0" => Some(UnarmedMethod::None),
            "1" => Some(UnarmedMethod::LongPress),
            "2" => Some(UnarmedMethod::Modifier),
            "3" => Some(UnarmedMethod::AddToCycles),
            _ => None,
        }
    }
}

impl FromIniStr for bool {
    fn from_ini(value: &str) -> Option<Self> {
        Some(value != "0")
    }
}

impl FromIniStr for u32 {
    fn from_ini(value: &str) -> Option<Self> {
        if let Ok(v) = value.parse::<u32>() {
            Some(v)
        } else {
            None
        }
    }
}

impl FromIniStr for i32 {
    fn from_ini(value: &str) -> Option<Self> {
        if let Ok(v) = value.parse::<i32>() {
            Some(v)
        } else {
            None
        }
    }
}

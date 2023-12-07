//! User-controlled settings. Reads the INI file written by MCM.
//! Provides an interface for all settings data to C++ and Rust both.
//! Does some data validation and some translation from older versions,
//! but this file is otherwise all fairly predictable.

use std::sync::Mutex;

use eyre::Result;
use ini::Ini;
use log::Level;
use once_cell::sync::Lazy;
use strum::Display;

use super::keys::Hotkey;
use crate::plugin::HudElement;

/// This is the path to players's modified settings.
static SETTINGS_PATH: &str = "./data/MCM/Settings/SoulsyHUD.ini";

/// This is the path to the mod settings definition file.
/// static INI_PATH: &str = "./data/MCM/Config/SoulsyHUD/settings.ini";

/// There can be only one. Not public because we want access managed.
static SETTINGS: Lazy<Mutex<UserSettings>> =
    Lazy::new(|| Mutex::new(UserSettings::new_from_file(SETTINGS_PATH)));

pub fn settings() -> UserSettings {
    let settings = SETTINGS
        .lock()
        .expect("Unrecoverable runtime problem: cannot acquire settings lock.");
    settings.clone()
}

/// Wrapper for C++ convenience; logs errors but does no more
pub fn refresh_user_settings() {
    match UserSettings::refresh() {
        Ok(_) => {
            log::info!("Refreshed user settings after MCM edits.");
        }
        Err(e) => {
            log::warn!("Failed to refresh user settings; using defaults; {e:#}");
        }
    }
}

/// User-modifiable settings for HUD behavior. Doesn't manage cycles.
///
/// These settings are read from an ini file managed by SkyUI's MCM, which provides
/// a UX for changing values. We are responsible for reading it, but do not need to
/// write it. We only ever hand out clones of this object to enforce the idea that
/// it's read-only.
#[derive(Debug, Clone)]
pub struct UserSettings {
    /// Desired log level. `sLogLevel`
    log_level: Level,

    /// The key for powers. uPowerCycleKey
    power: u32,
    /// The key for utility items. uUtilityCycleKey
    utility: u32,
    /// The key for the left hand's cycle. uLeftCycleKey
    left: u32,
    /// The key for the right hand's cycle. uRightCycleKey
    right: u32,
    /// The key for equip sets. iEquipSetCycleKey
    equipset: i32,

    /// How the player wants to use the utility item. uHowToActivate
    how_to_activate: ActivationMethod,
    /// The key to activate or use a utility item. uUtilityActivateKey
    activate: u32,
    /// An optional modifier key for activating the utility item. iUtilityActivateModifier
    activate_modifier: i32,
    /// If magicka, health, and stamina potions are grouped and auto-selected.
    group_potions: bool,

    /// How the player wants to advance a cycle. uHowToAdvance
    how_to_cycle: ActivationMethod,
    /// An optional modifier key for all cycle hotkeys. E.g., shift + key. iCycleModifierKey
    cycle_modifier: i32,

    /// How the player adds and removes items in menus. uHowTriggerInMenus
    how_to_toggle: ActivationMethod,
    /// Optional menu modifier key
    menu_modifier: i32,
    /// Favoriting weapons and spells adds to cycles.
    link_to_favorites: bool,

    /// How the player wants to handle unequipping slots. uHowToUnequip
    unarmed_handling: UnarmedMethod,
    /// An optional modifier key for unequipping a specific slot. iUnequipModifierKey
    unequip_modifier: i32,

    /// Matching left and right hands. bLongPressMatches
    long_press_matches: bool,

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
    /// True if the player wants us to cycle through ammo.
    cycle_ammo: bool,
    /// True if icons should be drawn in living color.
    colorize_icons: bool,
    /// Equip sets unequip. bEquipSetsUnequip
    equip_sets_unequip: bool,
    /// The identifier for the mod in SKSE cosaves. Defaults to SOLS.
    skse_identifier: String,
}

impl Default for UserSettings {
    fn default() -> Self {
        Self {
            log_level: Level::Info,
            // The map in key_path.h starts with numeral 1 => 2.
            showhide: 2,
            power: 3,
            left: 5,
            utility: 6,
            right: 7,
            equipset: 9,
            refresh_layout: 8,
            how_to_activate: ActivationMethod::Hotkey,
            activate: 4,
            activate_modifier: -1,
            group_potions: false,
            how_to_cycle: ActivationMethod::Hotkey,
            cycle_modifier: -1,
            long_press_matches: false,
            how_to_toggle: ActivationMethod::Hotkey,
            menu_modifier: -1,
            link_to_favorites: false,
            unarmed_handling: UnarmedMethod::None,
            unequip_modifier: -1,
            equip_delay_ms: 750, // in milliseconds
            long_press_ms: 1250, // in milliseconds
            autofade: true,
            fade_time: 2000,    // in milliseconds
            controller_kind: 0, // PS5
            cycling_slows_time: false,
            slow_time_factor: 0.25,
            cycle_ammo: true,
            colorize_icons: true,
            equip_sets_unequip: true,
            skse_identifier: "SOLS".to_string(),
        }
    }
}

impl UserSettings {
    pub fn new_from_file(fpath: &str) -> Self {
        let mut s = UserSettings::default();
        s.read_from_file(fpath).unwrap_or_default();
        s
    }

    pub fn refresh() -> Result<()> {
        let mut settings = SETTINGS
            .lock()
            .expect("Unrecoverable runtime problem: cannot acquire settings lock.");
        settings.read_from_file(SETTINGS_PATH)
    }

    /// Refresh ourselves from the MCM-controlled file.
    pub fn read_from_file(&mut self, fpath: &str) -> Result<()> {
        // We'll fall back to defaults at a different level.
        let conf = Ini::load_from_file(fpath)?;
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

        self.log_level = read_from_ini(self.log_level, "sLogLevel", options);
        let debug = read_from_ini(false, "bDebugMode", options);
        // Allow the player toggle setting to function while also letting me set a level.
        if debug && self.log_level > Level::Debug {
            self.log_level = Level::Debug;
        }

        self.left = read_from_ini(self.left, "uLeftCycleKey", controls);
        self.right = read_from_ini(self.right, "uRightCycleKey", controls);
        self.power = read_from_ini(self.power, "uPowerCycleKey", controls);
        self.utility = read_from_ini(self.utility, "uUtilityCycleKey", controls);
        self.how_to_cycle = read_from_ini(self.how_to_cycle, "uHowToCycle", controls);
        self.cycle_modifier = read_from_ini(self.cycle_modifier, "iCycleModifierKey", controls);
        self.long_press_matches =
            read_from_ini(self.long_press_matches, "bLongPressMatches", controls);

        self.how_to_toggle = read_from_ini(self.how_to_toggle, "uHowToggleInMenus", controls);
        self.menu_modifier = read_from_ini(self.menu_modifier, "iMenuModifierKey", controls);
        self.link_to_favorites = read_from_ini(self.link_to_favorites, "bLinkToFavorites", options);

        self.how_to_activate = read_from_ini(self.how_to_activate, "uHowToActivate", controls);
        self.activate = read_from_ini(self.activate, "uUtilityActivateKey", controls);
        self.activate_modifier =
            read_from_ini(self.activate_modifier, "iUtilityActivateModifier", controls);
        self.group_potions = read_from_ini(self.group_potions, "bGroupPotions", options);

        self.showhide = read_from_ini(self.showhide, "uShowHideKey", controls);
        self.refresh_layout = read_from_ini(self.refresh_layout, "uRefreshKey", controls);

        self.unarmed_handling = read_from_ini(self.unarmed_handling, "uHowToUnequip", controls);
        self.unequip_modifier =
            read_from_ini(self.unequip_modifier, "iUnequipModifierKey", controls);

        self.equip_delay_ms = u32::clamp(
            read_from_ini(self.equip_delay_ms, "uEquipDelay", options),
            0,
            2500,
        );
        self.long_press_ms = read_from_ini(self.equip_delay_ms, "uLongPressMillis", options);
        if self.long_press_ms < self.equip_delay_ms {
            self.long_press_ms = self.equip_delay_ms + 100;
        }

        self.autofade = read_from_ini(self.autofade, "bAutoFade", options);
        self.fade_time = u32::clamp(read_from_ini(self.fade_time, "uFadeTime", options), 0, 2500);
        self.controller_kind = u32::clamp(
            read_from_ini(self.controller_kind, "uControllerKind", options),
            0,
            1,
        );

        self.cycling_slows_time =
            read_from_ini(self.cycling_slows_time, "bCyclingSlowsTime", options);
        let percentage = read_from_ini(25, "uSlowTimeFactor", options);
        self.slow_time_factor = percentage as f32 / 100.0;

        self.cycle_ammo = read_from_ini(self.cycle_ammo, "bCycleAmmo", options);
        self.colorize_icons = read_from_ini(self.colorize_icons, "bColorizeIcons", options);
        self.skse_identifier =
            read_from_ini(self.skse_identifier.clone(), "sSKSEIdentifier", options);

        self.equipset = read_from_ini(self.equipset, "iEquipSetCycleKey", controls);
        self.equip_sets_unequip =
            read_from_ini(self.equip_sets_unequip, "bEquipSetsUnequip", options);

        Ok(())
    }

    pub fn log_level(&self) -> Level {
        self.log_level
    }

    pub fn log_level_number(&self) -> u32 {
        // See #defines in spdlog/include/common.h
        match self.log_level {
            Level::Error => 4,
            Level::Warn => 3,
            Level::Info => 2,
            Level::Debug => 1,
            Level::Trace => 0,
        }
    }

    pub fn unequip_with_modifier(&self) -> bool {
        // hiding the implementation here, possibly pointlessly
        self.unequip_modifier > 0
    }
    pub fn unequip_method(&self) -> &UnarmedMethod {
        &self.unarmed_handling
    }
    pub fn unequip_modifier(&self) -> i32 {
        self.unequip_modifier
    }

    pub fn start_long_press_timer(&self, key: &Hotkey) -> bool {
        let is_hand_cycle = matches!(key, Hotkey::Left | Hotkey::Right);
        let can_be_unequipped = matches!(key, Hotkey::Left | Hotkey::Power | Hotkey::Right);

        // These three should be mutually exclusive, so order shouldn't matter.
        // "should" ha ha ha
        if self.long_press_to_dual_wield() && is_hand_cycle {
            return true;
        }
        if matches!(self.how_to_activate, ActivationMethod::LongPress)
            && matches!(
                key,
                Hotkey::Left | Hotkey::Power | Hotkey::Right | Hotkey::Utility
            )
        {
            return true;
        }
        if matches!(self.unarmed_handling, UnarmedMethod::LongPress) && can_be_unequipped {
            return true;
        }

        false
    }

    pub fn how_to_toggle(&self) -> &ActivationMethod {
        &self.how_to_toggle
    }
    pub fn menu_modifier(&self) -> i32 {
        self.menu_modifier
    }
    pub fn link_to_favorites(&self) -> bool {
        self.link_to_favorites
    }

    pub fn cycle_advance_method(&self) -> &ActivationMethod {
        &self.how_to_cycle
    }
    pub fn cycle_with_modifier(&self) -> bool {
        self.cycle_modifier > 0
    }

    pub fn cycle_modifier(&self) -> i32 {
        self.cycle_modifier
    }

    pub fn long_press_to_dual_wield(&self) -> bool {
        self.long_press_matches
    }

    pub fn hotkey_for(&self, action: HudElement) -> u32 {
        match action {
            HudElement::Power => self.power,
            HudElement::Utility => self.utility,
            HudElement::Left => self.left,
            HudElement::Right => self.right,
            HudElement::Ammo => self.left, // This is objectively correct.
            HudElement::EquipSet => self.equipset as u32,
            _ => self.refresh_layout, // Required because this is a C-style enum.
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
    pub fn equipset(&self) -> i32 {
        self.equipset
    }
    pub fn equip_sets_unequip(&self) -> bool {
        self.equip_sets_unequip
    }

    pub fn utility_activation_method(&self) -> &ActivationMethod {
        &self.how_to_activate
    }
    pub fn activate_modifier(&self) -> i32 {
        self.activate_modifier
    }
    pub fn activate(&self) -> u32 {
        self.activate
    }

    pub fn group_potions(&self) -> bool {
        self.group_potions
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
        u32::clamp(self.controller_kind, 0, 2)
    }
    pub fn cycling_slows_time(&self) -> bool {
        self.cycling_slows_time
    }
    pub fn slow_time_factor(&self) -> f32 {
        self.slow_time_factor
    }

    pub fn cycle_ammo(&self) -> bool {
        self.cycle_ammo
    }

    pub fn colorize_icons(&self) -> bool {
        self.colorize_icons
    }

    pub fn skse_identifier(&self) -> u32 {
        let exactly_four = format!("{:4}", self.skse_identifier);
        let slice: [u8; 4] = exactly_four
            .as_bytes()
            .try_into()
            .expect("You must provide exactly four characters as the mod identifier string.");
        u32::from_le_bytes(slice)
    }
}

/// General-purpose enum for how to activate things.
#[derive(Debug, Clone, strum::Display, Copy)]
pub enum ActivationMethod {
    /// Tap the hotkey.
    Hotkey,
    /// Long-press the hotkey.
    LongPress,
    /// Use a modifier plus the hotkey.
    Modifier,
}

// Trait and implementations for reading from the ini file

fn read_from_ini<T: FromIniStr>(default: T, key: &str, section: &ini::Properties) -> T {
    if let Some(str_val) = section.get(key) {
        if let Some(v) = T::from_ini(str_val) {
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

impl FromIniStr for Level {
    fn from_ini(value: &str) -> Option<Self>
    where
        Self: Sized,
    {
        match value.parse::<Level>() {
            Ok(v) => Some(v),
            Err(e) => {
                // This is an error parsing a log level string. We might not manage
                // to log this error if it's at startup, but we try anyway.
                log::warn!("Error parsing log level string: {e:#}");
                Some(Level::Info)
            }
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
        Some(value != "0" && value.to_lowercase() != "false")
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

impl FromIniStr for f32 {
    fn from_ini(value: &str) -> Option<Self> {
        if let Ok(v) = value.parse::<f32>() {
            Some(v)
        } else {
            None
        }
    }
}
impl FromIniStr for String {
    fn from_ini(value: &str) -> Option<Self> {
        Some(value.to_string())
    }
}

// qualified so we don't collide with the macro strum::Display
// We implement this so the logs contain a human-readable dump of the settings
// at save game load, so people can debug.
impl std::fmt::Display for UserSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"   log level: {}
           show/hide HUD key: {}
             power cycle key: {}
           utility cycle key: {}
              left cycle key: {}
             right cycle key: {}
          equipset cycle key: {}
          refresh layout key: {}
             how_to_activate: {}
        activate consumables: {}
           activate_modifier: {}
               group_potions: {}
                how_to_cycle: {}
              cycle_modifier: {}
    dual-wield on long press: {}
               how_to_toggle: {}
               menu_modifier: {}
           link_to_favorites: {}
            unarmed_handling: {}
            unequip_modifier: {}
              equip_delay_ms: {} ms
               long_press_ms: {} ms
                    autofade: {}
                   fade_time: {} ms
             controller_kind: {}
          cycling_slows_time: {}
            slow_time_factor: {} %
                  cycle_ammo: {}
              colorize_icons: {}
          equip_sets_unequip: {}
             skse_identifier: {}"#,
            self.log_level,
            self.showhide,
            self.power,
            self.utility,
            self.left,
            self.right,
            self.equipset,
            self.refresh_layout,
            self.how_to_activate,
            self.activate,
            self.activate_modifier,
            self.group_potions,
            self.how_to_cycle,
            self.cycle_modifier,
            self.long_press_matches,
            self.how_to_toggle,
            self.menu_modifier,
            self.link_to_favorites,
            self.unarmed_handling,
            self.unequip_modifier,
            self.equip_delay_ms,
            self.long_press_ms,
            self.autofade,
            self.fade_time,
            self.controller_kind,
            self.cycling_slows_time,
            self.slow_time_factor,
            self.cycle_ammo,
            self.colorize_icons,
            self.equip_sets_unequip,
            self.skse_identifier
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::approx_constant)]
    fn ini_reader_trait_works() {
        let conf = Ini::load_from_file("./tests/fixtures/all-types.ini")
            .expect("ini fixture file should be readable");
        let section = conf
            .section(Some("section"))
            .expect("the text fixture has a section named 'section'");
        let u_unsigned_int: u32 = read_from_ini(0, "uUnsignedInt", section);
        assert_eq!(u_unsigned_int, 2);
        let u_unsigned_empty: u32 = read_from_ini(100, "uUnsignedEmpty", section);
        assert_eq!(u_unsigned_empty, 100);
        let i_signed_int: i32 = read_from_ini(-100, "iSignedInt", section);
        assert_eq!(i_signed_int, -2);
        let i_signed_empty: i32 = read_from_ini(-100, "iSignedEmpty", section);
        assert_eq!(i_signed_empty, -100);

        // This is why I need the lint allow above. Ha.
        let f_float: f32 = read_from_ini(0.0f32, "fFloat", section);
        assert_eq!(f_float, 2.71828);

        let f_float_empty: f32 = read_from_ini(1.0f32, "fFloatEmpty", section);
        assert_eq!(f_float_empty, 1.0);
        let b_boolean_num_t: bool = read_from_ini(false, "bBooleanNumT", section);
        assert!(b_boolean_num_t);
        let b_boolean_string_t: bool = read_from_ini(false, "bBooleanStringT", section);
        assert!(b_boolean_string_t);
        let b_boolean_num_f: bool = read_from_ini(true, "bBooleanNumF", section);
        assert!(!b_boolean_num_f);
        let b_boolean_string_f: bool = read_from_ini(true, "bBooleanStringF", section);
        assert!(!b_boolean_string_f);
        let b_boolean_empty: bool = read_from_ini(true, "bBooleanEmpty", section);
        assert!(b_boolean_empty);
        let s_string: String = read_from_ini("default".to_string(), "sString", section);
        assert_eq!(s_string.as_str(), "String with spaces");
        let s_string_empty: String = read_from_ini("default".to_string(), "sStringEmpty", section);
        assert_eq!(s_string_empty.as_str(), "");

        let missing_field: String = read_from_ini("default".to_string(), "missing_field", section);
        assert_eq!(missing_field.as_str(), "default");
    }

    #[test]
    fn can_read_example_ini() {
        let le_options = UserSettings::new_from_file("./tests/fixtures/SoulsyHUD.ini");
        assert!(le_options.long_press_ms > le_options.equip_delay_ms);
    }
}

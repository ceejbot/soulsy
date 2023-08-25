//! This module bundles up the public-facing interface of the controller for ease
//! of import into the bridge. It should have as little logic as is compatible
//! with keeping opaque Rust types from having to be exposed to C++.

use std::ffi::OsString;
use std::fs::File;
#[cfg(target_os = "windows")]
use std::os::windows::ffi::OsStringExt;
use std::path::Path;

use cxx::CxxVector;
use simplelog::*;

use super::cycles::*;
use super::settings::{user_settings, UserSettings};
use crate::data::*;
use crate::plugin::*;
use crate::{control, hud_layout};

// ---------- logging

pub fn initialize_rust_logging(logdir: &cxx::CxxVector<u16>) {
    let hudl = hud_layout(); // yeah, it's in here, sorry.
    let log_level = if hudl.debug {
        LevelFilter::Trace
    } else {
        LevelFilter::Info
    };

    #[cfg(any(target_os = "macos", target_os = "unix"))]
    let chonky_path = OsString::from("placeholder");
    #[cfg(target_os = "windows")]
    let chonky_path = OsString::from_wide(logdir.as_slice());
    let path = Path::new(chonky_path.as_os_str()).with_file_name("SoulsyHUD_rust.log");

    if let Ok(logfile) = File::create(path) {
        let _ = WriteLogger::init(log_level, Config::default(), logfile);
        log::info!("Rust side standing by.");
    } else {
        // Welp, we failed and I have nowhere to write the darn error. Ha ha.
    }
}

// ---------- the controller itself

/// Let's get this party started.
pub fn initialize_hud() {
    let settings = user_settings();
    log::info!("Reading and applying settings.");
    let mut ctrl = control::get();
    log::info!("{settings:?}");

    let _hud = hud_layout();
    ctrl.apply_settings();
    if !ctrl.cycles.loaded {
        log::info!("Cosave data not yet loaded.");
        ctrl.cycles = CycleData::default();
    }
}

/// Function for C++ to call to send a relevant button event to us.
pub fn handle_key_event(key: u32, button: &ButtonEvent) -> KeyEventResponse {
    control::get().handle_key_event(key, button)
}

pub fn show_ui() -> bool {
    control::get().cycles.hud_visible()
}

/// Function for C++ to call to send a relevant menu button-event to us.
///
/// We get a fully-filled out HudItem struct to use as we see fit.
// menu_item is boxed because it's arriving from C++.
#[allow(clippy::boxed_local)]
pub fn toggle_item(key: u32, #[allow(clippy::boxed_local)] menu_item: Box<HudItem>) {
    let action = Action::from(key);
    control::get().handle_toggle_item(action, *menu_item)
}

pub fn handle_menu_event(key: u32, button: &ButtonEvent) -> bool {
    control::get().handle_menu_event(key, button)
}

/// Get information about the item equipped in a specific slot.
pub fn entry_to_show_in_slot(element: HudElement) -> Box<HudItem> {
    control::get().entry_to_show_in_slot(element)
}

// Handle an equip delay timer expiring.
pub fn timer_expired(slot: Action) {
    control::get().timer_expired(slot);
}

/// Update our view of the player's equipment.
pub fn update_hud() -> bool {
    control::get().update_hud()
}

/// We know for sure the player just equipped this item.
pub fn handle_item_equipped(equipped: bool, form_spec: &String, right: bool, left: bool) -> bool {
    control::get().handle_item_equipped(equipped, form_spec, right, left)
}

/// A consumable's count changed. Record if relevant.
pub fn handle_inventory_changed(form_spec: &String, count: i32) {
    control::get().handle_inventory_changed(form_spec, count);
}

pub fn handle_favorite_event(
    button: &ButtonEvent,
    is_favorite: bool,
    #[allow(clippy::boxed_local)] item: Box<HudItem>, // needed to bridge with C++
) {
    control::get().handle_favorite_event(button, is_favorite, *item);
}

pub fn refresh_user_settings() {
    if let Some(e) = UserSettings::refresh().err() {
        log::warn!("Failed to read user settings! using defaults; {e:?}");
        return;
    }
    control::get().apply_settings();
}

pub fn clear_cycles() {
    control::get().clear_cycles();
}

pub fn get_cycle_names(which: u32) -> Vec<String> {
    control::get().cycle_names(which)
}

pub fn serialize_version() -> u32 {
    CycleData::serialize_version()
}

/// Serialize cycles for cosave.
pub fn serialize_cycles() -> Vec<u8> {
    control::get().cycles.serialize()
}

/// Cycle data loaded from cosave.
pub fn cycle_loaded_from_cosave(bytes: &CxxVector<u8>, version: u32) {
    let mut ctrl = control::get();
    if let Some(cosave_cycle) = CycleData::deserialize(bytes, version) {
        ctrl.cycles = cosave_cycle;
        ctrl.validate_cycles();
        log::info!("Cycles loaded and ready to rock.");
    } else {
        log::warn!("Cosave load failed. Defaulting to fresh start.");
        ctrl.cycles = CycleData::default();
    }
}

pub fn clear_cache() {
    control::get().cache.clear();
}

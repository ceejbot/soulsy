//! This module bundles up the public-facing interface of the controller for
//! ease of import into the bridge. It's the Rust equivalent of the helpers
//! class over on the C++ side. It can directly implement any logic that doesn't
//! demand the controller. In particular, it implements some support for
//! papyrus functions.

use cxx::CxxVector;

use super::cycles::*;
use super::settings::{settings, UserSettings};
use crate::control;
use crate::data::*;
use crate::layouts::{hud_layout, Layout};
use crate::plugin::*;

// ---------- boxed user settings

pub fn user_settings() -> Box<UserSettings> {
    Box::new(settings())
}

// ---------- the controller itself

/// Let's get this party started.
pub fn initialize_hud() {
    refresh_user_settings();
    let settings = settings();
    log::info!("Reading and applying settings. Your settings are:");
    log::info!("{settings}");

    Layout::refresh();
    let hud = hud_layout();
    let mut ctrl = control::get();

    if settings.autofade() {
        log::info!("The HUD is in autofade mode and ready to go.");
    } else {
        log::info!(
            "The HUD is in toggle mode and ready to go. Currently visible: {}",
            ctrl.cycles.hud_visible()
        );
    }
    log::info!("HUD location is: x={}; y={};", hud.anchor.x, hud.anchor.y);
}

/// Function for C++ to call to send a relevant button event to us.
pub fn handle_key_event(key: u32, button: &ButtonEvent) -> KeyEventResponse {
    control::get().handle_key_event(key, button)
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

/// We know for sure the player just equipped this item.
pub fn handle_item_equipped(
    equipped: bool,
    form_spec: &String,
    right: &String,
    left: &String,
) -> bool {
    control::get().handle_item_equipped(equipped, form_spec, right, left)
}

pub fn handle_grip_change(use_alt_grip: bool) {
    control::get().handle_grip_change(use_alt_grip);
}

/// A consumable's count changed. Record if relevant.
pub fn handle_inventory_changed(form_spec: &String, count: u32) {
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
        log::warn!("Failed to read user settings! using defaults; {e:#}");
        return;
    }
    control::get().apply_settings();
}

pub fn clear_cycles() {
    control::get().clear_cycles();
}

pub fn get_cycle_names(which: i32) -> Vec<String> {
    control::get().cycle_names(which)
}

pub fn get_cycle_formids(which: i32) -> Vec<String> {
    control::get().cycle_formids(which)
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
    refresh_user_settings();
    let mut ctrl = control::get();
    if let Some(cosave_cycle) = CycleData::deserialize(bytes, version) {
        ctrl.cycles = cosave_cycle;
        ctrl.refresh_after_load();
        log::info!("Cycles loaded and ready to rock.");
    } else {
        log::warn!("Cosave load failed. Defaulting to fresh start. Is your save corrupt?");
    }
}

pub fn clear_cache() {
    control::get().cache.clear();
}

/// This is straight-up papyrus support. We choose to return -1 to signal
/// failure because our use case is as array indexes in papyrus.
pub fn string_to_int(number: String) -> i32 {
    if let Ok(parsed) = number.parse::<i32>() {
        parsed
    } else {
        -1
    }
}

/// Equipment set functions for papyrus start here.
pub fn equipset_index_to_id(idx: String) -> i32 {
    // really I could get away with u8 here. just so long as it's smaller than an i32
    let Ok(parsed) = idx.parse::<u16>() else {
        return -1;
    };
    let ids = control::get().cycles.equipset_ids();
    if parsed as usize >= ids.len() {
        return -1;
    }
    if let Some(id) = ids.get(parsed as usize) {
        *id as i32
    } else {
        -1
    }
}

pub fn get_equipset_names() -> Vec<String> {
    control::get().cycles.equipset_names()
}

pub fn get_equipset_ids() -> Vec<String> {
    control::get()
        .cycles
        .equipset_ids()
        .iter()
        .map(|xs| xs.to_string())
        .collect()
}

pub fn handle_create_equipset(name: String) -> bool {
    let data = getEquippedItems();
    control::get().cycles.add_equipset(name, *data)
}

pub fn handle_update_equipset(id: u32) -> bool {
    let data = getEquippedItems();
    control::get().cycles.update_equipset(id, *data)
}

/// Rename the equipset with the given ID.
pub fn handle_rename_equipset(id: u32, name: String) -> bool {
    control::get().cycles.rename_equipset(id, name)
}

/// Remove the equipset with the given ID.
pub fn handle_remove_equipset(id: u32) -> bool {
    control::get().cycles.remove_equipset(id.to_string())
}

/// Create the equipped data struct.
pub fn equipped_data(items: Vec<String>, empty_slots: Vec<u8>) -> Box<EquippedData> {
    Box::new(EquippedData { items, empty_slots })
}

pub fn get_equipset_item_names(id: u32) -> Vec<String> {
    // this needs the cache
    control::get().get_equipset_item_names(id)
}

/// Use the icon from the named item for the equipment set with the given id.
pub fn set_equipset_icon(id: u32, itemname: String) -> bool {
    control::get().set_equipset_icon(id, itemname)
}

/// Look up an equipset by name, returning its id. Since uniqueness is not
/// really enforced for names, this returns the first one found.
pub fn look_up_equipset_by_name(name: String) -> u32 {
    control::get().cycles.equipset_by_name(name)
}

pub fn show_ui() -> bool {
    control::get().cycles.hud_visible()
}

// ----------- windows character shenanigans

use textcode::iso8859_15;

/// C++ calls this version.
pub fn string_to_utf8(bytes_ffi: &CxxVector<u8>) -> String {
    let bytes: Vec<u8> = bytes_ffi.iter().copied().collect();
    convert_to_utf8_doggedly(bytes)
}

// To test in game: install daegon
// player.additem 4c2b15f4 1
// Sacrÿfev Tëliimi

/// Get a valid Rust representation of this Windows codepage string data by hook or by crook.
pub fn convert_to_utf8_doggedly(input: Vec<u8>) -> String {
    let bytes = if input.ends_with(&[0]) {
        let chopped = input.len() - 1;
        let mut tmp = input.clone();
        tmp.truncate(chopped);
        tmp
    } else {
        input.clone()
    };
    if bytes.is_empty() {
        return String::new();
    }

    // Maybe it's the easy case and we're done!
    if let Ok(utf8string) = String::from_utf8(bytes.clone()) {
        return utf8string;
    }

    let mut dst = String::new();
    iso8859_15::decode(bytes.as_slice(), &mut dst);
    dst
}

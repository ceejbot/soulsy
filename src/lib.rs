/* wut */

pub mod controller;

use controller::settings::Settings;
use controller::*;

/// Rust defines the bridge between it and C++ in this file, using the affordances
/// of the `cxx` crate. At build time `cxx_build` will generate the header files required
/// by the C++ side.
#[cxx::bridge]
pub mod plugin {
    /// This struct exposes its fields because the UI implementation frequently
    /// refers to them. It is read-only for the C++ side. The values are filled out
    /// by lazily reading the layout toml file.
    #[derive(Deserialize, Serialize, Debug, Clone, Default)]
    pub struct HudLayout {
        /// Enable debug logging.
        debug: bool,
    }

    // I would rather not use exceptions for normal flow control.
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum KeyEventResponse {
        Okay,
        Unhandled,
        Error,
        ItemAdded,
        ItemRemoved,
        ItemInappropriate,
        TooManyItems,
        // i dunno yet
    }

    extern "Rust" {
        // Zero or more opaque types which both languages can pass around
        // but only Rust can see the fields.

        /// Give access to the settings to the C++ side.
        type Settings;

        /// Managed access to the layout object, so we can lazy-load if necessary.
        fn layout() -> &'static HudLayout;
        /// Managed access to the settings object, so we can lazy-load if necessary.
        fn boxed_settings() -> Box<Settings>;
        /// Handle an incoming key press event from the game. Returns true if handled.
        fn handle_key_event(key: u32) -> bool;
        /// Handle an in-menu event (which adds/removes items) from the game.
        fn handle_menu_event(key: u32, item: &TESForm) -> KeyEventResponse;
    }

    unsafe extern "C++" {
        include!("PCH.h");

        #[namespace = "RE"]
        type TESForm;
        #[namespace = "RE"]
        type BGSEquipSlot;

        #[namespace = "helper"]
        type slot_type;

        #[namespace = "helper"]
        fn is_two_handed(item: &TESForm) -> bool;

    }
}

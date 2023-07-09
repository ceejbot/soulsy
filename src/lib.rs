/// Rust defines the bridge between it and C++ in this file, using the affordances
/// of the `cxx` crate. At build time `cxx_build` will generate the header files required
/// by the C++ side.
pub mod controller;

use controller::settings::Settings;
use controller::{boxed_settings, handle_key_event, layout};

#[cxx::bridge]
mod plugin {
    // Any shared structs, whose fields will be visible to both languages.

    #[derive(Deserialize, Serialize, Debug, Clone, Default)]
    pub struct HudLayout {
        // Enable debug logging.
        debug: bool,
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
        /// Handle an incoming key press event from the game.
        fn handle_key_event(key: u32) -> bool;
    }

    unsafe extern "C++" {
        // structs whose fields are invisible to Rust; C++ is source of truth

        // functions implemented in C++

    }
}

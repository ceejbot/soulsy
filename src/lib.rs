pub mod controller;

use controller::settings::*;
use controller::handle_key_event;

#[cxx::bridge]
mod plugin {
    // A simple matter of typing.

    // Any shared structs, whose fields will be visible to both languages.

    extern "Rust" {
        // Zero or more opaque types which both languages can pass around
        // but only Rust can see the fields.

        /// Give access to the settings to the C++ side.
        type Settings;

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

fn boxed_settings() -> Box<Settings> {
    let s = settings();
    Box::new(s.clone()) // grimacing emoji
}

#[cfg(test)]
mod tests {
    // use crate::*;

    #[test]
    fn it_works() {
    }
}

pub mod control;
pub mod cycles;
pub mod layout;
pub mod settings;

// Everything pulled in here is available for lib.rs to use in the bridge.
pub use control::{handle_key_event, handle_menu_event};
pub use cycles::{create_cycle_entry, get_icon_file, CycleEntry};
pub use layout::layout;
pub use settings::{user_settings, UserSettings}; // hmm, is this for settings? I'm confused...

use crate::plugin::HudLayout;

/// Wrapper for C++ convenience; logs errors but does no more
pub fn refresh_user_settings() {
    match settings::UserSettings::refresh() {
        Ok(_) => {
            log::info!("refreshed user settings after MCM edits");
        }
        Err(e) => {
            log::warn!("failed to refresh user settings; using defaults; {e:?}");
        }
    }
}

/// Wrapper for C++ convenience.
pub fn boxed_layout() -> Box<HudLayout> {
    let v = layout();
    Box::new(v.clone()) // grimacing emoji
}

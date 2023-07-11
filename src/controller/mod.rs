pub mod control;
pub mod cycles;
pub mod layout;
pub mod settings;

pub use control::{handle_key_event, handle_menu_event};
pub use cycles::{get_icon_file, create_cycle_entry, CycleEntry};
pub use layout::layout;
pub use settings::{user_settings, UserSettings}; // hmm, is this for settings? I'm confused...

use crate::plugin::HudLayout;

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

pub fn boxed_layout() -> Box<HudLayout> {
    let v = layout();
    Box::new(v.clone()) // grimacing emoji
}

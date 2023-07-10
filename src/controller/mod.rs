pub mod control;
pub mod cycles;
pub mod layout;
pub mod settings;

pub use control::{handle_key_event, handle_menu_event};
pub use layout::layout;
pub use settings::{settings, Settings};

use crate::plugin::HudLayout;

pub fn boxed_settings() -> Box<Settings> {
    let v = settings();
    Box::new(v.clone()) // grimacing emoji
}

pub fn boxed_layout() -> Box<HudLayout> {
    let v = layout();
    Box::new(v.clone()) // grimacing emoji
}

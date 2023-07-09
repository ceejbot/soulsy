pub mod controller;
pub mod cycles;
pub mod layout;
pub mod settings;

pub use controller::handle_key_event;
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

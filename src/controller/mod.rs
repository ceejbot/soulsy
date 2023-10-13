//! The Rust side of the plugin: the controller and model in the MVC parlance
//!
//! The controller is explicitly described as a controller: keyboard and menu
//! events are fed to it, and it makes changes to the data as needed. It also
//! triggers the UI to update in certain cases, but otherwise does not mediate
//! between the data and the renderer.
//!
//! There is little defined in this module file, but everything it re-exports
//! is available to be bridged to C++ in the `plugin` module.
pub mod control;
pub mod cycleentries;
pub mod cycles;
pub mod facade;
pub mod keys;
pub mod layout;
pub mod settings;

pub use facade::*;
pub use layout::{deserialize_align, deserialize_named_anchor, hud_layout};
pub use settings::UserSettings;

//! The Rust side of the plugin: the controller and model in the MVC parlance
//!
//! The controller is explicitly described as a controller: keyboard and menu
//! events are fed to it, and it makes changes to the data as needed. It also
//! triggers the UI to update in certain cases, but otherwise does not mediate
//! between the data and the renderer.
//!
//! There is nothing defined in the module file, but everything it imports and
//! uses is available to be bridged to C++.
pub mod control;
pub mod cycles;
pub mod layout;
pub mod settings;

pub use control::{equipped_in_slot, handle_key_event, handle_menu_event};
pub use cycles::{create_cycle_entry, get_icon_file, CycleEntry};
pub use layout::layout;
pub use settings::{refresh_user_settings, user_settings, UserSettings}; // hmm, is this for settings? I'm confused...

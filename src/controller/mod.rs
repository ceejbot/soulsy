//! The Rust side of the plugin: the controller and model in the MVC parlance
//!
//! The controller is explicitly described as a controller: keyboard and menu
//! events are fed to it, and it makes changes to the data as needed. It also
//! triggers the UI to update in certain cases, but otherwise does not mediate
//! between the data and the renderer.
//!
//! There is little defined in this module file, but everything it imports and
//! uses is available to be bridged to C++.
pub mod control;
pub mod cycles;
pub mod entrykind;
pub mod layout;
pub mod settings;

// We don't have much logging setup code, so just shove it in here.
use std::fs::File;
use std::path::PathBuf;

pub use control::public::*;
pub use cycles::{create_cycle_entry, default_cycle_entry, get_icon_file, CycleEntry};
pub use layout::layout;
pub use settings::{refresh_user_settings, user_settings, UserSettings}; // hmm, is this for settings? I'm confused...
use simplelog::*;

pub fn initialize_rust_logging(logdir: &cxx::CxxString) {
    // TODO: read from config
    let level = LevelFilter::Info;

    let mut pathbuf = PathBuf::from(logdir.to_string());
    pathbuf.set_file_name("SoulsyHUD_rust.log");

    if let Ok(logfile) = File::create(&pathbuf) {
        let _ = WriteLogger::init(level, Config::default(), logfile);
    } else {
        // Welp, we failed and I have nowhere to write the darn error. Ha ha.
        panic!(
            "run in circles scream and shout: rust can't log; {}",
            pathbuf.display()
        );
    }
}

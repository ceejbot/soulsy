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
pub use cycles::{default_cycle_entry, get_icon_file, make_tesitem, TesItemData};
pub use entrykind::{kind_has_count, kind_is_magic};
pub use layout::layout;
pub use settings::{refresh_user_settings, user_settings, UserSettings}; // hmm, is this for settings? I'm confused...

use simplelog::*;

pub fn initialize_rust_logging(logdir: &cxx::CxxString) {
    let hudl = layout(); // yeah, it's in here, sorry. we can reload this at runtime.
    let log_level = if hudl.debug {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };

    let mut pathbuf = PathBuf::from(logdir.to_string());
    pathbuf.set_file_name("SoulsyHUD_rust.log");

    if let Ok(logfile) = File::create(&pathbuf) {
        let _ = WriteLogger::init(log_level, Config::default(), logfile);
        log::info!("rust side logging standing by");
    } else {
        // Welp, we failed and I have nowhere to write the darn error. Ha ha.
        panic!(
            "run in circles scream and shout: rust can't log; {}",
            pathbuf.display()
        );
    }
}

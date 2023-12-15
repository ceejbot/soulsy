//! Logging functions exposed to C++. There's an initialization function
//! that must be called to tell the plugin where to log. The other functions
//! are for C++ to use to share a log file with Rust. For now, C++ must pass
//! a preformatted-string to these functions. This is wasteful, but I don't
//! C strings prove a bit of a pain.

use std::ffi::OsString;
use std::fs::File;
#[cfg(target_os = "windows")]
use std::os::windows::ffi::OsStringExt;
use std::path::Path;

use simplelog::*;

use super::settings::settings;

// ---------- logging

pub fn initialize_rust_logging(_logdir: &cxx::CxxVector<u16>) {
    let config = settings();
    let log_level = config.log_level().to_level_filter();

    #[cfg(not(target_os = "windows"))]
    let chonky_path = OsString::from("placeholder");
    #[cfg(target_os = "windows")]
    let chonky_path = OsString::from_wide(_logdir.as_slice());
    let path = Path::new(chonky_path.as_os_str()).with_file_name("SoulsyHUD.log");

    let Ok(logfile) = File::create(path) else {
        // Welp, we failed and I have nowhere to write the darn error. Ha ha.
        return;
    };
    let config = simplelog::ConfigBuilder::new()
        .set_thread_level(LevelFilter::Off)
        .set_level_padding(simplelog::LevelPadding::Right)
        .build();
    let Ok(_) = WriteLogger::init(log_level, config, logfile) else {
        // oh dear
        return;
    };
    log::info!("SoulsyHUD version {} coming online.", env!("CARGO_PKG_VERSION"));
}

pub fn log_error(message: String) {
    log::error!("{}", message);
}

pub fn log_warn(message: String) {
    log::warn!("{}", message);
}

pub fn log_info(message: String) {
    log::info!("{}", message);
}

pub fn log_debug(message: String) {
    log::debug!("{}", message);
}

pub fn log_trace(message: String) {
    log::trace!("{}", message);
}

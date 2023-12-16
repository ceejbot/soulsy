//! Logging functions exposed to C++. There's an initialization function
//! that must be called to tell the plugin where to log. The other functions
//! are for C++ to use to share a log file with Rust. For now, C++ must pass
//! a preformatted-string to these functions. This is wasteful, but I don't
//! C strings prove a bit of a pain.

use std::ffi::OsString;
#[cfg(target_os = "windows")]
use std::os::windows::ffi::OsStringExt;
use std::path::Path;
use std::sync::Arc;

use once_cell::sync::OnceCell;
use spdlog::formatter::{pattern, PatternFormatter};
use spdlog::{Level, Logger};

use super::settings::settings;

static CPPLOGGER: OnceCell<Logger> = OnceCell::new();

// ---------- logging

pub fn initialize_logging(_logdir: &cxx::CxxVector<u16>) -> bool {
    #[cfg(not(target_os = "windows"))]
    let chonky_path = OsString::from("placeholder");
    #[cfg(target_os = "windows")]
    let chonky_path = OsString::from_wide(_logdir.as_slice());
    let path = Path::new(chonky_path.as_os_str()).with_file_name("SoulsyHUD.log");
    log_to_file(&path)
}

fn log_to_file(path: &Path) -> bool {
    let config = settings();
    let level_filter = spdlog::LevelFilter::MoreSevereEqual(config.log_level());

    // Make a file logger with default formatting. Set this as the default logger.
    // Make a second logger with the same sinks. Give it the formatting that the C++ logging
    // needs, and make the functions below use it in their macro invocations.

    let formatter = PatternFormatter::new(pattern!("{time} [{level}] {payload}{eol}"));
    let Ok(file_sink) = spdlog::sink::FileSink::builder()
        .path(path)
        .truncate(false)
        .level_filter(level_filter)
        .formatter(Box::new(formatter))
        .build()
    else {
        // oh dear
        return false;
    };

    let Ok(rust_logger) = Logger::builder().sink(Arc::new(file_sink)).build() else {
        // oh dear oh dear
        return false;
    };
    spdlog::set_default_logger(Arc::new(rust_logger));

    let formatter = PatternFormatter::new(pattern!("{time} [{level}] {payload}{eol}"));
    let Ok(cpp_sink) = spdlog::sink::FileSink::builder()
        .path(path)
        .truncate(false)
        .level_filter(level_filter)
        .formatter(Box::new(formatter))
        .build()
    else {
        // oh dear oh dear
        return false;
    };
    let Ok(cpp_logger) = Logger::builder().sink(Arc::new(cpp_sink)).build() else {
        // oh dear oh dear oh dear
        return false;
    };

    let Ok(_) = CPPLOGGER.set(cpp_logger) else {
        // too many oh dears to type
        return false;
    };
    // use it in the functions below

    spdlog::info!(
        "SoulsyHUD version {} coming online.",
        env!("CARGO_PKG_VERSION")
    );
    true
}

pub fn log_critical(message: String) {
    if let Some(logger) = CPPLOGGER.get() {
        spdlog::log!(logger:logger, Level::Critical, "{}", message);
    } else {
        spdlog::critical!("{}", message);
    }
}

pub fn log_error(message: String) {
    if let Some(logger) = CPPLOGGER.get() {
        spdlog::log!(logger:logger, Level::Error, "{}", message);
    } else {
        spdlog::error!("{}", message);
    }
}

pub fn log_warn(message: String) {
    if let Some(logger) = CPPLOGGER.get() {
        spdlog::log!(logger:logger, Level::Warn, "{}", message);
    } else {
        spdlog::warn!("{}", message);
    }
}

pub fn log_info(message: String) {
    if let Some(logger) = CPPLOGGER.get() {
        spdlog::log!(logger:logger, Level::Info, "{}", message);
    } else {
        spdlog::info!("{}", message);
    }
}

pub fn log_debug(message: String) {
    if let Some(logger) = CPPLOGGER.get() {
        spdlog::log!(logger:logger, Level::Debug, "{}", message);
    } else {
        spdlog::debug!("{}", message);
    }
}

pub fn log_trace(message: String) {
    if let Some(logger) = CPPLOGGER.get() {
        spdlog::log!(logger:logger, Level::Trace, "{}", message);
    } else {
        spdlog::trace!("{}", message);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn initialize_logging_test(logdir: &str) -> bool {
        let chonky_path = OsString::from(logdir);
        let path = Path::new(chonky_path.as_os_str()).with_file_name("SoulsyHUD.log");
        log_to_file(&path)
    }

    #[test]
    fn can_construct_logger() {
        assert!(initialize_logging_test("."));
        let logger = CPPLOGGER.get().expect("should be set after initialization");
        logger.flush();
        log_critical("critical level log".to_string());
        log_error("error level log".to_string());
        log_warn("warn level log".to_string());
        log_info("info level log".to_string());
        log_debug("debug level log".to_string());
        log_trace("trace level log".to_string());
        spdlog::info!("info level rust log");
        logger.flush();
    }
}

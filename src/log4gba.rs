use core::fmt::Write;
use gba::mgba::{MgbaBufferedLogger, MgbaMessageLevel};

pub fn log(message_level: MgbaMessageLevel, message: impl core::fmt::Debug) {
    if let Ok(mut logger) = MgbaBufferedLogger::try_new(message_level) {
        // Concatenate all parameters into a single string
        writeln!(logger, "{:?}", message).ok();
    }
}

pub fn fatal(message: impl core::fmt::Debug) {
    log(MgbaMessageLevel::Fatal, message)
}

pub fn error(message: impl core::fmt::Debug) {
    log(MgbaMessageLevel::Error, message)
}

pub fn warning(message: impl core::fmt::Debug) {
    log(MgbaMessageLevel::Warning, message)
}

pub fn debug(message: impl core::fmt::Debug) {
    log(MgbaMessageLevel::Debug, message)
}

pub fn info(message: impl core::fmt::Debug) {
    log(MgbaMessageLevel::Info, message)
}

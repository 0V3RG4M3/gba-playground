#![no_std]
#![no_main]

use core::fmt::Write;
use gba::mgba::{MgbaBufferedLogger, MgbaMessageLevel};

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    if let Ok(mut logger) = MgbaBufferedLogger::try_new(MgbaMessageLevel::Fatal) {
        writeln!(logger, "{info}").ok();
    }
    loop {}
}

#[no_mangle]
pub fn main() -> ! {
    loop {}
}
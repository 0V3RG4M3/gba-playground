#![no_std]
#![no_main]

use core::fmt::Write;
use gba::bios;
use gba::mmio;
use gba::interrupts::IrqBits;
use gba::video::{DisplayStatus};
use gba::mgba::{MgbaBufferedLogger, MgbaMessageLevel};

use gba_playground::gba_synth::GbaSynth;
use gba_playground::egj2025::reg_tune as egj2025_tune;
use gba_playground::log4gba;

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    if let Ok(mut logger) = MgbaBufferedLogger::try_new(MgbaMessageLevel::Fatal) {
        writeln!(logger, "{info}").ok();
    }
    loop {}
}



#[unsafe(no_mangle)]
pub fn main() -> ! {
    log4gba::debug("Starting jukebox...");

    mmio::DISPSTAT.write(DisplayStatus::new().with_irq_vblank(true));
    mmio::IE.write(IrqBits::VBLANK);
    mmio::IME.write(true);


    let mut synth = GbaSynth::new();
    synth.init(&egj2025_tune::TUNE);

    log4gba::debug("Starting loop...");
    loop {
        bios::VBlankIntrWait();
        synth.play_step();
    }
}

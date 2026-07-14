#![no_std]
#![no_main]

use core::fmt::Write;
use gba::bios;
use gba::interrupts::IrqBits;
use gba::keys::KeyInput;
use gba::mgba::{MgbaBufferedLogger, MgbaMessageLevel};
use gba::mmio;
use gba::video::DisplayStatus;

use gba_playground::discography::noisebeat;
use gba_playground::egj2025::reg_tune as egj2025_tune;
use gba_playground::egj2026::tune1 as egj2026_tune;
use gba_playground::gba_synth::GbaSynth;
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

    // Initialize key input
    let mut key_was_pressed: KeyInput = KeyInput::new();

    // Create list of tunes
    let tunes = [&egj2025_tune::TUNE, &egj2026_tune::TUNE, &noisebeat::TUNE];
    let mut current_tune_index = 0;

    // Initialize the GBA Synthesizer
    let mut synth = GbaSynth::new();
    synth.init(tunes[current_tune_index]);

    log4gba::debug("Starting loop...");
    loop {
        bios::VBlankIntrWait();

        let key_input: KeyInput = mmio::KEYINPUT.read();

        if key_input.down() {
            // on press
            if !key_was_pressed.down() {
                log4gba::debug("Down key pressed");
                current_tune_index = (current_tune_index + 1) % tunes.len();
                synth.init(tunes[current_tune_index]);
            }
        }

        if key_input.up() {
            // on press
            if !key_was_pressed.up() {
                log4gba::debug("Up key pressed");
                current_tune_index = (current_tune_index + tunes.len() - 1) % tunes.len();
                synth.init(tunes[current_tune_index]);
            }
        }

        key_was_pressed = key_input;

        synth.play_step();
    }
}

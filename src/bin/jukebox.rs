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
use gba_playground::egj2026::sfx_jump;
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

    // Initialize state variables
    let mut key_was_pressed: KeyInput = KeyInput::new();
    let mut is_playing = true;

    // Create list of tunes
    let tunes = [&egj2025_tune::TUNE, &egj2026_tune::TUNE, &noisebeat::TUNE];
    let mut tune_ind = 0;

    // Create list of sound effects
    let sfxs = [&sfx_jump::TUNE];
    let mut sfx_ind = 0;

    // Initialize the GBA Synthesizer
    let mut synth = GbaSynth::new();
    synth.init(tunes[tune_ind]);

    log4gba::debug("Starting loop...");
    loop {
        bios::VBlankIntrWait();

        let key_input: KeyInput = mmio::KEYINPUT.read();

        if key_input.down() {
            // on press
            if !key_was_pressed.down() {
                log4gba::debug("Down key pressed");
                tune_ind = (tune_ind + 1) % tunes.len();
                synth.init(tunes[tune_ind]);
            }
        }

        if key_input.up() {
            // on press
            if !key_was_pressed.up() {
                log4gba::debug("Up key pressed");
                tune_ind = (tune_ind + tunes.len() - 1) % tunes.len();
                synth.init(tunes[tune_ind]);
            }
        }

        if key_input.right() {
            // on press
            if !key_was_pressed.right() {
                log4gba::debug("Right key pressed");
                sfx_ind = (sfx_ind + 1) % sfxs.len();
                synth.trigger_sfx(sfxs[sfx_ind]);
            }
        }

        if key_input.left() {
            // on press
            if !key_was_pressed.left() {
                log4gba::debug("Left key pressed");
                sfx_ind = (sfx_ind + sfxs.len() - 1) % sfxs.len();
                synth.trigger_sfx(sfxs[sfx_ind]);
            }
        }

        if key_input.start() {
            // on press
            if !key_was_pressed.start() {
                log4gba::debug("Start key pressed");
                is_playing = !is_playing;
            }
        }

        if key_input.l() {
            // on press
            if !key_was_pressed.l() {
                log4gba::debug("L key pressed");
                synth.init(tunes[tune_ind]);
            }
        }

        // hold R to double the speed
        let speed = if key_input.r() { 2 } else { 1 };

        key_was_pressed = key_input;

        if is_playing {
            for _ in 0..speed {
                synth.play_step();
            }
        }
    }
}

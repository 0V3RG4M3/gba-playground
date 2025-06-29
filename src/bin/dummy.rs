#![no_std]
#![no_main]

use core::fmt::Write;
use gba::mgba::{MgbaBufferedLogger, MgbaMessageLevel};

use gba::mmio;
use gba::sound;

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    if let Ok(mut logger) = MgbaBufferedLogger::try_new(MgbaMessageLevel::Fatal) {
        writeln!(logger, "{info}").ok();
    }
    loop {}
}

fn enable_sound() {
    mmio::SOUND_ENABLED.write(sound::SoundEnable::new().with_enabled(true));

    mmio::LEFT_RIGHT_VOLUME.write(
        sound::LeftRightVolume::new()
            .with_right_volume(7)
            .with_left_volume(7)
            .with_tone1_left(true)
            .with_tone2_left(true)
            .with_noise_left(true)
            .with_tone1_right(true)
            .with_tone2_right(true)
            .with_noise_right(true),
    );

    mmio::SOUND_MIX.write(sound::SoundMix::new().with_psg(sound::PsgMix::_50));
}

#[no_mangle]
pub fn main() -> ! {
    enable_sound();
    loop {}
}

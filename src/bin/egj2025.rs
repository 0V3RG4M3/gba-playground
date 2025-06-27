#![no_std]
#![no_main]

use core::fmt::Write;

use gba;
use gba::bios;
use gba::fixed::{i16fx8, i32fx8};
use gba::interrupts::IrqBits;
use gba::mgba::{MgbaBufferedLogger, MgbaMessageLevel};
use gba::mmio;
use gba::video::DisplayStatus;

use gba_playground::egj2025::screen_splash_scene::ScreenSplashScene;
use gba_playground::scene::SceneRunner;

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    if let Ok(mut logger) = MgbaBufferedLogger::try_new(MgbaMessageLevel::Fatal) {
        writeln!(logger, "{info}").ok();
    }
    loop {}
}

#[no_mangle]
extern "C" fn main() -> ! {
    let mut scene_runner = SceneRunner::<()>::new::<ScreenSplashScene>();
    loop {
        mmio::DISPSTAT.write(DisplayStatus::new().with_irq_vblank(true));
        mmio::IE.write(IrqBits::new().with_vblank(true));
        mmio::IME.write(true);
        gba::RUST_IRQ_HANDLER.write(None);
        mmio::BG2PA.write(i16fx8::wrapping_from(1));
        mmio::BG2PC.write(i16fx8::wrapping_from(0));
        mmio::BG2X.write(i32fx8::from_bits(0));
        mmio::BG2Y.write(i32fx8::from_bits(0));
        bios::VBlankIntrWait();
        scene_runner = scene_runner.run(&mut ());
    }
}

#![no_std]
#![no_main]

use core::fmt::Write;

use gba::bios;
use gba::interrupts::IrqBits;
use gba::mgba::{MgbaBufferedLogger, MgbaMessageLevel};
use gba::mmio;
use gba::video::{Color, DisplayStatus};

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    if let Ok(mut logger) = MgbaBufferedLogger::try_new(MgbaMessageLevel::Fatal) {
        writeln!(logger, "{info}").ok();
    }
    loop {}
}

#[no_mangle]
extern "C" fn main() -> ! {
    mmio::DISPSTAT.write(DisplayStatus::new().with_irq_vblank(true).with_irq_hblank(true));
    mmio::IE.write(IrqBits::new().with_vblank(true).with_hblank(true));

    mmio::BG_PALETTE.index(1).write(Color::BLACK);
    mmio::BG_PALETTE.index(2).write(Color::WHITE);

    loop {
        bios::VBlankIntrWait();

        loop { 
            bios::IntrWait(true, IrqBits::HBLANK);
            let vcount = mmio::VCOUNT.read();
            let next_vcount = if vcount == 227 { 0 } else { vcount + 1 };
            if next_vcount >= 160 {
                continue;
            }
        }
    }
}

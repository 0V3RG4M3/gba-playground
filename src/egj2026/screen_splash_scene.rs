use core::fmt::Write;

use gba::interrupts::IrqBits;
use gba::mmio::DISPCNT;
use gba::prelude::{DisplayControl, DisplayStatus, VideoMode};
use gba::{bios, mmio, video};

use crate::egj2026::screens;
use crate::log4gba;
use crate::scene::{Scene, SceneRunner};

pub struct ScreenSplashScene {}

impl ScreenSplashScene {
    fn wait_start_bt() {
        let mut rcnt = mmio::RCNT.read();
        rcnt &= !(1 << 14);
        rcnt &= !(1 << 15);
        mmio::RCNT.write(rcnt);

        let mut siocnt = mmio::SIOCNT.read();
        siocnt &= !(1 << 12);
        siocnt |= 1 << 13;
        siocnt |= 1 << 14;
        mmio::SIOCNT.write(siocnt);

        loop {
            bios::VBlankIntrWait();
            let mut siocnt = mmio::SIOCNT.read();
            let parent = (siocnt >> 2) & 1 == 0;
            let ready = (siocnt >> 3) & 1 != 0;
            if ready {
                if let Ok(mut logger) =
                    gba::mgba::MgbaBufferedLogger::try_new(gba::mgba::MgbaMessageLevel::Debug)
                {
                    writeln!(logger, "{ready} {parent} {siocnt:x?}").ok();
                }

                if parent {
                    mmio::SIOMLT_SEND.write(42);
                    siocnt |= 1 << 7;
                    mmio::SIOCNT.write(siocnt);
                } else {
                    mmio::SIOMLT_SEND.write(24);
                }
            }

            gba::RUST_IRQ_HANDLER.write(Some(irq_handler));

            let key_input = mmio::KEYINPUT.read();
            if key_input.start() {
                break;
            }
        }
    }
}

impl Scene for ScreenSplashScene {
    type C = ();

    fn new(_: &mut ()) -> ScreenSplashScene {
        ScreenSplashScene {}
    }

    fn run(&mut self, _: &mut Self::C) -> SceneRunner<Self::C> {
        mmio::DISPSTAT.write(DisplayStatus::new().with_irq_vblank(true));
        mmio::IE.write(IrqBits::new().with_vblank(true).with_hblank(true).with_serial(true));
        mmio::IME.write(true);

        video::video3_set_bitmap(&screens::SCREEN_SPLASH);
        DISPCNT.write(DisplayControl::new().with_video_mode(VideoMode::_3).with_show_bg2(true));

        log4gba::debug("wait start bt");
        Self::wait_start_bt();
        log4gba::debug("start bt pressed");

        loop {}
    }
}

#[unsafe(link_section = ".iwram")]
extern "C" fn irq_handler(irq_bits: IrqBits) {
    if !irq_bits.serial() {
        return;
    }

    let siocnt = mmio::SIOCNT.read();
    let parent = (siocnt >> 2) & 1 == 0;
    let ready = (siocnt >> 3) & 1 != 0;
    let siomulti0 = mmio::SIOMULTI0.read();
    let siomulti1 = mmio::SIOMULTI1.read();
    if let Ok(mut logger) =
        gba::mgba::MgbaBufferedLogger::try_new(gba::mgba::MgbaMessageLevel::Debug)
    {
        writeln!(logger, "irq {ready} {parent} {siocnt:x?} {siomulti0} {siomulti1}").ok();
    }
}

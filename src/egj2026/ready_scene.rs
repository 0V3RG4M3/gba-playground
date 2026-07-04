use gba::interrupts::IrqBits;
use gba::mmio::DISPCNT;
use gba::prelude::{DisplayControl, DisplayStatus, VideoMode};
use gba::{bios, mmio, video};

use crate::egj2026::screens;
use crate::scene::{Scene, SceneRunner};

pub struct ReadyScene {}

impl Scene for ReadyScene {
    type C = ();

    fn new(_: &mut ()) -> ReadyScene {
        ReadyScene {}
    }

    fn run(&mut self, _: &mut Self::C) -> SceneRunner<Self::C> {
        mmio::DISPSTAT.write(DisplayStatus::new().with_irq_vblank(true));
        mmio::IE.write(IrqBits::new().with_vblank(true).with_hblank(true).with_serial(true));
        mmio::IME.write(true);

        video::video3_set_bitmap(&screens::SCREEN_YOUWIN);
        DISPCNT.write(DisplayControl::new().with_video_mode(VideoMode::_3).with_show_bg2(true));

        loop {
            bios::VBlankIntrWait();

            /*let key_input = mmio::KEYINPUT.read();
            if key_input.start() {
                break;
            }*/
        }
    }
}

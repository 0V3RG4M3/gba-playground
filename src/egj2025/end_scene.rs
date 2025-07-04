use gba::interrupts::IrqBits;
use gba::keys::KeyInput;
use gba::mmio::DISPCNT;
use gba::prelude::{DisplayControl, DisplayStatus, VideoMode};
use gba::{bios, mmio, video};

use crate::egj2025::context::Context;
use crate::egj2025::screens;
use crate::egj2025::splash_scene::SplashScene;
use crate::log4gba;
use crate::scene::{Scene, SceneRunner};

pub struct EndScene {}

impl EndScene {
    fn wait_start_bt() {
        let mut prev_key_input: Option<KeyInput> = None;
        loop {
            bios::VBlankIntrWait();
            let key_input = mmio::KEYINPUT.read();
            if let Some(prev_key_input) = prev_key_input {
                if !prev_key_input.start() && key_input.start() {
                    break;
                }
            }
            prev_key_input = Some(key_input);
        }
    }
}

impl Scene for EndScene {
    type C = Context;

    fn new(_: &mut Context) -> EndScene {
        EndScene {}
    }

    fn run(&mut self, _: &mut Context) -> SceneRunner<Context> {
        mmio::DISPSTAT.write(DisplayStatus::new().with_irq_vblank(true));
        mmio::IE.write(IrqBits::new().with_vblank(true).with_hblank(true));
        mmio::IME.write(true);

        video::video3_set_bitmap(&screens::SCREEN_YOUWIN);
        DISPCNT.write(DisplayControl::new().with_video_mode(VideoMode::_3).with_show_bg2(true));

        log4gba::debug("wait start bt");
        Self::wait_start_bt();
        log4gba::debug("start bt pressed");

        SceneRunner::<Context>::new::<SplashScene>()
    }
}

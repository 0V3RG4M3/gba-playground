use gba::interrupts::IrqBits;
use gba::keys::KeyInput;
use gba::mmio::DISPCNT;
use gba::prelude::{DisplayControl, DisplayStatus, VideoMode};
use gba::{bios, mmio, video};

use crate::egj2025::context::Context;
use crate::egj2025::level_scene_runners;
use crate::egj2025::screens;
use crate::log4gba;
use crate::scene::{Scene, SceneRunner};

pub struct EventScene {}

impl EventScene {
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

impl Scene for EventScene {
    type C = Context;

    fn new(_: &mut Context) -> EventScene {
        EventScene {}
    }

    fn run(&mut self, context: &mut Context) -> SceneRunner<Context> {
        mmio::DISPSTAT.write(DisplayStatus::new().with_irq_vblank(true));
        mmio::IE.write(IrqBits::new().with_vblank(true));
        mmio::IME.write(true);

        video::video3_set_bitmap(&screens::SCREEN_ENIGME2);
        DISPCNT.write(DisplayControl::new().with_video_mode(VideoMode::_3).with_show_bg2(true));

        log4gba::debug("wait start bt");
        Self::wait_start_bt();
        log4gba::debug("start bt pressed");

        level_scene_runners::LEVEL_SCENE_RUNNERS[context.level_index]()
    }
}

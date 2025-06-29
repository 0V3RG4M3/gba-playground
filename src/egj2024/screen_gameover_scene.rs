use crate::egj2024::game_scene::GameScene;
use crate::egj2024::screens;
use crate::log4gba;
use crate::scene::{Scene, SceneRunner};
use gba::interrupts::IrqBits;
use gba::mmio::DISPCNT;
use gba::prelude::{DisplayControl, DisplayStatus, VideoMode};
use gba::{bios, mmio, video};

pub struct ScreenGameoverScene {}

impl ScreenGameoverScene {
    fn wait_start_bt() {
        loop {
            bios::VBlankIntrWait();
            let key_input = mmio::KEYINPUT.read();
            if key_input.start() {
                break;
            }
        }
    }
}

impl Scene for ScreenGameoverScene {
    type C = ();

    fn new(_: &mut ()) -> ScreenGameoverScene {
        ScreenGameoverScene {}
    }

    fn run(&mut self, _: &mut Self::C) -> SceneRunner<Self::C> {
        mmio::DISPSTAT.write(DisplayStatus::new().with_irq_vblank(true));
        mmio::IE.write(IrqBits::new().with_vblank(true));
        mmio::IME.write(true);

        video::video3_set_bitmap(&screens::SCREEN_GAMEOVER);
        DISPCNT.write(DisplayControl::new().with_video_mode(VideoMode::_3).with_show_bg2(true));

        log4gba::debug("wait start bt");
        Self::wait_start_bt();
        log4gba::debug("start bt pressed");

        SceneRunner::<()>::new::<GameScene>()
    }
}

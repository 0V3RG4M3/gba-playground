use crate::egj2024::game_scene::GameScene;
use crate::log4gba;
use crate::scene::{Scene, SceneRunner};
use crate::screens;
use gba::interrupts::IrqBits;
use gba::mmio::DISPCNT;
use gba::prelude::{DisplayControl, DisplayStatus, VideoMode};
use gba::{bios, mmio, video};

pub struct ScreenSplashScene {}

impl ScreenSplashScene {
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

impl Scene for ScreenSplashScene {
    type C = ();

    fn new(_: &mut ()) -> ScreenSplashScene {
        ScreenSplashScene {}
    }

    fn run(&mut self, _: &mut Self::C) -> SceneRunner<Self::C> {
        mmio::DISPSTAT.write(DisplayStatus::new().with_irq_vblank(true));
        mmio::IE.write(IrqBits::new().with_vblank(true).with_hblank(true));
        mmio::IME.write(true);

        video::video3_set_bitmap(&screens::SCREEN_SPLASH);
        DISPCNT.write(DisplayControl::new().with_video_mode(VideoMode::_3).with_show_bg2(true));

        log4gba::debug("wait start bt");
        Self::wait_start_bt();
        log4gba::debug("start bt pressed");

        SceneRunner::<()>::new::<GameScene>()
    }
}

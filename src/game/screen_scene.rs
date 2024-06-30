use gba::{bios, mmio};
use gba::interrupts::IrqBits;
use gba::mem_fns::__aeabi_memcpy;
use gba::mmio::{DISPCNT, TEXT_SCREENBLOCKS};
use gba::prelude::{DisplayControl, DisplayStatus, VideoMode};
use crate::game::game_scene::GameScene;
use crate::scene::{Scene, SceneRunner};
use crate::screens;
use crate::log4gba;

pub struct ScreenScene {}

impl ScreenScene {
    fn wait_start_bt(){
        loop {
            bios::VBlankIntrWait();
            let key_input = mmio::KEYINPUT.read();
            if key_input.start() {break}

        }
    }
}


impl Scene for ScreenScene {
    type C = ();

    fn new(_: &mut ()) -> ScreenScene {
        ScreenScene {}
    }

    fn run(&mut self, context: &mut Self::C) -> SceneRunner<Self::C> {
        mmio::DISPSTAT.write(DisplayStatus::new().with_irq_vblank(true));
        mmio::IE.write(IrqBits::new().with_vblank(true).with_hblank(true));
        mmio::IME.write(true);

        let a = TEXT_SCREENBLOCKS.get_frame(0).unwrap().as_usize();
        unsafe {
            __aeabi_memcpy(
                a as _,
                screens::SCREEN_SPLASH.as_ptr().cast(),
                core::mem::size_of_val(screens::SCREEN_SPLASH) as _,
            )
        };
        DISPCNT.write(DisplayControl::new().with_video_mode(VideoMode::_3).with_show_bg2(true));

        log4gba::debug("wait start bt");
        Self::wait_start_bt();
        log4gba::debug("start bt pressed");

        SceneRunner::<()>::new::<GameScene>()
    }
}
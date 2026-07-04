use gba::interrupts::IrqBits;
use gba::mmio::DISPCNT;
use gba::prelude::{DisplayControl, DisplayStatus, VideoMode};
use gba::{bios, mmio, video};

use crate::egj2026::link;
use crate::egj2026::screens;
use crate::egj2026::tx_state::TxState;
use crate::egj2026::wait_scene::WaitScene;
use crate::scene::{Scene, SceneRunner};

pub struct ReadyScene {}

impl Scene for ReadyScene {
    type C = ();

    fn new(_: &mut ()) -> ReadyScene {
        ReadyScene {}
    }

    fn run(&mut self, _: &mut Self::C) -> SceneRunner<Self::C> {
        mmio::DISPSTAT.write(DisplayStatus::new().with_irq_vblank(true));
        mmio::IE.write(IrqBits::new().with_vblank(true).with_serial(true));
        mmio::IME.write(true);

        video::video3_set_bitmap(&screens::SCREEN_YOUWIN);
        DISPCNT.write(DisplayControl::new().with_video_mode(VideoMode::_3).with_show_bg2(true));

        let mut frame = 1;

        loop {
            let key_input = mmio::KEYINPUT.read();
            let tx_state = TxState::new().with_frame(frame).with_key_input(key_input);
            link::write(tx_state);

            bios::VBlankIntrWait();

            let rx_state = link::read();
            if !rx_state.connected() {
                continue;
            }

            if rx_state.frame() != frame {
                continue;
            }

            if rx_state.key_inputs()[0].start() {
                break SceneRunner::<()>::new::<WaitScene>();
            }

            frame = (frame + 1) % 16;
        }
    }
}

use gba::interrupts::IrqBits;
use gba::mmio::DISPCNT;
use gba::prelude::{DisplayControl, DisplayStatus, VideoMode};
use gba::{bios, mmio, video};

use crate::egj2026::context::Context;
use crate::egj2026::game_scene::GameScene;
use crate::egj2026::link;
use crate::egj2026::screens;
use crate::egj2026::tx_state::TxState;
use crate::scene::{Scene, SceneRunner};

pub struct ReadyScene;

impl Scene for ReadyScene {
    type C = Context;

    fn new(_: &mut Self::C) -> Self {
        Self
    }

    fn run(&mut self, context: &mut Self::C) -> SceneRunner<Self::C> {
        mmio::DISPSTAT.write(DisplayStatus::new().with_irq_vblank(true));
        mmio::IE.write(IrqBits::new().with_vblank(true).with_serial(true));
        mmio::IME.write(true);

        video::video3_set_bitmap(&screens::SCREEN_YOUWIN);
        DISPCNT.write(DisplayControl::new().with_video_mode(VideoMode::_3).with_show_bg2(true));

        loop {
            gba::RUST_IRQ_HANDLER.write(Some(irq_handler));

            let key_input = mmio::KEYINPUT.read();
            let tx_state = TxState::new().with_frame(context.frame).with_key_input(key_input);
            link::write(tx_state);

            bios::VBlankIntrWait();

            let rx_state = link::read();
            if !rx_state.connected() {
                continue;
            }

            if rx_state.frame() != context.frame {
                continue;
            }

            if rx_state.key_inputs()[0].start() {
                break SceneRunner::<Self::C>::new::<GameScene>();
            }

            context.frame = (context.frame + 1) % 16;
        }
    }
}

#[unsafe(link_section = ".iwram")]
extern "C" fn irq_handler(irq_bits: IrqBits) {
    if irq_bits.serial() {
        link::process();
    }
}

use gba::interrupts::IrqBits;
use gba::mmio::DISPCNT;
use gba::prelude::{DisplayControl, DisplayStatus, VideoMode};
use gba::{bios, mmio, video};

use crate::egj2026::context::Context;
use crate::egj2026::link;
use crate::egj2026::ready_scene::ReadyScene;
use crate::egj2026::screens;
use crate::egj2026::tx_state::TxState;
use crate::scene::{Scene, SceneRunner};

pub struct WaitScene;

impl Scene for WaitScene {
    type C = Context;

    fn new(_: &mut Self::C) -> Self {
        Self
    }

    fn run(&mut self, _: &mut Self::C) -> SceneRunner<Self::C> {
        mmio::DISPSTAT.write(DisplayStatus::new().with_irq_vblank(true));
        mmio::IE.write(IrqBits::new().with_vblank(true).with_serial(true));
        mmio::IME.write(true);

        video::video3_set_bitmap(&screens::SCREEN_SPLASH);
        DISPCNT.write(DisplayControl::new().with_video_mode(VideoMode::_3).with_show_bg2(true));

        link::init();

        loop {
            gba::RUST_IRQ_HANDLER.write(Some(irq_handler));

            let tx_state = TxState::new();
            link::write(tx_state);

            bios::VBlankIntrWait();

            let rx_state = link::read();
            if rx_state.connected() {
                break SceneRunner::<Self::C>::new::<ReadyScene>();
            }
        }
    }
}

#[unsafe(link_section = ".iwram")]
extern "C" fn irq_handler(irq_bits: IrqBits) {
    if irq_bits.serial() {
        link::process();
    }
}

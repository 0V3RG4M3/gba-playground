use core::cmp;

use gba::bios;
use gba::interrupts::IrqBits;
use gba::mmio;
use gba::video::obj::{ObjAttr, ObjAttr0, ObjAttr1, ObjAttr2, ObjAttrWriteExt, ObjDisplayStyle};
use gba::video::{BackgroundControl, Color, DisplayControl, DisplayStatus, TextEntry};

use crate::egj2026::context::Context;
use crate::egj2026::link;
use crate::egj2026::tx_state::TxState;
use crate::scene::{Scene, SceneRunner};

pub struct GameScene;

impl Scene for GameScene {
    type C = Context;

    fn new(_: &mut Self::C) -> Self {
        Self
    }

    fn run(&mut self, context: &mut Self::C) -> SceneRunner<Self::C> {
        mmio::DISPSTAT.write(DisplayStatus::new().with_irq_vblank(true));
        mmio::IE.write(IrqBits::new().with_vblank(true).with_serial(true));
        mmio::IME.write(true);

        mmio::BG_PALETTE.index(1).write(Color::BLACK);
        mmio::BG_PALETTE.index(2).write(Color::WHITE);
        mmio::OBJ_PALETTE.index(1).write(Color::MAGENTA);

        mmio::CHARBLOCK0_8BPP.index(0).write([0x01010101; 16]);
        mmio::CHARBLOCK0_8BPP.index(1).write([0x02020202; 16]);
        let screenblock = mmio::TEXT_SCREENBLOCKS.get_frame(1).unwrap();
        for y in 0..32 {
            for x in 0..32 {
                let tile = if y < 16 { 0 } else { 1 };
                screenblock.index(x, y).write(TextEntry::new().with_tile(tile));
            }
        }
        mmio::BG0CNT.write(BackgroundControl::new().with_bpp8(true).with_screenblock(1));

        mmio::OBJ_TILES.index(0).write([0x01010101; 8]);
        mmio::OBJ_TILES.index(1).write([0x01010101; 8]);
        for i in 1..128 {
            let va = mmio::OBJ_ATTR0.index(i);
            va.write(ObjAttr0::new().with_style(ObjDisplayStyle::NotDisplayed));
        }

        let mut vy = 0;
        let (mut px, mut py): (i16, i16) = (32, 128);

        loop {
            let mut obj_attr = ObjAttr::new();
            obj_attr.0 = ObjAttr0::new().with_y(py as u16 - 8).with_bpp8(true);
            obj_attr.1 = ObjAttr1::new().with_x(px as u16);
            obj_attr.2 = ObjAttr2::new();
            mmio::OBJ_ATTR_ALL.index(0).write(obj_attr);

            mmio::DISPCNT.write(DisplayControl::new().with_show_bg0(true).with_show_obj(true));

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

            let key_input = rx_state.key_inputs()[0];

            let mut vx = 0;
            if key_input.left() {
                vx -= 8;
            }
            if key_input.right() {
                vx += 8;
            }

            if py == 128 {
                vy = if key_input.up() { -8 } else { 0 };
            } else {
                vy += 1;
            }

            px = cmp::min(cmp::max(0, px + vx), 232);
            py = cmp::min(cmp::max(8, py + vy), 128);
        }
    }
}

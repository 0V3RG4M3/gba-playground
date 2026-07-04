use core::cmp;
use core::iter;

use gba::bios;
use gba::interrupts::IrqBits;
use gba::mmio;
use gba::video::obj::{ObjAttr, ObjAttr0, ObjAttr1, ObjAttr2, ObjAttrWriteExt, ObjDisplayStyle};
use gba::video::{BackgroundControl, Color, DisplayControl, DisplayStatus, TextEntry};

use crate::egj2026::backgrounds;
use crate::egj2026::context::Context;
use crate::egj2026::link;
use crate::egj2026::sprites;
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

        mmio::DISPCNT.write(DisplayControl::new());

        mmio::BG_PALETTE.index(1).write(Color::BLACK);
        mmio::BG_PALETTE.index(2).write(Color::WHITE);

        mmio::CHARBLOCK0_8BPP.index(0).write([0x01010101; 16]);
        mmio::CHARBLOCK0_8BPP.index(1).write([0x02020202; 16]);
        /*let screenblock = mmio::TEXT_SCREENBLOCKS.get_frame(24).unwrap();
        for y in 0..32 {
            for x in 0..32 {
                let tile = if y < 10 { 1 } else { 0 };
                screenblock.index(x, y).write(TextEntry::new().with_tile(tile));
            }
        }*/
        let bg0cnt = BackgroundControl::new()
            .with_priority(2)
            .with_charblock(0)
            .with_bpp8(true)
            .with_screenblock(24)
            .with_size(1);
        mmio::BG0CNT.write(bg0cnt);
        let bg1cnt = BackgroundControl::new()
            .with_priority(1)
            .with_charblock(1)
            .with_bpp8(true)
            .with_screenblock(26)
            .with_size(1);
        mmio::BG1CNT.write(bg1cnt);
        let bg2cnt = BackgroundControl::new()
            .with_priority(0)
            .with_charblock(2)
            .with_bpp8(true)
            .with_screenblock(28)
            .with_size(1);
        mmio::BG2CNT.write(bg2cnt);

        mmio::OBJ_TILES.index(0).write([0x01010101; 8]);
        mmio::OBJ_TILES.index(1).write([0x01010101; 8]);
        for i in 0..128 {
            let va = mmio::OBJ_ATTR0.index(i);
            va.write(ObjAttr0::new().with_style(ObjDisplayStyle::NotDisplayed));
        }

        backgrounds::load();
        sprites::load();

        let player = Player { px: 32, py: 0, vy: 0, hflip: false, animation: Animation::Idle(0) };
        let mut players = [player; 2];
        let mut parent = true;

        loop {
            for (i, player) in players.iter().enumerate() {
                let vflip = parent != (i == 0);
                let py = if vflip { 80 + player.py } else { 48 - player.py };
                let mut obj_attr = ObjAttr::new();
                obj_attr.0 = ObjAttr0::new().with_y(py as u16).with_bpp8(true);
                obj_attr.1 = ObjAttr1::new()
                    .with_x(player.px as u16)
                    .with_hflip(player.hflip)
                    .with_vflip(vflip)
                    .with_size(2);
                obj_attr.2 = ObjAttr2::new().with_tile_id(player.tile_id());
                mmio::OBJ_ATTR_ALL.index(i).write(obj_attr);
            }

            let dispcnt = DisplayControl::new()
                .with_obj_vram_1d(true)
                .with_show_bg0(true)
                .with_show_bg1(true)
                .with_show_bg2(true)
                .with_show_obj(true);
            mmio::DISPCNT.write(dispcnt);

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

            parent = rx_state.parent();

            let key_inputs = rx_state.key_inputs();
            for (key_input, player) in iter::zip(&key_inputs, &mut players) {
                let Player { px, py, vy, hflip, animation } = player;

                let mut vx = 0;
                if key_input.left() {
                    vx -= 4;
                }
                if key_input.right() {
                    vx += 4;
                }

                *hflip = match vx {
                    1.. => false,
                    ..=-1 => true,
                    _ => *hflip,
                };

                if *py == 0 {
                    *vy = if key_input.up() { 8 } else { 0 };
                } else {
                    *vy -= 1;
                }

                *px = cmp::min(cmp::max(0, *px + vx), 232);
                *py = cmp::min(cmp::max(0, *py + *vy), 72);

                *animation = match (py, vx, vy, *animation) {
                    (0, 0, 0, Animation::Idle(mut frame)) => {
                        frame += 1;
                        frame %= 40;
                        Animation::Idle(frame)
                    }
                    (0, 0, 0, _) => Animation::Idle(0),
                    (0, _, 0, Animation::Run(mut frame)) => {
                        frame += 1;
                        frame %= 8;
                        Animation::Run(frame)
                    }
                    (0, _, 0, _) => Animation::Run(0),
                    (_, _, 1.., _) => Animation::Jump,
                    (_, _, _, _) => Animation::Fall,
                };
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Player {
    px: i16,
    py: i16,
    vy: i16,
    hflip: bool,
    animation: Animation,
}

impl Player {
    fn tile_id(&self) -> u16 {
        let tile_id = match self.animation {
            Animation::Idle(frame) => match frame / 4 {
                0 => sprites::INDEX_IDLE1,
                1 => sprites::INDEX_IDLE2,
                2 => sprites::INDEX_IDLE3,
                3 => sprites::INDEX_IDLE4,
                4 => sprites::INDEX_IDLE5,
                5 => sprites::INDEX_IDLE6,
                6 => sprites::INDEX_IDLE7,
                7 => sprites::INDEX_IDLE8,
                8 => sprites::INDEX_IDLE9,
                _ => sprites::INDEX_IDLE10,
            },
            Animation::Run(frame) => match frame / 4 {
                0 => sprites::INDEX_RUN1,
                1 => sprites::INDEX_RUN2,
                2 => sprites::INDEX_RUN3,
                3 => sprites::INDEX_RUN4,
                4 => sprites::INDEX_RUN5,
                5 => sprites::INDEX_RUN6,
                6 => sprites::INDEX_RUN7,
                _ => sprites::INDEX_RUN8,
            },
            Animation::Jump => sprites::INDEX_JUMP,
            Animation::Fall => sprites::INDEX_FALL,
        };
        tile_id as u16
    }
}

#[derive(Clone, Copy, Debug)]
enum Animation {
    Idle(u8),
    Run(u8),
    Jump,
    Fall,
}

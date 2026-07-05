use core::cmp;
use core::iter;

use gba::bios;
use gba::gba_cell::GbaCell;
use gba::interrupts::IrqBits;
use gba::mmio;
use gba::video::obj::{ObjAttr, ObjAttr0, ObjAttr1, ObjAttr2, ObjAttrWriteExt, ObjDisplayStyle};
use gba::video::{BackgroundControl, DisplayControl, DisplayStatus};

use crate::egj2026::backgrounds;
use crate::egj2026::context::Context;
use crate::egj2026::link;
use crate::egj2026::sfx_jump;
use crate::egj2026::sprites;
use crate::egj2026::tune0;
use crate::egj2026::tune1;
use crate::egj2026::tx_state::TxState;
use crate::gba_synth2;
use crate::math;
use crate::scene::{Scene, SceneRunner};

pub struct GameScene;

impl Scene for GameScene {
    type C = Context;

    fn new(_: &mut Self::C) -> Self {
        Self
    }

    fn run(&mut self, context: &mut Self::C) -> SceneRunner<Self::C> {
        mmio::DISPSTAT.write(DisplayStatus::new().with_irq_vblank(true).with_irq_hblank(true));
        mmio::IE.write(IrqBits::new().with_vblank(true).with_hblank(true).with_serial(true));
        mmio::IME.write(true);

        mmio::DISPCNT.write(DisplayControl::new());

        let bg0cnt = BackgroundControl::new()
            .with_priority(0)
            .with_charblock(2)
            .with_bpp8(true)
            .with_screenblock(27)
            .with_size(1);
        mmio::BG0CNT.write(bg0cnt);
        let bg1cnt = BackgroundControl::new()
            .with_priority(1)
            .with_charblock(1)
            .with_bpp8(true)
            .with_screenblock(25)
            .with_size(1);
        mmio::BG1CNT.write(bg1cnt);
        let bg2cnt = BackgroundControl::new()
            .with_priority(2)
            .with_charblock(0)
            .with_bpp8(true)
            .with_screenblock(24)
            .with_size(0);
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

        let mut is_synth_initialized = false;
        

        loop {
            for (i, player) in players.iter().enumerate() {
                let vflip = parent != (i == 0);
                let px = if vflip {
                    let px = 104 + player.px - players[i ^ 1].px;
                    match px {
                        -255..=-1 => (px + 512) as u16,
                        0..=255 => px as u16,
                        _ => 384,
                    }
                } else {
                    104
                };
                let py = if vflip { 80 + player.py } else { 48 - player.py };
                let mut obj_attr = ObjAttr::new();
                obj_attr.0 = ObjAttr0::new().with_y(py as u16).with_bpp8(true);
                obj_attr.1 = ObjAttr1::new()
                    .with_x(px)
                    .with_hflip(player.hflip)
                    .with_vflip(vflip)
                    .with_size(2);
                obj_attr.2 = ObjAttr2::new().with_tile_id(player.tile_id());
                mmio::OBJ_ATTR_ALL.index(vflip.into()).write(obj_attr);

                if vflip {
                    PLAYER_OFFSET.write(px);
                } else {
                    mmio::BG0HOFS.write(player.px as u16 / 4);
                    mmio::BG1HOFS.write(player.px as u16 / 8);
                    mmio::BG2HOFS.write(0);
                    BG_OFFSET.write(player.px as u16);
                }
            }

            let dispcnt = DisplayControl::new()
                .with_obj_vram_1d(true)
                .with_show_bg0(true)
                .with_show_bg1(true)
                .with_show_bg2(true)
                .with_show_obj(true);
            mmio::DISPCNT.write(dispcnt);

            let frame = FRAME.read();
            let frame = (frame + 1) % 512;
            FRAME.write(frame);

            for (i, offset) in OFFSETS.iter().enumerate() {
                let u = US[i];
                let a0 = u as u16 + FRAME.read();
                let a1 = (u as u16 >> 8) + (FRAME.read() / 2);
                let sum = math::fast_sin(a0 as u8) + math::fast_sin(a1 as u8);
                let off = sum.into_raw() >> 16;
                let off = off * (i as i32 + 1);
                let off = off >> 16;
                offset.write(off as u16);
            }

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

            parent = rx_state.parent();
            if !is_synth_initialized {
                is_synth_initialized = true;
                if parent {
                    gba_synth2::init_synth(&tune0::TUNE_TRACK1, tune0::TUNE_SIZE, tune0::TUNE_LOOP_SIZE);
                } else {
                    gba_synth2::init_synth(&tune1::TUNE_TRACK1, tune1::TUNE_SIZE, tune1::TUNE_LOOP_SIZE);
                }
            }
            
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
                    if key_input.up() {
                        *vy = 8;
                        gba_synth2::trigger_sfx(
                            &sfx_jump::TUNE_TRACK1,
                            sfx_jump::TUNE_SIZE,
                            sfx_jump::TUNE_LOOP_SIZE,
                        );
                    } else {
                        *vy = 0;
                    }
                } else {
                    *vy -= 1;
                }

                *px = cmp::min(cmp::max(0, *px + vx), 4096);
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

            gba_synth2::play_step();
            gba_synth2::play_sound_effect();
            gba_synth2::write_to_registers();
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

#[unsafe(link_section = ".iwram")]
extern "C" fn irq_handler(irq_bits: IrqBits) {
    if irq_bits.serial() {
        link::process();
    }

    if !irq_bits.hblank() {
        return;
    }

    let line = mmio::VCOUNT.read() + 1;
    if line < 80 || line >= 160 {
        return;
    }

    let offset = OFFSETS[line as usize - 80].read();
    mmio::BG0HOFS.write(BG_OFFSET.read() / 4 + offset);
    mmio::BG1HOFS.write(BG_OFFSET.read() / 8 + offset);
    mmio::BG2HOFS.write((offset % 256) as u16);
    let obj_attr1 = mmio::OBJ_ATTR1.index(1).read();
    let obj_attr1 = obj_attr1.with_x(PLAYER_OFFSET.read() + offset as u16);
    mmio::OBJ_ATTR1.index(1).write(obj_attr1);
}

const fn us() -> [u16; 80] {
    let mut us = [0; _];
    let mut line = 0;
    while line < us.len() {
        let u = math::fast_recip(line as u8 + 1).into_raw() >> 16;
        let u = (u * 160) >> 6;
        us[line] = u as u16;
        line += 1;
    }
    us
}

static BG_OFFSET: GbaCell<u16> = GbaCell::new(0);
static PLAYER_OFFSET: GbaCell<u16> = GbaCell::new(0);
static FRAME: GbaCell<u16> = GbaCell::new(0);
static OFFSETS: [GbaCell<u16>; 80] = [const { GbaCell::new(0) }; 80];

const US: [u16; 80] = us();

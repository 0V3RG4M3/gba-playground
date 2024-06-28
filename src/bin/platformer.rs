#![no_std]
#![no_main]

use core::cmp;
use core::fmt::Write;

use gba::bios;
use gba::interrupts::IrqBits;
use gba::keys::KeyInput;
use gba::mgba::{MgbaBufferedLogger, MgbaMessageLevel};
use gba::mmio;
use gba::video::obj::{ObjAttr, ObjAttr0, ObjAttr1, ObjAttr2, ObjDisplayStyle};
use gba::video::{BackgroundControl, Color, DisplayControl, DisplayStatus, TextEntry};

use gba_playground::gba_synth;

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    if let Ok(mut logger) = MgbaBufferedLogger::try_new(MgbaMessageLevel::Fatal) {
        writeln!(logger, "{info}").ok();
    }
    loop {}
}

#[no_mangle]
extern "C" fn main() -> ! {
    mmio::DISPSTAT.write(DisplayStatus::new().with_irq_vblank(true));
    mmio::IE.write(IrqBits::VBLANK);
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

    gba_synth::init_synth();

    // let mut pitch = 60;
    // let mut vel = 64;
    let mut div_code: u16 = 0;
    let mut shift: u16 = 0;
    let mut key_was_pressed: KeyInput = KeyInput::new();
    let mut frame_id = 0;
    loop {
        bios::VBlankIntrWait();
        gba_synth::play_tune(frame_id);

        let mut obj_attr = ObjAttr::new();
        obj_attr.0 = ObjAttr0::new().with_y(py as u16 - 8).with_bpp8(true);
        obj_attr.1 = ObjAttr1::new().with_x(px as u16);
        obj_attr.2 = ObjAttr2::new();
        mmio::OBJ_ATTR_ALL.index(0).write(obj_attr);

        mmio::DISPCNT.write(DisplayControl::new().with_show_bg0(true).with_show_obj(true));

        let key_input = mmio::KEYINPUT.read();

        let mut vx = 0;
        if key_input.left() {
            // on press
            if !key_was_pressed.left() {
                vx -= 8;
                // pitch -= 1;
                // gba_synth::play_tone1(pitch, vel);
                div_code = (div_code + 8 - 1) % 8;
                // gba_synth::play_noise(div_code, shift, 63)
            }
        }

        if key_input.right() {
            // on press
            if !key_was_pressed.right() {
                vx += 8;
                // pitch += 1;
                // gba_synth::play_tone1(pitch, vel);
                div_code = (div_code + 1) % 8;
                // gba_synth::play_noise(div_code, shift, 63)
            }
        }

        if key_input.up() {
            // on press
            if !key_was_pressed.up() {
                // vel += 8;
                // gba_synth::play_tone1(pitch, vel);
                shift = (shift + 1) % 16;
                // gba_synth::play_noise(div_code, shift, 63)
            }
        }
        if key_input.down() {
            // on press
            if !key_was_pressed.down() {
                // vel -= 8;
                // gba_synth::play_tone1(pitch, vel);
                shift = (shift + 16 - 1) % 16;
                // gba_synth::play_noise(div_code, shift, 63)
            }
        }

        if py == 128 {
            vy = if key_input.up() { -8 } else { 0 };
        } else {
            vy += 1;
        }

        px = cmp::min(cmp::max(0, px + vx), 232);
        py = cmp::min(cmp::max(8, py + vy), 128);

        key_was_pressed = key_input;
        frame_id = (frame_id + 1) % gba_synth::get_tune_step_count();
    }
}

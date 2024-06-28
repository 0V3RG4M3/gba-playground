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
use gba_playground::log4gba;

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    if let Ok(mut logger) = MgbaBufferedLogger::try_new(MgbaMessageLevel::Fatal) {
        writeln!(logger, "{info}").ok();
    }
    loop {}
}

macro_rules! clamp {
    ($v:expr, $lo:expr, $hi:expr) => {
        cmp::min(cmp::max($lo, $v), $hi)
    };
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

    let (mut px, mut py): (i16, i16) = (32, 128);

    gba_synth::init_synth();

    let mut shift_5: u16 = 0; // x0
    let mut div_code_3: u16 = 0; // y0
    let mut length_6 = 0; // x1
    let mut step_time_3 = 0; // y1
    let mut counter7: bool = false;
    let mut stop_when_expired: bool = false;

    let mut key_was_pressed: KeyInput = KeyInput::new();
    let mut frame_id = 0;
    let (mut x0, mut y0) = (0, 0);
    let (mut x1, mut y1) = (0, 0);
    let mut z = 0; // l-r
    let mut ab = 0;

    const MAX_X0: i32 = 31;
    const MAX_Y0: i32 = 7;
    const MAX_X1: i32 = 63;
    const MAX_Y1: i32 = 7;
    const MAX_Z: i32 = 1;
    const MAX_AB: i32 = 3;
    let mut on_press = false;

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

        if (key_input.left() && !key_was_pressed.left())
            || (key_input.right() && !key_was_pressed.right())
        {
            let axis = key_input.right() as i32 - key_input.left() as i32;

            if z == 0 {
                x0 = clamp!(x0 + axis, 0, MAX_X0);
                shift_5 = x0 as u16;
            } else {
                x1 = clamp!(x1 + axis, 0, MAX_X1);
                length_6 = x1 as u16;
            }
            on_press = true;
        }

        if (key_input.up() && !key_was_pressed.up())
            || (key_input.down() && !key_was_pressed.down())
        {
            let axis = key_input.down() as i32 - key_input.up() as i32;

            if z == 0 {
                y0 = clamp!(y0 + axis, 0, MAX_Y0);
                div_code_3 = y0 as u16;
            } else {
                y1 = clamp!(y1 + axis, 0, MAX_Y1);
                step_time_3 = y1 as u16;
            }
            on_press = true;
        }

        if (key_input.l() && !key_was_pressed.l()) || (key_input.r() && !key_was_pressed.r()) {
            let axis = key_input.r() as i32 - key_input.l() as i32;
            z = clamp!(z + axis, 0, MAX_Z);
            on_press = true;
        }

        if (key_input.b() && !key_was_pressed.b()) || (key_input.a() && !key_was_pressed.a()) {
            let axis = key_input.a() as i32 - key_input.b() as i32;
            ab = clamp!(ab + axis, 0, MAX_AB);
            counter7 = (ab & 0b01) != 0;
            stop_when_expired = (ab & 0b10) != 0;
            on_press = true;
        }

        if z == 0 {
            px = 32 + 4 * x0 as i16;
            py = 32 + 16 * y0 as i16;
        } else {
            px = 32 + 2 * x1 as i16;
            py = 32 + 16 * y1 as i16;
        }
        if on_press {
            mmio::BG_PALETTE.index(1).write(if counter7 { Color::BLUE } else { Color::BLACK });
            mmio::BG_PALETTE.index(2).write(if stop_when_expired {
                Color::WHITE
            } else {
                Color::BLACK
            });
            mmio::OBJ_PALETTE.index(1).write(if z == 0 { Color::MAGENTA } else { Color::GREEN });

            // use these logs to create a sound design in
            log4gba::debug("play_noise");
            log4gba::debug(shift_5);
            log4gba::debug(div_code_3);
            log4gba::debug(counter7);
            log4gba::debug(stop_when_expired);
            log4gba::debug(length_6);
            log4gba::debug(step_time_3);
            log4gba::debug("-----");

            gba_synth::play_noise(
                shift_5,
                div_code_3,
                counter7,
                stop_when_expired,
                length_6,
                step_time_3,
                127,
            );
            on_press = false;
        }
        key_was_pressed = key_input;
        frame_id = (frame_id + 1) % gba_synth::get_tune_step_count();
    }
}

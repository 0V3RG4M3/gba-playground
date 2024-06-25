#![no_std]
#![no_main]

use core::fmt::Write;

use gba::asm_runtime;
use gba::bios;
use gba::fixed::{i16fx8, i32fx8};
use gba::gba_cell::GbaCell;
use gba::interrupts::IrqBits;
use gba::mgba::{MgbaBufferedLogger, MgbaMessageLevel};
use gba::mmio;
use gba::video::{BackgroundControl, Color, DisplayControl, DisplayStatus, VideoMode};

static CAM_X: GbaCell<i32> = GbaCell::new(64 << 8);
static CAM_Y: GbaCell<i32> = GbaCell::new(16 << 8);
static CAM_Z: GbaCell<i32> = GbaCell::new(64 << 8);
static CAM_YAW_SIN: GbaCell<i32> = GbaCell::new(98);
static CAM_YAW_COS: GbaCell<i32> = GbaCell::new(237);

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    if let Ok(mut logger) = MgbaBufferedLogger::try_new(MgbaMessageLevel::Fatal) {
        writeln!(logger, "{info}").ok();
    }
    loop {}
}

#[no_mangle]
extern "C" fn main() -> ! {
    mmio::DISPSTAT.write(DisplayStatus::new().with_irq_vblank(true).with_irq_hblank(true));
    mmio::IE.write(IrqBits::new().with_vblank(true).with_hblank(true));
    mmio::IME.write(true);

    mmio::BG_PALETTE.index(1).write(Color::from_rgb(12, 17, 31));
    mmio::BG_PALETTE.index(2).write(Color::from_rgb(31, 22, 0));
    mmio::BG_PALETTE.index(3).write(Color::from_rgb(27, 4, 15));

    let mut tile = [0; 16];
    for (i, value) in tile.iter_mut().enumerate() {
        *value = match (i % 2 == 0, i < 8) {
            (true, true) => 0x01010101,
            (false, false) => 0x02020202,
            _ => 0x03030303,
        };
    }
    mmio::CHARBLOCK0_8BPP.index(0).write(tile);

    let mut cam_yaw_angle = 0;

    loop {
        bios::VBlankIntrWait();

        let key_input = mmio::KEYINPUT.read();

        cam_yaw_angle -= key_input.l() as u8;
        cam_yaw_angle += key_input.r() as u8;
        cam_yaw_angle = cam_yaw_angle % 64;
        let cam_yaw_sin = sin(cam_yaw_angle);
        let cam_yaw_cos = cos(cam_yaw_angle);
        CAM_YAW_SIN.write(cam_yaw_sin);
        CAM_YAW_COS.write(cam_yaw_cos);

        let mut cam_x = CAM_X.read();
        cam_x -= (key_input.left() as i32) * cam_yaw_cos;
        cam_x += (key_input.right() as i32) * cam_yaw_cos;
        cam_x += (key_input.up() as i32) * cam_yaw_sin;
        cam_x -= (key_input.down() as i32) * cam_yaw_sin;
        CAM_X.write(cam_x);

        let mut cam_z = CAM_Z.read();
        cam_z -= (key_input.left() as i32) * cam_yaw_sin;
        cam_z += (key_input.right() as i32) * cam_yaw_sin;
        cam_z -= (key_input.up() as i32) * cam_yaw_cos;
        cam_z += (key_input.down() as i32) * cam_yaw_cos;
        CAM_Z.write(cam_z);

        let mut cam_y = CAM_Y.read();
        cam_y -= (key_input.b() as i32) << 8;
        cam_y += (key_input.a() as i32) << 8;
        cam_y = cam_y.max(1 << 8);
        CAM_Y.write(cam_y);

        asm_runtime::RUST_IRQ_HANDLER.write(Some(irq_handler));

        //mmio::BG2CNT.write(BackgroundControl::new().with_charblock(1));
        mmio::BG2CNT.write(BackgroundControl::new().with_screenblock(1)); // FIXME

        let display_control =
            DisplayControl::new().with_video_mode(VideoMode::_2).with_show_bg2(true);
        mmio::DISPCNT.write(display_control);
    }
}

#[link_section = ".iwram"]
extern "C" fn irq_handler(irq_bits: IrqBits) {
    if !irq_bits.hblank() {
        return;
    }

    let vcount = mmio::VCOUNT.read();
    let lambda = CAM_Y.read() / (vcount as i32 + 1);
    let yaw_sin = CAM_YAW_SIN.read();
    let yaw_cos = CAM_YAW_COS.read();

    let x = CAM_X.read() - (((yaw_cos * 120 - yaw_sin * 128) * lambda) >> 8);
    let y = CAM_Z.read() - (((yaw_sin * 120 + yaw_cos * 128) * lambda) >> 8);
    mmio::BG2X.write(i32fx8::from_raw(x));
    mmio::BG2Y.write(i32fx8::from_raw(y));

    let pa = (lambda * yaw_cos) >> 8;
    let pc = (lambda * yaw_sin) >> 8;
    mmio::BG2PA.write(i16fx8::from_raw(pa as i16));
    mmio::BG2PC.write(i16fx8::from_raw(pc as i16));

    /*let vcount = mmio::VCOUNT.read();
    if vcount < 79 {
        return;
    } else if vcount == 79 {
        mmio::BG2CNT.write(BackgroundControl::new().with_screenblock(1));
    }*/
}

fn sin(angle: u8) -> i32 {
    match angle {
        0 => 0,
        1 => 25,
        2 => 50,
        3 => 74,
        4 => 98,
        5 => 121,
        6 => 142,
        7 => 162,
        8 => 181,
        9 => 198,
        10 => 213,
        11 => 226,
        12 => 237,
        13 => 245,
        14 => 251,
        15 => 255,
        16 => 256,
        17..=31 => sin(32 - angle),
        32..=63 => -sin(angle - 32),
        _ => panic!(),
    }
}

fn cos(angle: u8) -> i32 {
    match angle {
        0..=47 => sin(angle + 16),
        48..=63 => sin(angle - 48),
        _ => panic!(),
    }
}

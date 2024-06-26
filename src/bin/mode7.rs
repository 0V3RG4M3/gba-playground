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

use gba_playground::math;

static CAM_X: GbaCell<i32> = GbaCell::new(64 << 8);
static CAM_Y: GbaCell<i32> = GbaCell::new(16 << 8);
static CAM_Z: GbaCell<i32> = GbaCell::new(64 << 8);
static CAM_YAW_SIN: GbaCell<i32> = GbaCell::new(0 << 8);
static CAM_YAW_COS: GbaCell<i32> = GbaCell::new(1 << 8);
static CAM_PITCH_SIN: GbaCell<i32> = GbaCell::new(0 << 8);
static CAM_PITCH_COS: GbaCell<i32> = GbaCell::new(1 << 8);
static HORIZON: GbaCell<i32> = GbaCell::new(160);

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
    let mut cam_pitch_angle = 0;

    loop {
        bios::VBlankIntrWait();

        let key_input = mmio::KEYINPUT.read();

        cam_yaw_angle -= key_input.left() as u8;
        cam_yaw_angle += key_input.right() as u8;
        let cam_yaw_sin = math::fast_sin30(cam_yaw_angle) >> 22;
        let cam_yaw_cos = math::fast_cos30(cam_yaw_angle) >> 22;
        CAM_YAW_SIN.write(cam_yaw_sin);
        CAM_YAW_COS.write(cam_yaw_cos);

        cam_pitch_angle -= key_input.up() as u8;
        cam_pitch_angle += key_input.down() as u8;
        cam_pitch_angle = 128 + (cam_pitch_angle + 128).clamp(64, 192);
        let cam_pitch_sin = math::fast_sin30(cam_pitch_angle) >> 22;
        let cam_pitch_cos = math::fast_cos30(cam_pitch_angle) >> 22;
        CAM_PITCH_SIN.write(cam_pitch_sin);
        CAM_PITCH_COS.write(cam_pitch_cos);

        let mut cam_x = CAM_X.read();
        cam_x += (key_input.a() as i32) * cam_yaw_sin;
        cam_x -= (key_input.b() as i32) * cam_yaw_sin;
        CAM_X.write(cam_x);

        let mut cam_z = CAM_Z.read();
        cam_z -= (key_input.a() as i32) * cam_yaw_cos;
        cam_z += (key_input.b() as i32) * cam_yaw_cos;
        CAM_Z.write(cam_z);

        let mut cam_y = CAM_Y.read();
        cam_y -= (key_input.l() as i32) << 8;
        cam_y += (key_input.r() as i32) << 8;
        cam_y = cam_y.max(1 << 8);
        CAM_Y.write(cam_y);

        let horizon = if cam_pitch_cos == 0 {
            if cam_pitch_sin > 0 { 0 } else { 160 }
        } else {
            (80 - ((768 * cam_pitch_sin - cam_y) * 256) / (768 * cam_pitch_cos)).clamp(0, 160)
        };
        HORIZON.write(horizon);

        mmio::BG2CNT.write(BackgroundControl::new().with_charblock(1));

        process_line(0);

        asm_runtime::RUST_IRQ_HANDLER.write(Some(irq_handler));

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
    process_line(vcount as i32 + 1);
}

#[link_section = ".iwram"]
fn process_line(line: i32) {
    if line < HORIZON.read() || line >= 160 {
        return;
    } else {
        mmio::BG2CNT.write(BackgroundControl::new().with_screenblock(1));
    }

    let yaw_sin = CAM_YAW_SIN.read();
    let yaw_cos = CAM_YAW_COS.read();
    let pitch_sin = CAM_PITCH_SIN.read();
    let pitch_cos = CAM_PITCH_COS.read();
    let by = (line - 80) * pitch_cos + 256 * pitch_sin;
    let bz = (line - 80) * pitch_sin - 256 * pitch_cos;
    //let lambda = (CAM_Y.read() * (math::fast_recip30(vcount as u8 + 1) >> 14)) >> 12;
    let lambda = (CAM_Y.read() << 12) / by.max(1);

    let pa = (lambda * yaw_cos) >> 8;
    let pc = (lambda * yaw_sin) >> 8;
    mmio::BG2PA.write(i16fx8::from_raw((pa >> 4) as i16));
    mmio::BG2PC.write(i16fx8::from_raw((pc >> 4) as i16));

    let x = CAM_X.read() - (120 * (pa >> 4)) - ((bz * pc) >> 12);
    let y = CAM_Z.read() - (120 * (pc >> 4)) + ((bz * pa) >> 12);
    mmio::BG2X.write(i32fx8::from_raw(x));
    mmio::BG2Y.write(i32fx8::from_raw(y));
}

#![no_std]
#![no_main]

use core::fmt::Write;

use gba::asm_runtime;
use gba::bios;
use gba::interrupts::IrqBits;
use gba::mgba::{MgbaBufferedLogger, MgbaMessageLevel};
use gba::mmio;
use gba::video::{BackgroundControl, Color, DisplayControl, DisplayStatus, VideoMode};

use gba_playground::fixed::Fixed;
use gba_playground::mode7::{self, Camera};

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

    let mut camera = Camera::new();

    loop {
        bios::VBlankIntrWait();

        let key_input = mmio::KEYINPUT.read();

        let mut cam_yaw_angle = camera.yaw_angle();
        cam_yaw_angle -= key_input.left() as u8;
        cam_yaw_angle += key_input.right() as u8;
        camera.set_yaw_angle(cam_yaw_angle);

        let mut cam_pitch_angle = camera.pitch_angle();
        cam_pitch_angle -= key_input.up() as u8;
        cam_pitch_angle += key_input.down() as u8;
        cam_pitch_angle = 128 + (cam_pitch_angle + 128).clamp(64, 192);
        camera.set_pitch_angle(cam_pitch_angle);

        camera.x += camera.yaw_sin() * (key_input.a() as i32);
        camera.x -= camera.yaw_sin() * (key_input.b() as i32);

        camera.z -= camera.yaw_cos() * (key_input.a() as i32);
        camera.z += camera.yaw_cos() * (key_input.b() as i32);

        camera.y -= Fixed::from_raw((key_input.l() as i32) << 20);
        camera.y += Fixed::from_raw((key_input.r() as i32) << 20);
        camera.y = Fixed::from_raw(camera.y.into_raw().max(1 << 20));

        mmio::BG2CNT.write(BackgroundControl::new().with_charblock(1));

        mode7::prepare_frame(&camera);
        mode7::process_line(0);

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
    mode7::process_line(vcount as i32 + 1);
}

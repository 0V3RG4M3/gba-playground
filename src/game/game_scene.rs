use gba::asm_runtime;
use gba::bios;
use gba::interrupts::IrqBits;
use gba::mmio;
use gba::video::{BackgroundControl, Color, DisplayControl, DisplayStatus, VideoMode};

use crate::mode7::{self, Camera};
use crate::scene::{Scene, SceneRunner};

pub struct GameScene {}

impl Scene for GameScene {
    type C = ();

    fn new(_: &mut ()) -> GameScene {
        GameScene {}
    }

    fn run(&mut self, _: &mut ()) -> SceneRunner<()> {
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
        camera.set_pitch_angle(16);

        loop {
            bios::VBlankIntrWait();

            let key_input = mmio::KEYINPUT.read();

            let mut cam_yaw_angle = camera.yaw_angle();
            cam_yaw_angle -= key_input.l() as u8;
            cam_yaw_angle += key_input.r() as u8;
            camera.set_yaw_angle(cam_yaw_angle);

            camera.x += camera.yaw_sin() * (key_input.up() as i32);
            camera.x -= camera.yaw_sin() * (key_input.down() as i32);
            camera.x -= camera.yaw_cos() * (key_input.left() as i32);
            camera.x += camera.yaw_cos() * (key_input.right() as i32);

            camera.z -= camera.yaw_cos() * (key_input.up() as i32);
            camera.z += camera.yaw_cos() * (key_input.down() as i32);
            camera.z -= camera.yaw_sin() * (key_input.left() as i32);
            camera.z += camera.yaw_sin() * (key_input.right() as i32);

            mmio::BG2CNT.write(BackgroundControl::new().with_charblock(1));

            mode7::prepare_frame(&camera);
            mode7::process_line(0);

            asm_runtime::RUST_IRQ_HANDLER.write(Some(irq_handler));

            let display_control =
                DisplayControl::new().with_video_mode(VideoMode::_2).with_show_bg2(true);
            mmio::DISPCNT.write(display_control);
        }
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

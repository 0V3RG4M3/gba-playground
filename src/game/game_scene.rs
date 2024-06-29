use gba::asm_runtime;
use gba::bios;
use gba::fixed::i16fx8;
use gba::interrupts::IrqBits;
use gba::mmio;
use gba::video::obj::{ObjAttr, ObjAttr0, ObjAttr1, ObjAttr2, ObjDisplayStyle, ObjShape};
use gba::video::{BackgroundControl, Color, DisplayControl, DisplayStatus, VideoMode};

use crate::fixed::Fixed;
use crate::mode7::{self, Camera, Sprite};
use crate::scene::{Scene, SceneRunner};
use crate::sprites;
use crate::vec3::Vec3;

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

        mmio::OBJ_TILES.index(0).write([0x01010101; 8]);
        mmio::OBJ_TILES.index(1).write([0x01010101; 8]);
        for i in 1..128 {
            let va = mmio::OBJ_ATTR0.index(i);
            va.write(ObjAttr0::new().with_style(ObjDisplayStyle::NotDisplayed));
        }

        sprites::load_sprites();

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

        let pos =
            Vec3::<i32, 8> { x: Fixed::from_int(8), y: Fixed::from_int(2), z: Fixed::from_int(8) };
        let mut obj_attr = ObjAttr::new();
        obj_attr.0 = ObjAttr0::new().with_bpp8(true).with_shape(ObjShape::Horizontal);
        obj_attr.1 = ObjAttr1::new();
        obj_attr.2 = ObjAttr2::new();
        let mut sprite = Sprite { obj: obj_attr, pos: pos, scale: Fixed::from_int(1) };

        loop {
            bios::VBlankIntrWait();

            let key_input = mmio::KEYINPUT.read();

            let mut cam_yaw_angle = camera.yaw_angle();
            cam_yaw_angle -= key_input.l() as u8;
            cam_yaw_angle += key_input.r() as u8;
            camera.set_yaw_angle(cam_yaw_angle);

            camera.pos.x += camera.yaw_sin() * (key_input.up() as i32);
            camera.pos.x -= camera.yaw_sin() * (key_input.down() as i32);
            camera.pos.x -= camera.yaw_cos() * (key_input.left() as i32);
            camera.pos.x += camera.yaw_cos() * (key_input.right() as i32);

            camera.pos.z -= camera.yaw_cos() * (key_input.up() as i32);
            camera.pos.z += camera.yaw_cos() * (key_input.down() as i32);
            camera.pos.z -= camera.yaw_sin() * (key_input.left() as i32);
            camera.pos.z += camera.yaw_sin() * (key_input.right() as i32);

            mmio::BG2CNT.write(BackgroundControl::new().with_charblock(1));

            mode7::prepare_frame(&camera);

            mode7::prepare_sprite(&camera, &mut sprite);
            mmio::OBJ_ATTR_ALL.index(0).write(sprite.obj);
            let scale = i16fx8::from_raw(sprite.scale.into_raw() as i16);
            mmio::AFFINE_PARAM_A.index(0).write(scale);
            mmio::AFFINE_PARAM_D.index(0).write(scale);

            mode7::process_line(0);

            asm_runtime::RUST_IRQ_HANDLER.write(Some(irq_handler));

            let display_control = DisplayControl::new()
                .with_video_mode(VideoMode::_2)
                .with_show_bg2(true)
                .with_show_obj(true);
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

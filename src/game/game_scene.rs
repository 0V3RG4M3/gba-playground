use gba::asm_runtime;
use gba::bios;
use gba::fixed::i16fx8;
use gba::interrupts::IrqBits;
use gba::mmio;
use gba::video::obj::{ObjAttr0, ObjDisplayStyle};
use gba::video::{BackgroundControl, Color, DisplayControl, DisplayStatus, VideoMode};

use crate::fixed::Fixed;
use crate::game::item::ItemState;
use crate::game::leader::Leader;
use crate::game::level::Level;
use crate::game::levels;
use crate::mode7::{self, Camera};
use crate::scene::{Scene, SceneRunner};
use crate::sprites;

pub struct GameScene {}

impl GameScene {
    fn run_level<const A: usize, const R: usize>(&mut self, level: Level<A, R>) -> ! {
        let mut items = level.available_items;
        let recipe_items = level.recipe_items;

        let mut leader = Leader::new(96, 32);

        let mut camera = Camera::new();
        camera.set_pitch_angle(16);

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

            let equipped_item_index = items
                .iter()
                .enumerate()
                .find(|(_, item)| item.state == ItemState::EquippedByPlayer)
                .map(|(i, _)| i);
            if let Some(equipped_item_index) = equipped_item_index {
                if key_input.b() && !key_input.a() {
                    let item = &mut items[equipped_item_index];
                    let pos = &mut item.sprite.pos;
                    pos.x = camera.pos.x + camera.yaw_sin() * 32;
                    pos.z = camera.pos.z - camera.yaw_cos() * 32;
                    item.state = ItemState::Available;
                }
            } else {
                if key_input.a() && !key_input.b() {
                    for item in &mut items {
                        if item.state != ItemState::Available {
                            continue;
                        }
                        let mut pos = item.sprite.pos - camera.pos;
                        pos.y = Fixed::from_int(0);
                        let sq_dist = pos.dot(pos);
                        if sq_dist.into_int() < 32 * 32 {
                            item.state = ItemState::EquippedByPlayer;
                            break;
                        }
                    }
                }
            }

            let _ = leader.process(&mut items, &recipe_items);

            mmio::BG2CNT.write(BackgroundControl::new().with_charblock(1));

            mode7::prepare_frame(&camera);

            for (i, item) in items.iter_mut().enumerate() {
                let sprite = &mut item.sprite;
                if item.state == ItemState::Available {
                    mode7::prepare_sprite(&camera, sprite);
                    let affine_index = sprite.obj.1.affine_index() as usize;
                    let scale = i16fx8::from_raw(sprite.scale.into_raw() as i16);
                    mmio::AFFINE_PARAM_A.index(affine_index).write(scale);
                    mmio::AFFINE_PARAM_D.index(affine_index).write(scale);
                } else if item.state == ItemState::EquippedByPlayer {
                    sprite.obj.0 = sprite.obj.0.with_style(ObjDisplayStyle::Normal);
                    sprite.obj.1 = sprite.obj.1.with_x(0);
                    sprite.obj.0 = sprite.obj.0.with_y(0);
                } else {
                    sprite.obj.0 = sprite.obj.0.with_style(ObjDisplayStyle::NotDisplayed);
                }
                mmio::OBJ_ATTR_ALL.index(i).write(sprite.obj);
            }

            let sprite = &mut leader.sprite;
            mode7::prepare_sprite(&camera, sprite);
            let affine_index = sprite.obj.1.affine_index() as usize;
            let scale = i16fx8::from_raw(sprite.scale.into_raw() as i16);
            mmio::AFFINE_PARAM_A.index(affine_index).write(scale);
            mmio::AFFINE_PARAM_D.index(affine_index).write(scale);
            mmio::OBJ_ATTR_ALL.index(31).write(sprite.obj);

            mode7::process_line(0);

            asm_runtime::RUST_IRQ_HANDLER.write(Some(irq_handler));

            let display_control = DisplayControl::new()
                .with_video_mode(VideoMode::_2)
                .with_show_bg2(true)
                .with_show_obj(true)
                .with_obj_vram_1d(true);
            mmio::DISPCNT.write(display_control);
        }
    }
}

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
        for i in 0..128 {
            let va = mmio::OBJ_ATTR0.index(i);
            va.write(ObjAttr0::new().with_style(ObjDisplayStyle::NotDisplayed));
        }

        sprites::load();

        let mut tile = [0; 16];
        for (i, value) in tile.iter_mut().enumerate() {
            *value = match (i % 2 == 0, i < 8) {
                (true, true) => 0x01010101,
                (false, false) => 0x02020202,
                _ => 0x03030303,
            };
        }
        mmio::CHARBLOCK0_8BPP.index(0).write(tile);

        self.run_level(levels::first())
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

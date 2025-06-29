use gba::keys::KeyInput;

use crate::egj2025::item::Item;
use crate::fixed::Fixed;
use crate::gba_synth;
use crate::mode7::Camera;
use crate::sfx;

pub struct Player {
    item_index: Option<usize>,
    key_was_pressed: KeyInput,
}

impl Player {
    pub fn new() -> Player {
        Player { item_index: None, key_was_pressed: KeyInput::new() }
    }

    pub fn item_index(&self) -> Option<usize> {
        self.item_index
    }

    pub fn process(&mut self, items: &mut [Item], camera: &mut Camera, key_input: &KeyInput) {
        let mut cam_yaw_angle = camera.yaw_angle();
        cam_yaw_angle -= key_input.left() as u8;
        cam_yaw_angle += key_input.right() as u8;
        camera.set_yaw_angle(cam_yaw_angle);

        camera.pos.x += camera.yaw_sin() * (key_input.up() as i32);
        camera.pos.x -= camera.yaw_sin() * (key_input.down() as i32);
        camera.pos.x -= camera.yaw_cos() * (key_input.l() as i32);
        camera.pos.x += camera.yaw_cos() * (key_input.r() as i32);

        camera.pos.z -= camera.yaw_cos() * (key_input.up() as i32);
        camera.pos.z += camera.yaw_cos() * (key_input.down() as i32);
        camera.pos.z -= camera.yaw_sin() * (key_input.l() as i32);
        camera.pos.z += camera.yaw_sin() * (key_input.r() as i32);

        match self.item_index {
            Some(item_index) => {
                if key_input.b() && !self.key_was_pressed.b() {
                    gba_synth::play_sfx(sfx::ITEM_DROPPED);

                    let item = &mut items[item_index];
                    let pos = &mut item.sprite.pos;
                    pos.x = camera.pos.x + camera.yaw_sin() * 32;
                    pos.z = camera.pos.z - camera.yaw_cos() * 32;
                    self.item_index = None;
                }
            }
            None => {
                if key_input.a() && !self.key_was_pressed.a() {
                    for (i, item) in items.iter().enumerate() {
                        let mut pos = item.sprite.pos - camera.pos;
                        pos.y = Fixed::from_int(0);
                        let sq_dist = pos.dot(pos);
                        if sq_dist.into_int() < 32 * 32 {
                            gba_synth::play_sfx(sfx::ITEM_COLLECTED);
                            self.item_index = Some(i);
                            break;
                        } else {
                            gba_synth::play_sfx(sfx::CANT_TAKE);
                        }
                    }
                }
            }
        }
        self.key_was_pressed = *key_input;
    }
}

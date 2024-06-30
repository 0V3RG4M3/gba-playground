use gba::keys::KeyInput;
use gba::video::obj::ObjDisplayStyle;

use crate::fixed::Fixed;
use crate::game::cauldron::Cauldron;
use crate::game::item::{Item, ItemKind, ItemState};
use crate::mode7::Camera;

pub struct Player {
    index: usize,
}

impl Player {
    pub fn new() -> Player {
        Player { index: 0 }
    }

    pub fn process<const A: usize, const R: usize>(
        &mut self,
        items: &mut [Item; A],
        recipe_items: &[ItemKind; R],
        cauldron: &Cauldron,
        camera: &mut Camera,
        key_input: &KeyInput,
    ) -> Result<(), ()> {
        if self.index >= R {
            return Ok(());
        }

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
            if key_input.a() && !key_input.b() {
                if cauldron.sprite.obj.0.style() != ObjDisplayStyle::NotDisplayed {
                    let mut pos = cauldron.sprite.pos - camera.pos;
                    pos.y = Fixed::from_int(0);
                    let sq_dist = pos.dot(pos);
                    if sq_dist.into_int() < 32 * 32 {
                        let item = &mut items[equipped_item_index];
                        item.state = ItemState::ConsumedByPlayer;
                        if item.kind != recipe_items[self.index] {
                            return Err(());
                        }
                        self.index += 1;
                    }
                }
            }
            if key_input.b() && !key_input.a() {
                let item = &mut items[equipped_item_index];
                let pos = &mut item.sprite.pos;
                pos.x = camera.pos.x + camera.yaw_sin() * 32;
                pos.z = camera.pos.z - camera.yaw_cos() * 32;
                item.state = ItemState::Available;
            }
        } else {
            if key_input.a() && !key_input.b() {
                for item in items {
                    if item.state != ItemState::Available {
                        continue;
                    }
                    if item.sprite.obj.0.style() == ObjDisplayStyle::NotDisplayed {
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
        Ok(())
    }
}

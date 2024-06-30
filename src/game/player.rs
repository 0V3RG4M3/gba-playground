use crate::gba_synth;
use crate::sfx;
use gba::keys::KeyInput;
use gba::video::obj::ObjDisplayStyle;

use crate::fixed::Fixed;
use crate::game::cauldron::Cauldron;
use crate::game::item::{Item, ItemKind, ItemState};
use crate::mode7::Camera;
use crate::sprites;

pub struct Player {
    index: usize,
    key_was_pressed: KeyInput,
}

impl Player {
    pub fn new() -> Player {
        Player { index: 0, key_was_pressed: KeyInput::new() }
    }

    pub fn process<const A: usize, const R: usize>(
        &mut self,
        items: &mut [Item; A],
        recipe_items: &[ItemKind; R],
        cauldron: &mut Cauldron,
        camera: &mut Camera,
        key_input: &KeyInput,
    ) -> Result<bool, ()> {
        if self.index >= R {
            cauldron.sprite.obj.2 =
                cauldron.sprite.obj.2.with_tile_id(sprites::INDEX_10_CALDRON_FIRE as u16);
            return Ok(true);
        }

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

        let equipped_item_index = items
            .iter()
            .enumerate()
            .find(|(_, item)| item.state == ItemState::EquippedByPlayer)
            .map(|(i, _)| i);
        if let Some(equipped_item_index) = equipped_item_index {
            if key_input.a() && !self.key_was_pressed.a() {
                if cauldron.sprite.obj.0.style() != ObjDisplayStyle::NotDisplayed {
                    let mut pos = cauldron.sprite.pos - camera.pos;
                    pos.y = Fixed::from_int(0);
                    let sq_dist = pos.dot(pos);
                    if sq_dist.into_int() < 32 * 32 {
                        let item = &mut items[equipped_item_index];
                        item.state = ItemState::ConsumedByPlayer;
                        if item.kind != recipe_items[self.index] {
                            // Player places bad item in cauldron
                            gba_synth::play_sfx(sfx::ITEM_DROPPED_IN_CAULDRON_FAIL);
                            return Err(());
                        }
                        // Player successfully places item in cauldron
                        gba_synth::play_sfx(sfx::ITEM_DROPPED_IN_CAULDRON_SUCCESS);
                        self.index += 1;
                    }
                }
            }
            if key_input.b() && !self.key_was_pressed.b() {
                // Player drops item on the floor
                gba_synth::play_sfx(sfx::ITEM_DROPPED);
                let item = &mut items[equipped_item_index];
                let pos = &mut item.sprite.pos;
                pos.x = camera.pos.x + camera.yaw_sin() * 32;
                pos.z = camera.pos.z - camera.yaw_cos() * 32;
                item.state = ItemState::Available;
            }
        } else {
            if key_input.a() && !self.key_was_pressed.a() {
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
                        // Player successfully collected an item
                        gba_synth::play_sfx(sfx::ITEM_COLLECTED);
                        item.state = ItemState::EquippedByPlayer;
                        break;
                    } else {
                        // No item close enough to be taken by the user
                        gba_synth::play_sfx(sfx::CANT_TAKE);
                    }
                }
            }
        }
        self.key_was_pressed = *key_input;
        Ok(false)
    }
}

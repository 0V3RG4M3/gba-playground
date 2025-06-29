use gba;
use gba::fixed::i16fx8;
use gba::keys::KeyInput;
use gba::mmio;
use gba::video::obj::ObjDisplayStyle;

use crate::egj2025::item::{Item, ItemKind};
use crate::egj2025::level::Level;
use crate::egj2025::player;
use crate::fixed::Fixed;
use crate::gba_synth;
use crate::mode7::{self, Camera, Sprite};
use crate::sfx;

pub struct KeyLevel {
    items: [Item; 2],
    item_index: Option<usize>,
    key_was_pressed: KeyInput,
}

impl Level for KeyLevel {
    fn new() -> KeyLevel {
        let items =
            [Item::new(0, ItemKind::Door, 128, 2, 24), Item::new(1, ItemKind::Key, 66, 2, 31)];
        let item_index = None;
        let key_was_pressed = KeyInput::new();
        KeyLevel { items, item_index, key_was_pressed }
    }

    fn process(
        &mut self,
        camera: &mut Camera,
        sprites: &mut [Sprite; 32],
        key_input: &KeyInput,
    ) -> bool {
        let mut result = false;

        player::process(camera, key_input);

        let mut other_item_index = None;
        for (i, item) in self.items.iter().enumerate() {
            if Some(i) == self.item_index {
                continue;
            }
            let mut pos = item.sprite.pos - camera.pos;
            pos.y = Fixed::from_int(0);
            let sq_dist = pos.dot(pos);
            if sq_dist.into_int() < 32 * 32 {
                other_item_index = Some(i);
                break;
            }
        }

        match self.item_index {
            Some(item_index) => {
                if key_input.a() && !self.key_was_pressed.a() {
                    gba_synth::play_sfx(sfx::ITEM_DROPPED);
                    let item = &mut self.items[item_index];
                    let pos = &mut item.sprite.pos;
                    pos.x = camera.pos.x + camera.yaw_sin() * 32;
                    pos.z = camera.pos.z - camera.yaw_cos() * 32;
                    self.item_index = None;
                    if let Some(other_item_index) = other_item_index {
                        let item = &self.items[item_index];
                        let other_item = &self.items[other_item_index];
                        result = other_item.kind == ItemKind::Key && item.kind == ItemKind::Door;
                    }
                }
            }
            None => {
                if key_input.a() && !self.key_was_pressed.a() {
                    if other_item_index.is_some() {
                        gba_synth::play_sfx(sfx::ITEM_COLLECTED);
                    } else {
                        gba_synth::play_sfx(sfx::CANT_TAKE);
                    }
                    self.item_index = other_item_index;
                }
            }
        }
        self.key_was_pressed = *key_input;

        for (i, item) in self.items.iter_mut().enumerate() {
            let sprite = &mut item.sprite;
            if self.item_index == Some(i) {
                sprite.obj.0 = sprite.obj.0.with_style(ObjDisplayStyle::Normal);
                sprite.obj.1 = sprite.obj.1.with_x(0);
                sprite.obj.0 = sprite.obj.0.with_y(0);
            } else {
                mode7::prepare_sprite(&camera, sprite);
                let affine_index = sprite.obj.1.affine_index() as usize;
                let scale = i16fx8::from_bits(sprite.scale.into_raw() as i16);
                mmio::AFFINE_PARAM_A.index(affine_index).write(scale);
                mmio::AFFINE_PARAM_D.index(affine_index).write(scale);
            }
            sprites[i] = *sprite;
        }

        return result;
    }
}

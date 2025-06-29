use gba;
use gba::fixed::i16fx8;
use gba::keys::KeyInput;
use gba::mmio;
use gba::video::obj::ObjDisplayStyle;

use crate::egj2025::item::{Item, ItemKind};
use crate::egj2025::level::Level;
use crate::egj2025::player::Player;
use crate::mode7::{self, Camera, Sprite};

pub struct KeyLevel {
    player: Player,
    items: [Item; 2],
    item_index: Option<usize>,
}

impl Level for KeyLevel {
    fn new() -> KeyLevel {
        let player = Player::new();
        let items = [
            Item::new(0, ItemKind::Melon, 128, 2, 24),
            Item::new(1, ItemKind::Watermelon, 66, 2, 31),
        ];
        let item_index = None;
        KeyLevel { player, items, item_index }
    }

    fn process(
        &mut self,
        camera: &mut Camera,
        sprites: &mut [Sprite; 32],
        key_input: &KeyInput,
    ) -> bool {
        self.player.process(&mut self.items, camera, key_input);

        let pairs = [(ItemKind::Melon, ItemKind::Watermelon)];
        let is_done = pairs.iter().all(|(p0, p1)| {
            for (i0, e0) in self.items.iter().enumerate() {
                if Some(i0) == self.item_index {
                    continue;
                }
                if e0.kind != *p0 {
                    continue;
                }
                for (i1, e1) in self.items.iter().enumerate() {
                    if i1 == i0 {
                        continue;
                    }
                    if Some(i1) == self.item_index {
                        continue;
                    }
                    if e1.kind != *p1 {
                        continue;
                    }
                    let pos = e1.sprite.pos - e0.sprite.pos;
                    let sq_dist = pos.dot(pos);
                    if sq_dist.into_int() < 32 * 32 {
                        return true;
                    }
                }
            }
            return false;
        });
        if is_done {
            return true;
        }

        for (i, item) in self.items.iter_mut().enumerate() {
            let sprite = &mut item.sprite;
            if self.player.item_index() == Some(i) {
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

        return false;
    }
}

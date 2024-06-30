use gba::video::obj::{ObjAttr, ObjAttr0, ObjAttr1, ObjAttr2, ObjShape};

use crate::fixed::Fixed;
use crate::game::cauldron::Cauldron;
use crate::game::item::{Item, ItemKind, ItemState};
use crate::math;
use crate::mode7::Sprite;
use crate::sprites;
use crate::vec3::Vec3;

pub struct Leader {
    pub sprite: Sprite,
    index: usize,
}

impl Leader {
    pub fn new(x: i32, z: i32) -> Leader {
        let pos = Vec3 { x: Fixed::from_int(x), y: Fixed::from_int(2), z: Fixed::from_int(z) };
        Leader {
            sprite: Sprite {
                obj: ObjAttr {
                    0: ObjAttr0::new().with_bpp8(true).with_shape(ObjShape::Vertical),
                    1: ObjAttr1::new().with_affine_index(31).with_size(2),
                    2: ObjAttr2::new().with_tile_id(sprites::INDEX_00_SNAIL_BACK as u16),
                },
                pos,
                scale: Fixed::from_int(1),
                z: Fixed::from_int(0),
                rescale_shift: 2,
            },
            index: 0,
        }
    }

    pub fn process<const A: usize, const R: usize>(
        &mut self,
        items: &mut [Item; A],
        recipe_items: &[ItemKind; R],
        cauldron: &mut Cauldron,
    ) -> Result<(), ()> {
        if self.index >= R {
            cauldron.sprite.obj.2 =
                cauldron.sprite.obj.2.with_tile_id(sprites::INDEX_10_CALDRON_FIRE as u16);
            return Ok(());
        }

        let target = self.find_target(items, recipe_items[self.index], cauldron).ok_or(())?;
        let x = target.pos.x.into_int();
        let z = target.pos.z.into_int();
        let angle = math::fast_atan2(x, z);
        self.sprite.pos.x += Fixed::from(math::fast_cos(angle)) / 8;
        self.sprite.pos.z += Fixed::from(math::fast_sin(angle)) / 8;

        if target.sq_dist < 8 * 8 {
            let item = &mut items[target.item_index];
            if item.state == ItemState::Available {
                item.state = ItemState::EquippedByLeader;
            } else {
                item.state = ItemState::ConsumedByLeader;
                self.index += 1;
            }
        }

        Ok(())
    }

    fn find_target<const A: usize>(
        &self,
        items: &[Item; A],
        recipe_item: ItemKind,
        cauldron: &Cauldron,
    ) -> Option<Target> {
        let equipped_item_index = items
            .iter()
            .enumerate()
            .find(|(_, item)| item.state == ItemState::EquippedByLeader)
            .map(|(i, _)| i);
        if let Some(item_index) = equipped_item_index {
            let mut pos = cauldron.sprite.pos - self.sprite.pos;
            pos.y = Fixed::from_int(0);
            let sq_dist = pos.dot(pos).into_int();
            return Some(Target { item_index, sq_dist, pos });
        }

        let mut target: Option<Target> = None;
        for (item_index, item) in items.iter().enumerate() {
            if item.kind != recipe_item {
                continue;
            }
            if item.state != ItemState::Available {
                continue;
            }

            let mut pos = item.sprite.pos - self.sprite.pos;
            pos.y = Fixed::from_int(0);
            let sq_dist = pos.dot(pos).into_int();
            if let Some(target) = target {
                if sq_dist > target.sq_dist {
                    continue;
                }
            }
            target = Some(Target { item_index, sq_dist, pos });
        }
        target
    }
}

#[derive(Clone, Copy, Debug)]
struct Target {
    item_index: usize,
    sq_dist: i32,
    pos: Vec3<i32, 8>,
}

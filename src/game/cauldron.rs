use gba::video::obj::{ObjAttr, ObjAttr0, ObjAttr1, ObjAttr2};

use crate::fixed::Fixed;
use crate::mode7::Sprite;
use crate::sprites;
use crate::vec3::Vec3;

pub struct Cauldron {
    pub sprite: Sprite,
}

impl Cauldron {
    pub fn new(affine_index: u16, x: i32, z: i32) -> Cauldron {
        let pos = Vec3 { x: Fixed::from_int(x), y: Fixed::from_int(2), z: Fixed::from_int(z) };
        Cauldron {
            sprite: Sprite {
                obj: ObjAttr {
                    0: ObjAttr0::new().with_bpp8(true),
                    1: ObjAttr1::new().with_affine_index(affine_index).with_size(2),
                    2: ObjAttr2::new().with_tile_id(sprites::INDEX_07_CALDRON_COLD as u16),
                },
                pos,
                scale: Fixed::from_int(1),
            },
        }
    }
}

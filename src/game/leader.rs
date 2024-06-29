use gba::video::obj::{ObjAttr, ObjAttr0, ObjAttr1, ObjAttr2, ObjShape};

use crate::fixed::Fixed;
use crate::mode7::Sprite;
use crate::vec3::Vec3;

pub struct Leader {
    pub sprite: Sprite,
}

impl Leader {
    pub fn new(x: i32, z: i32) -> Leader {
        let pos = Vec3 { x: Fixed::from_int(x), y: Fixed::from_int(2), z: Fixed::from_int(z) };
        Leader {
            sprite: Sprite {
                obj: ObjAttr {
                    0: ObjAttr0::new().with_bpp8(true).with_shape(ObjShape::Vertical),
                    1: ObjAttr1::new().with_affine_index(31),
                    2: ObjAttr2::new(),
                },
                pos,
                scale: Fixed::from_int(1),
            },
        }
    }
}

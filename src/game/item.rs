use gba::video::obj::{ObjAttr, ObjAttr0, ObjAttr1, ObjAttr2, ObjShape};

use crate::fixed::Fixed;
use crate::mode7::Sprite;
use crate::vec3::Vec3;

pub struct Item {
    pub sprite: Sprite,
    pub state: ItemState,
}

impl Item {
    pub fn new(shape: ObjShape, affine_index: u16, pos: Vec3<i32, 8>) -> Item {
        Item {
            sprite: Sprite {
                obj: ObjAttr {
                    0: ObjAttr0::new().with_bpp8(true).with_shape(shape),
                    1: ObjAttr1::new().with_affine_index(affine_index),
                    2: ObjAttr2::new(),
                },
                pos,
                scale: Fixed::from_int(1),
            },
            state: ItemState::Available,
        }
    }
}

#[derive(PartialEq)]
pub enum ItemState {
    Available,
    Equipped,
}

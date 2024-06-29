use gba::video::obj::{ObjAttr, ObjAttr0, ObjAttr1, ObjAttr2, ObjShape};

use crate::fixed::Fixed;
use crate::mode7::Sprite;
use crate::vec3::Vec3;

pub struct Item {
    pub sprite: Sprite,
    pub state: ItemState,
}

impl Item {
    pub fn new(affine_index: u16, kind: ItemKind, x: i32, y: i32, z: i32) -> Item {
        let pos = Vec3 { x: Fixed::from_int(x), y: Fixed::from_int(y), z: Fixed::from_int(z) };
        Item {
            sprite: Sprite {
                obj: ObjAttr {
                    0: ObjAttr0::new().with_bpp8(true).with_shape(kind.shape()),
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

#[derive(PartialEq)]
pub enum ItemKind {
    Banana,
    BoarThigh,
    Melon,
    Watermelon,
}

impl ItemKind {
    pub fn shape(&self) -> ObjShape {
        match self {
            ItemKind::Banana => ObjShape::Horizontal,
            ItemKind::BoarThigh => ObjShape::Square,
            ItemKind::Melon => ObjShape::Square,
            ItemKind::Watermelon => ObjShape::Square,
        }
    }
}

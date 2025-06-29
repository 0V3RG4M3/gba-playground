use gba::video::obj::{ObjAttr, ObjAttr0, ObjAttr1, ObjAttr2, ObjShape};

use crate::egj2025::sprites;
use crate::fixed::Fixed;
use crate::mode7::Sprite;
use crate::vec3::Vec3;

pub struct Item {
    pub kind: ItemKind,
    pub sprite: Sprite,
}

impl Item {
    pub fn new(affine_index: u16, kind: ItemKind, x: i32, y: i32, z: i32) -> Item {
        let pos = Vec3 { x: Fixed::from_int(x), y: Fixed::from_int(y), z: Fixed::from_int(z) };
        Item {
            kind,
            sprite: Sprite {
                obj: ObjAttr {
                    0: ObjAttr0::new().with_bpp8(true).with_shape(kind.shape()),
                    1: ObjAttr1::new().with_affine_index(affine_index).with_size(kind.size()),
                    2: ObjAttr2::new().with_tile_id(kind.tile_id()),
                },
                pos,
                scale: Fixed::from_int(1),
                z: Fixed::from_int(0),
                rescale_factor: Fixed::from_int(1) / 5,
            },
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ItemKind {
    Door,
    Key,
}

impl ItemKind {
    pub fn shape(&self) -> ObjShape {
        match self {
            ItemKind::Door => ObjShape::Square,
            ItemKind::Key => ObjShape::Square,
        }
    }

    pub fn size(&self) -> u16 {
        match self {
            ItemKind::Door => 2,
            ItemKind::Key => 2,
        }
    }

    pub fn tile_id(&self) -> u16 {
        match self {
            ItemKind::Door => sprites::INDEX_24_DOOR_32 as u16,
            ItemKind::Key => sprites::INDEX_28_KEY1_32 as u16,
        }
    }
}

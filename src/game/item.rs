use gba::video::obj::{ObjAttr, ObjAttr0, ObjAttr1, ObjAttr2, ObjShape};

use crate::fixed::Fixed;
use crate::mode7::Sprite;
use crate::sprites;
use crate::vec3::Vec3;

pub struct Item {
    pub kind: ItemKind,
    pub sprite: Sprite,
    pub state: ItemState,
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
            },
            state: ItemState::Available,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ItemState {
    Available,
    EquippedByLeader,
    ConsumedByLeader,
    EquippedByPlayer,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ItemKind {
    Banana,
    BoarThigh,
    Melon,
    Watermelon,
    IceCream0,
    IceCream1,
}

impl ItemKind {
    pub fn shape(&self) -> ObjShape {
        match self {
            ItemKind::Banana => ObjShape::Horizontal,
            ItemKind::BoarThigh => ObjShape::Square,
            ItemKind::Melon => ObjShape::Square,
            ItemKind::Watermelon => ObjShape::Horizontal,
            ItemKind::IceCream0 => ObjShape::Vertical,
            ItemKind::IceCream1 => ObjShape::Vertical,
        }
    }

    pub fn size(&self) -> u16 {
        match self {
            ItemKind::Banana => 2,
            ItemKind::BoarThigh => 2,
            ItemKind::Melon => 2,
            ItemKind::Watermelon => 2,
            ItemKind::IceCream0 => 2,
            ItemKind::IceCream1 => 2,
        }
    }

    pub fn tile_id(&self) -> u16 {
        match self {
            ItemKind::Banana => sprites::INDEX_01_BANANA as u16,
            ItemKind::BoarThigh => sprites::INDEX_03_BONE_MEAT as u16,
            ItemKind::Melon => sprites::INDEX_04_MELONE as u16,
            ItemKind::Watermelon => sprites::INDEX_05_WATERMELONE as u16,
            ItemKind::IceCream0 => sprites::INDEX_11_ICESCREAM_CONE as u16,
            ItemKind::IceCream1 => sprites::INDEX_12_ICESCREAM_CONE as u16,
        }
    }
}

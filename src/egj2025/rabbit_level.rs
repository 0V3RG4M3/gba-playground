use gba::fixed::i16fx8;
use gba::keys::KeyInput;
use gba::mmio;
use gba::video::obj::{ObjAttr, ObjAttr0, ObjAttr1, ObjAttr2};

use crate::egj2025::level::Level;
use crate::egj2025::player;
use crate::egj2025::sprites;
use crate::fixed::Fixed;
use crate::mode7::{self, Camera, Sprite};
use crate::vec3::Vec3;

pub struct RabbitLevel {
    rabbit: Sprite,
}

impl Level for RabbitLevel {
    fn new() -> RabbitLevel {
        let pos = Vec3 { x: Fixed::from_int(144), y: Fixed::from_int(2), z: Fixed::from_int(80) };
        let rabbit = Sprite {
            obj: ObjAttr {
                0: ObjAttr0::new().with_bpp8(true),
                1: ObjAttr1::new().with_affine_index(0).with_size(2),
                2: ObjAttr2::new().with_tile_id(sprites::INDEX_20_BUNNY2_32 as u16),
            },
            pos,
            scale: Fixed::from_int(1),
            z: Fixed::from_int(0),
            rescale_factor: Fixed::from_int(1) / 5,
        };
        RabbitLevel { rabbit }
    }

    fn process(
        &mut self,
        camera: &mut Camera,
        sprites: &mut [Sprite; 32],
        key_input: &KeyInput,
    ) -> bool {
        let pos = camera.pos;

        player::process(camera, key_input);

        let mut speed = camera.pos - pos;
        speed.x = speed.x * 2;
        speed.y = Fixed::from_int(0);
        speed.z = speed.z * 2;
        self.rabbit.pos = self.rabbit.pos + speed;

        mode7::prepare_sprite(&camera, &mut self.rabbit);
        let scale = i16fx8::from_bits(self.rabbit.scale.into_raw() as i16);
        mmio::AFFINE_PARAM_A.index(0).write(scale);
        mmio::AFFINE_PARAM_D.index(0).write(scale);
        sprites[0] = self.rabbit;

        let mut pos = self.rabbit.pos - camera.pos;
        pos.y = Fixed::from_int(0);
        let sq_dist = pos.dot(pos);
        sq_dist.into_int() < 32 * 32
    }
}

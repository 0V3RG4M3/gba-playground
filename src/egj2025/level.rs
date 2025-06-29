use gba::keys::KeyInput;

use crate::mode7::{Camera, Sprite};

pub trait Level {
    fn new() -> Self;
    fn process(
        &mut self,
        camera: &mut Camera,
        sprites: &mut [Sprite; 32],
        key_input: &KeyInput,
    ) -> bool;
}

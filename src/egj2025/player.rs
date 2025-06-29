use gba::keys::KeyInput;

use crate::mode7::Camera;

pub fn process(camera: &mut Camera, key_input: &KeyInput) {
    let mut cam_yaw_angle = camera.yaw_angle();
    cam_yaw_angle -= key_input.left() as u8;
    cam_yaw_angle += key_input.right() as u8;
    camera.set_yaw_angle(cam_yaw_angle);

    camera.pos.x += camera.yaw_sin() * (key_input.up() as i32);
    camera.pos.x -= camera.yaw_sin() * (key_input.down() as i32);
    camera.pos.x -= camera.yaw_cos() * (key_input.l() as i32);
    camera.pos.x += camera.yaw_cos() * (key_input.r() as i32);

    camera.pos.z -= camera.yaw_cos() * (key_input.up() as i32);
    camera.pos.z += camera.yaw_cos() * (key_input.down() as i32);
    camera.pos.z -= camera.yaw_sin() * (key_input.l() as i32);
    camera.pos.z += camera.yaw_sin() * (key_input.r() as i32);
}

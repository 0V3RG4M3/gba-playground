use gba::fixed::{i16fx8, i32fx8};
use gba::gba_cell::GbaCell;
use gba::mmio;
use gba::video::obj::{ObjAttr, ObjDisplayStyle, ObjShape};
use gba::video::BackgroundControl;

use crate::fixed::Fixed;
use crate::math;
use crate::vec3::Vec3;

const FOCAL_LENGTH: i32 = 256;
const NEAR: i32 = 24;
const FAR: i32 = 512;

static SIZE: GbaCell<u16> = GbaCell::new(0);
static CAM_X: GbaCell<Fixed<i32, 8>> = GbaCell::new(Fixed::from_raw(0));
static CAM_Y: GbaCell<Fixed<i32, 20>> = GbaCell::new(Fixed::from_raw(0));
static CAM_Z: GbaCell<Fixed<i32, 8>> = GbaCell::new(Fixed::from_raw(0));
static CAM_YAW_SIN: GbaCell<Fixed<i32, 8>> = GbaCell::new(Fixed::from_raw(0));
static CAM_YAW_COS: GbaCell<Fixed<i32, 8>> = GbaCell::new(Fixed::from_raw(0));
static CAM_PITCH_SIN: GbaCell<Fixed<i32, 8>> = GbaCell::new(Fixed::from_raw(0));
static CAM_PITCH_COS: GbaCell<Fixed<i32, 8>> = GbaCell::new(Fixed::from_raw(0));
static HORIZON: GbaCell<i32> = GbaCell::new(160);

#[derive(Clone, Copy, Debug)]
pub struct Camera {
    pub pos: Vec3<i32, 8>,
    yaw_angle: u8,
    yaw_sin: Fixed<i32, 8>,
    yaw_cos: Fixed<i32, 8>,
    pitch_angle: u8,
    pitch_sin: Fixed<i32, 8>,
    pitch_cos: Fixed<i32, 8>,
}

impl Camera {
    pub fn new() -> Camera {
        let pos = Vec3 { x: Fixed::from_int(0), y: Fixed::from_int(0), z: Fixed::from_int(0) };
        Camera {
            pos,
            yaw_angle: 0,
            yaw_sin: Fixed::from(math::fast_sin(0)),
            yaw_cos: Fixed::from(math::fast_cos(0)),
            pitch_angle: 0,
            pitch_sin: Fixed::from(math::fast_sin(0)),
            pitch_cos: Fixed::from(math::fast_cos(0)),
        }
    }

    pub fn yaw_angle(&self) -> u8 {
        self.yaw_angle
    }

    pub fn yaw_sin(&self) -> Fixed<i32, 8> {
        self.yaw_sin
    }

    pub fn yaw_cos(&self) -> Fixed<i32, 8> {
        self.yaw_cos
    }

    pub fn pitch_angle(&self) -> u8 {
        self.pitch_angle
    }

    pub fn pitch_sin(&self) -> Fixed<i32, 8> {
        self.pitch_sin
    }

    pub fn pitch_cos(&self) -> Fixed<i32, 8> {
        self.pitch_cos
    }

    pub fn set_yaw_angle(&mut self, yaw_angle: u8) {
        self.yaw_angle = yaw_angle;
        self.yaw_sin = Fixed::from(math::fast_sin(yaw_angle));
        self.yaw_cos = Fixed::from(math::fast_cos(yaw_angle));
    }

    pub fn set_pitch_angle(&mut self, pitch_angle: u8) {
        self.pitch_angle = pitch_angle;
        self.pitch_sin = Fixed::from(math::fast_sin(pitch_angle));
        self.pitch_cos = Fixed::from(math::fast_cos(pitch_angle));
    }

    pub fn u(&self) -> Vec3<i32, 8> {
        Vec3 { x: self.yaw_cos, y: Fixed::from_int(0), z: self.yaw_sin }
    }

    pub fn v(&self) -> Vec3<i32, 8> {
        Vec3 {
            x: self.yaw_sin.mul(self.pitch_sin),
            y: self.pitch_cos,
            z: -self.yaw_cos.mul(self.pitch_sin),
        }
    }

    pub fn w(&self) -> Vec3<i32, 8> {
        Vec3 {
            x: -self.yaw_sin.mul(self.pitch_cos),
            y: self.pitch_sin,
            z: self.yaw_cos.mul(self.pitch_cos),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Sprite {
    pub obj: ObjAttr,
    pub pos: Vec3<i32, 8>,
    pub scale: Fixed<i32, 8>,
    pub z: Fixed<i32, 8>,
    pub rescale_factor: Fixed<i32, 8>,
}

impl Sprite {
    pub fn new() -> Sprite {
        Sprite {
            obj: ObjAttr::new(),
            pos: Vec3 { x: Fixed::from_int(0), y: Fixed::from_int(0), z: Fixed::from_int(0) },
            scale: Fixed::from_int(1),
            z: Fixed::from_int(0),
            rescale_factor: Fixed::from_int(1),
        }
    }
}

pub fn prepare_sprite(camera: &Camera, sprite: &mut Sprite) {
    let size = sprite.obj.1.size();
    let mut x_shift = size;
    let mut y_shift = size;
    match sprite.obj.0.shape() {
        ObjShape::Square => (),
        ObjShape::Horizontal => {
            if size < 2 {
                x_shift += 1;
            }
            if size != 0 {
                y_shift -= 1;
            }
        }
        ObjShape::Vertical => {
            if size != 0 {
                x_shift -= 1;
            }
            if size < 2 {
                y_shift += 1;
            }
        }
    };
    let size_x = 8 << x_shift;
    let size_y = 8 << y_shift;

    let rescale_factor = sprite.rescale_factor;
    sprite.pos.y = rescale_factor * size_y;
    let pos = Vec3::<i32, 8> {
        x: sprite.pos.x - camera.pos.x,
        y: sprite.pos.y - camera.pos.y,
        z: sprite.pos.z - camera.pos.z,
    };

    let pos =
        Vec3::<i32, 8> { x: pos.dot(camera.u()), y: -pos.dot(camera.v()), z: -pos.dot(camera.w()) };

    sprite.obj.0 = sprite.obj.0.with_style(ObjDisplayStyle::NotDisplayed);

    if pos.z.into_int() < NEAR || pos.z.into_int() >= FAR {
        return;
    }

    let scale: Fixed<i32, 8> = Fixed::<i32, 16>::from_int(FOCAL_LENGTH).div(pos.z);

    let left: Fixed<i32, 8> = pos.x.mul(scale);
    let right: Fixed<i32, 8> = (pos.x + rescale_factor * size_x).mul(scale);
    if right.into_int() < -120 || left.into_int() >= 120 {
        return;
    }

    let top: Fixed<i32, 8> = pos.y.mul(scale);
    let bottom: Fixed<i32, 8> = (pos.y + rescale_factor * size_y).mul(scale);
    if bottom.into_int() < -80 || top.into_int() >= 80 {
        return;
    }

    let y: Fixed<i32, 8> = (pos.y + rescale_factor * (size_y / 2)).mul(scale);
    let x: Fixed<i32, 8> = pos.x.mul(scale);
    let x = x.into_int() - size_x;
    let y = y.into_int() - size_y;

    sprite.obj.1 = sprite.obj.1.with_x((x + 120) as u16);
    sprite.obj.0 = sprite.obj.0.with_y((y + 80) as u16);

    sprite.z = pos.z;
    let scale = pos.z.into_raw() / rescale_factor.into_raw();
    sprite.scale = Fixed::from_raw(scale);

    sprite.obj.0 = sprite.obj.0.with_style(ObjDisplayStyle::DoubleSizeAffine);
}

pub fn prepare_frame(size: u16, camera: &Camera) {
    SIZE.write(size);
    CAM_X.write(camera.pos.x);
    CAM_Y.write(Fixed::from(camera.pos.y));
    CAM_Z.write(camera.pos.z);
    CAM_YAW_SIN.write(camera.yaw_sin);
    CAM_YAW_COS.write(camera.yaw_cos);
    CAM_PITCH_SIN.write(camera.pitch_sin);
    CAM_PITCH_COS.write(camera.pitch_cos);
    let horizon = if camera.pitch_cos().into_raw() == 0 {
        if camera.pitch_sin().into_raw() > 0 {
            0
        } else {
            160
        }
    } else {
        let n =
            (FAR * camera.pitch_sin().into_raw() - (camera.pos.y.into_raw() >> 12)) * FOCAL_LENGTH;
        let d = FAR * camera.pitch_cos().into_raw().abs();
        (80 - n / d).clamp(0, 160)
    };
    HORIZON.write(horizon);
}

#[link_section = ".iwram"]
pub fn process_line(line: i32) {
    if line < HORIZON.read() || line >= 160 {
        return;
    }

    let yaw_sin = CAM_YAW_SIN.read();
    let yaw_cos = CAM_YAW_COS.read();
    let pitch_sin = CAM_PITCH_SIN.read();
    let pitch_cos = CAM_PITCH_COS.read();
    let by = pitch_cos * (line - 80) + pitch_sin * FOCAL_LENGTH;
    if by.into_int() <= 1 {
        return;
    }
    let size = SIZE.read();
    mmio::BG2CNT.write(BackgroundControl::new().with_screenblock(1).with_size(size));
    let bz = pitch_sin * (line - 80) - pitch_cos * FOCAL_LENGTH;
    let lambda: Fixed<i32, 12> = CAM_Y.read().div(by.max(Fixed::from_raw(1)));

    let pa: Fixed<i32, 12> = lambda.mul(yaw_cos);
    let pc: Fixed<i32, 12> = lambda.mul(yaw_sin);
    mmio::BG2PA.write(i16fx8::from_bits(Fixed::<i32, 8>::from(pa).into_raw() as i16));
    mmio::BG2PC.write(i16fx8::from_bits(Fixed::<i32, 8>::from(pc).into_raw() as i16));

    let x = CAM_X.read() - (Fixed::from(pa) * 120) - bz.mul(pc);
    let y = CAM_Z.read() - (Fixed::from(pc) * 120) + bz.mul(pa);
    mmio::BG2X.write(i32fx8::from_bits(x.into_raw()));
    mmio::BG2Y.write(i32fx8::from_bits(y.into_raw()));
}

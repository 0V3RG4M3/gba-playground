use gba::fixed::{i16fx8, i32fx8};
use gba::gba_cell::GbaCell;
use gba::mmio;
use gba::video::BackgroundControl;

use crate::fixed::Fixed;
use crate::math;

static CAM_X: GbaCell<Fixed<i32, 8>> = GbaCell::new(Fixed::from_raw(0));
static CAM_Y: GbaCell<Fixed<i32, 20>> = GbaCell::new(Fixed::from_raw(0));
static CAM_Z: GbaCell<Fixed<i32, 8>> = GbaCell::new(Fixed::from_raw(0));
static CAM_YAW_SIN: GbaCell<Fixed<i32, 8>> = GbaCell::new(Fixed::from_raw(0));
static CAM_YAW_COS: GbaCell<Fixed<i32, 8>> = GbaCell::new(Fixed::from_raw(0));
static CAM_PITCH_SIN: GbaCell<Fixed<i32, 8>> = GbaCell::new(Fixed::from_raw(0));
static CAM_PITCH_COS: GbaCell<Fixed<i32, 8>> = GbaCell::new(Fixed::from_raw(0));
static HORIZON: GbaCell<i32> = GbaCell::new(160);

pub struct Camera {
    pub x: Fixed<i32, 8>,
    pub y: Fixed<i32, 20>,
    pub z: Fixed<i32, 8>,
    yaw_angle: u8,
    yaw_sin: Fixed<i32, 8>,
    yaw_cos: Fixed<i32, 8>,
    pitch_angle: u8,
    pitch_sin: Fixed<i32, 8>,
    pitch_cos: Fixed<i32, 8>,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            x: Fixed::from_int(64),
            y: Fixed::from_int(16),
            z: Fixed::from_int(64),
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
}

pub fn prepare_frame(camera: &Camera) {
    CAM_X.write(camera.x);
    CAM_Y.write(camera.y);
    CAM_Z.write(camera.z);
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
        let n = (768 * camera.pitch_sin().into_raw() - (camera.y.into_raw() >> 12)) * 256;
        let d = 768 * camera.pitch_cos().into_raw();
        (80 - n / d).clamp(0, 160)
    };
    HORIZON.write(horizon);
}

#[link_section = ".iwram"]
pub fn process_line(line: i32) {
    if line < HORIZON.read() || line >= 160 {
        return;
    } else {
        mmio::BG2CNT.write(BackgroundControl::new().with_screenblock(1));
    }

    let yaw_sin = CAM_YAW_SIN.read();
    let yaw_cos = CAM_YAW_COS.read();
    let pitch_sin = CAM_PITCH_SIN.read();
    let pitch_cos = CAM_PITCH_COS.read();
    let by = pitch_cos * (line - 80) + pitch_sin * 256;
    let bz = pitch_sin * (line - 80) - pitch_cos * 256;
    let lambda: Fixed<i32, 12> = CAM_Y.read().div(by.max(Fixed::from_raw(1)));

    let pa: Fixed<i32, 12> = lambda.mul(yaw_cos);
    let pc: Fixed<i32, 12> = lambda.mul(yaw_sin);
    mmio::BG2PA.write(i16fx8::from_raw(Fixed::<i32, 8>::from(pa).into_raw() as i16));
    mmio::BG2PC.write(i16fx8::from_raw(Fixed::<i32, 8>::from(pc).into_raw() as i16));

    let x = CAM_X.read() - (Fixed::from(pa) * 120) - bz.mul(pc);
    let y = CAM_Z.read() - (Fixed::from(pc) * 120) + bz.mul(pa);
    mmio::BG2X.write(i32fx8::from_raw(x.into_raw()));
    mmio::BG2Y.write(i32fx8::from_raw(y.into_raw()));
}

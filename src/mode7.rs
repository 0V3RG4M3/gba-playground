use gba::fixed::{i16fx8, i32fx8};
use gba::gba_cell::GbaCell;
use gba::mmio;
use gba::video::BackgroundControl;

use crate::fixed::Fixed;

static CAM_X: GbaCell<Fixed<i32, 8>> = GbaCell::new(Fixed::from_raw(0));
static CAM_Y: GbaCell<Fixed<i32, 20>> = GbaCell::new(Fixed::from_raw(0));
static CAM_Z: GbaCell<Fixed<i32, 8>> = GbaCell::new(Fixed::from_raw(0));
static CAM_YAW_SIN: GbaCell<Fixed<i32, 8>> = GbaCell::new(Fixed::from_raw(0));
static CAM_YAW_COS: GbaCell<Fixed<i32, 8>> = GbaCell::new(Fixed::from_raw(0));
static CAM_PITCH_SIN: GbaCell<Fixed<i32, 8>> = GbaCell::new(Fixed::from_raw(0));
static CAM_PITCH_COS: GbaCell<Fixed<i32, 8>> = GbaCell::new(Fixed::from_raw(0));
static HORIZON: GbaCell<i32> = GbaCell::new(160);

pub fn prepare_frame() {}

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

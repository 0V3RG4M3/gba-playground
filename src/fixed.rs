use core::ops::{Add, AddAssign, Div, Mul, Shl, Shr, Sub, SubAssign};

use gba::gba_cell::GbaCellSafe;

pub trait Int: Shl<i8, Output = Self> + Shr<i8, Output = Self> + Sized {}

impl<I: Shl<i8, Output = Self> + Shr<i8, Output = Self> + Sized> Int for I {}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Fixed<I: Int, const B: u8> {
    raw: I,
}

impl<I: Int, const B: u8> Fixed<I, B> {
    pub const fn from_raw(raw: I) -> Fixed<I, B> {
        Fixed { raw }
    }

    pub fn into_raw(self) -> I {
        self.raw
    }

    pub fn from_int(int: I) -> Fixed<I, B> {
        Fixed { raw: int << (B as i8) }
    }

    pub fn into_int(self) -> I {
        self.raw >> (B as i8)
    }

    pub fn from<const BS: u8>(source: Fixed<I, BS>) -> Fixed<I, B> {
        Fixed { raw: source.raw >> (BS as i8 - B as i8) }
    }
}

impl<I: Int + Mul<Output = I>, const B: u8> Fixed<I, B> {
    pub fn mul<const BR: u8, const BO: u8>(self, other: Fixed<I, BR>) -> Fixed<I, BO> {
        Fixed { raw: (self.raw * other.raw) >> (B as i8 + BR as i8 - BO as i8) }
    }
}

impl<I: Int + Div<Output = I>, const B: u8> Fixed<I, B> {
    pub fn div<const BR: u8, const BO: u8>(self, other: Fixed<I, BR>) -> Fixed<I, BO> {
        Fixed { raw: (self.raw / other.raw) >> (B as i8 - BR as i8 - BO as i8) }
    }
}

impl<I: Int + Add<Output = I>, const B: u8> Add for Fixed<I, B> {
    type Output = Fixed<I, B>;

    fn add(self, other: Fixed<I, B>) -> Fixed<I, B> {
        Fixed { raw: self.raw + other.raw }
    }
}

impl<I: Int + Sub<Output = I>, const B: u8> Sub for Fixed<I, B> {
    type Output = Fixed<I, B>;

    fn sub(self, other: Fixed<I, B>) -> Fixed<I, B> {
        Fixed { raw: self.raw - other.raw }
    }
}

impl<I: Int + Mul<Output = I>, const B: u8> Mul<I> for Fixed<I, B> {
    type Output = Fixed<I, B>;

    fn mul(self, int: I) -> Self::Output {
        Fixed { raw: self.raw * int }
    }
}

impl<I: Int + AddAssign, const B: u8> AddAssign for Fixed<I, B> {
    fn add_assign(&mut self, other: Fixed<I, B>) {
        self.raw += other.raw;
    }
}

impl<I: Int + SubAssign, const B: u8> SubAssign for Fixed<I, B> {
    fn sub_assign(&mut self, other: Fixed<I, B>) {
        self.raw -= other.raw;
    }
}

unsafe impl<I: Int + GbaCellSafe, const B: u8> GbaCellSafe for Fixed<I, B> {}

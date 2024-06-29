use core::ops::{Add, Mul, Sub};

use crate::fixed::{Fixed, Int};

#[derive(Clone, Copy, Debug)]
pub struct Vec3<I: Int, const B: u8> {
    pub x: Fixed<I, B>,
    pub y: Fixed<I, B>,
    pub z: Fixed<I, B>,
}

impl<I: Int + Add<Output = I> + Mul<Output = I>, const B: u8> Vec3<I, B> {
    pub fn dot(self: Vec3<I, B>, other: Vec3<I, B>) -> Fixed<I, B> {
        self.x.mul(other.x) + self.y.mul(other.y) + self.z.mul(other.z)
    }
}

impl<I: Int + Add<Output = I>, const B: u8> Add for Vec3<I, B> {
    type Output = Vec3<I, B>;

    fn add(self, other: Vec3<I, B>) -> Vec3<I, B> {
        Vec3 { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
}

impl<I: Int + Sub<Output = I>, const B: u8> Sub for Vec3<I, B> {
    type Output = Vec3<I, B>;

    fn sub(self, other: Vec3<I, B>) -> Vec3<I, B> {
        Vec3 { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
    }
}

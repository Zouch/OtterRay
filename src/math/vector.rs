use std::ops;
use math::*;

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 { x: x, y: y, z: z }
    }

    pub fn zero() -> Vector3 { Vector3 { x: 0.0, y: 0.0, z: 0.0, } }
    pub fn ones() -> Vector3 { Vector3 { x: 1.0, y: 1.0, z: 1.0, } }

    pub fn unit_x() -> Vector3 { Vector3 { x: 1.0, y: 0.0, z: 0.0, } }
    pub fn unit_y() -> Vector3 { Vector3 { x: 0.0, y: 1.0, z: 0.0, } }
    pub fn unit_z() -> Vector3 { Vector3 { x: 0.0, y: 0.0, z: 1.0, } }
}

impl ops::Neg for Vector3 {
    type Output = Vector3;
    fn neg(self) -> Vector3 {
        Vector3 { x: -self.x, y: -self.y, z: -self.z }
    }
}

impl ops::Add<Vector3> for Vector3 {
    type Output = Vector3;
    fn add(self, b: Vector3) -> Vector3 {
        Vector3 { x: self.x + b.x, y: self.y + b.y, z: self.z + b.z }
    }
}

impl ops::AddAssign<Vector3> for Vector3 {
    fn add_assign(&mut self, b: Vector3) {
        *self = *self + b;
    }
}

impl ops::Sub<Vector3> for Vector3 {
    type Output = Vector3;
    fn sub(self, b: Vector3) -> Vector3 {
        Vector3 { x: self.x - b.x, y: self.y - b.y, z: self.z - b.z }
    }
}

impl ops::SubAssign<Vector3> for Vector3 {
    fn sub_assign(&mut self, b: Vector3) {
        *self = *self - b;
    }
}

impl ops::Mul<f32> for Vector3 {
    type Output = Vector3;
    fn mul(self, x: f32) -> Vector3 {
        Vector3 { x: self.x * x, y: self.y * x, z: self.z * x }
    }
}

impl ops::Mul<Vector3> for f32 {
    type Output = Vector3;
    fn mul(self, v: Vector3) -> Vector3 {
        Vector3 { x: self * v.x, y: self * v.y, z: self * v.z }
    }
}

impl ops::MulAssign<f32> for Vector3 {
    fn mul_assign(&mut self, x: f32) {
        *self = *self * x;
    }
}

impl ops::Mul<Vector3> for Vector3 {
    type Output = Vector3;
    fn mul(self, v: Vector3) -> Vector3 {
        Vector3 { x: self.x * v.x, y: self.y * v.y, z: self.z * v.z }
    }
}

impl ops::MulAssign<Vector3> for Vector3 {
    fn mul_assign(&mut self, x: Vector3) {
        *self = *self * x;
    }
}

impl ops::Div<f32> for Vector3 {
    type Output = Vector3;
    fn div(self, x: f32) -> Vector3 {
        let inv_x = 1.0 / x;
        Vector3 { x: self.x * inv_x, y: self.y * inv_x, z: self.z * inv_x }
    }
}

impl ops::Div<Vector3> for f32 {
    type Output = Vector3;
    fn div(self, v: Vector3) -> Vector3 {
        Vector3 { x: self / v.x, y: self / v.y, z: self / v.z }
    }
}

impl ops::DivAssign<f32> for Vector3 {
    fn div_assign(&mut self, x: f32) {
        *self = *self / x;
    }
}

impl ops::Div<Vector3> for Vector3 {
    type Output = Vector3;
    fn div(self, v: Vector3) -> Vector3 {
        Vector3 { x: self.x / v.x, y: self.y / v.y, z: self.z / v.z }
    }
}

impl ops::DivAssign<Vector3> for Vector3 {
    fn div_assign(&mut self, v: Vector3) {
        *self = *self / v;
    }
}

pub fn cross(a: Vector3, b: Vector3) -> Vector3 {
    Vector3 {
        x: a.y * b.z - a.z * b.y,
        y: a.z * b.x - a.x * b.z,
        z: a.x * b.y - a.y * b.x,
    }
}

pub fn inner_product(a: Vector3, b: Vector3) -> f32 { a.x * b.x + a.y * b.y + a.z * b.z }
pub fn dot(a: Vector3, b: Vector3) -> f32 { inner_product(a, b) }
pub fn length_squared(v: Vector3) -> f32 { dot(v, v) }
pub fn length(v: Vector3) -> f32 { sqrt(length_squared(v)) }
pub fn inv_length(v: Vector3) -> f32 { 1.0 / sqrt(length_squared(v)) }
pub fn normalize(v: Vector3) -> Vector3 { v * inv_length(v) }

pub fn safe_normalize(v: Vector3) -> Vector3 {
    let result: Vector3;
    let lensq = length_squared(v);
    if lensq > square(1e-4) {
        result = v / sqrt(lensq);
    } else {
        result = Vector3::zero();
    }

    return result;
}
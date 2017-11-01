use std::f32;

#[macro_use]
pub mod vector;
pub use self::vector::*;

#[macro_use]
pub mod color;
pub use self::color::*;

#[macro_use]
pub mod ray;
pub use self::ray::*;

#[inline]
#[allow(dead_code)]
pub fn sqrt(x: f32) -> f32 {
    let result = x.sqrt();
    return result;
}

#[inline]
#[allow(dead_code)]
pub fn abs(x: f32) -> f32 {
    let result = if x < 0.0 { -x } else { x };
    return result;
}

#[inline]
#[allow(dead_code)]
pub fn floor(x: f32) -> f32 {
    let result = x.floor();
    return result;
}

#[inline]
#[allow(dead_code)]
pub fn ceil(x: f32) -> f32 {
    let result = x.ceil();
    return result;
}

#[inline]
#[allow(dead_code)]
pub fn sin(x: f32) -> f32 {
    let result = x.sin();
    return result;
}

#[inline]
#[allow(dead_code)]
pub fn cos(x: f32) -> f32 {
    let result = x.cos();
    return result;
}

#[inline]
#[allow(dead_code)]
pub fn tan(x: f32) -> f32 {
    let result = x.tan();
    return result;
}

#[inline]
#[allow(dead_code)]
pub fn asin(x: f32) -> f32 {
    let result = x.asin();
    return result;
}

#[inline]
#[allow(dead_code)]
pub fn acos(x: f32) -> f32 {
    let result = x.acos();
    return result;
}

#[inline]
#[allow(dead_code)]
pub fn atan(x: f32) -> f32 {
    let result = x.atan();
    return result;
}

#[inline]
#[allow(dead_code)]
pub fn square(x: f32) -> f32 {
    let result = x * x;
    return result;
}

pub fn min(a: f32, b: f32) -> f32 {
    let result = if a < b { a } else { b };
    return result;
}

pub fn max(a: f32, b: f32) -> f32 {
    let result = if a > b { a } else { b };
    return result;
}

pub fn clamp(x: f32, a: f32, b: f32) -> f32 {
    max(min(x, b), a)
}

pub fn saturate(x: f32) -> f32 {
    clamp(x, 0.0, 1.0)
}
use math::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
}

impl Ray {
    pub fn new(origin: Vector3, direction: Vector3) -> Ray {
        Ray {
            origin: origin,
            direction: safe_normalize(direction),
        }
    }
}

#[inline]
pub fn ray_point(ray: Ray, t: f32) -> Vector3 {
    let result = ray.origin + t * ray.direction;
    return result;
}
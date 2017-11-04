pub mod camera;
pub use self::camera::*;

pub mod world;
pub use self::world::*;

use math::*;

pub fn make_plane(n: Vector3, d: f32, mat_index: u32) -> Object {
    let plane = Plane::new(n, d);
    let result = Object::new(Box::new(plane), mat_index);
    return result;
}

pub fn make_sphere(p: Vector3, r: f32, mat_index: u32) -> Object {
    let sphere = Sphere::new(p, r);
    let result = Object::new(Box::new(sphere), mat_index);
    return result;
}
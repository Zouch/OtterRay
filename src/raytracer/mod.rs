pub mod camera;
pub use self::camera::*;

pub mod world;
pub use self::world::*;

use math::*;

pub fn make_plane(n: Vector3, d: f32, material: Material) -> Object {
    let plane = Plane::new(n, d);
    let result = Object::new(Box::new(plane), material);
    return result;
}

pub fn make_sphere(p: Vector3, r: f32, material: Material) -> Object {
    let sphere = Sphere::new(p, r);
    let result = Object::new(Box::new(sphere), material);
    return result;
}
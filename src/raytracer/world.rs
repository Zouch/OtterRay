use std::f32;
use std::thread;
use std::sync::mpsc::Sender;

use math::*;
use Camera;
use utils::Image;

extern crate rand;
use self::rand::distributions::{IndependentSample, Range};

pub struct Intersection {
    t: f32,
    position: Vector3,
    normal: Vector3,
    material: Material,
    is_valid: bool
}

impl Intersection {
    fn new() -> Intersection {
        Intersection{
            t: f32::MAX,
            position: Vector3::new(0.0, 0.0, 0.0),
            normal: Vector3::new(0.0, 0.0, 0.0),
            material: Material::new(Color::BLACK),
            is_valid: false,
        }
    }
}

pub trait Intersectable {
    fn intersect(&self, ray: Ray) -> Intersection;
    fn box_clone(&self) -> Box<Intersectable>;
}

impl Clone for Box<Intersectable> {
    fn clone(&self) -> Box<Intersectable> {
        self.box_clone()
    }
}

#[derive(Clone)]
pub struct Plane { normal: Vector3, d: f32 }
impl Plane {
    pub fn new(normal: Vector3, d: f32) -> Plane {
        Plane { normal: normal, d: d }
    }
}

#[derive(Clone)]
pub struct Sphere { position: Vector3, r: f32 }
impl Sphere {
    pub fn new(position: Vector3, r: f32) -> Sphere {
        Sphere { position: position, r: r }
    }
}

#[derive(Clone, Copy)]
pub struct Material {
    albedo: Color,
}

#[derive(Clone)]
pub struct Object {
    geometry: Box<Intersectable>,
    material: Material,
}
impl Object {
    pub fn new(geometry: Box<Intersectable>, material: Material) -> Object {
        Object {
            geometry: geometry,
            material: material,
        }
    }
}

impl Material {
    pub fn new(albedo: Color) -> Material {
        Material {
            albedo: albedo,
        }
    }
}

#[derive(Clone)]
pub struct World {
    objects: Vec<Object>,
    camera: Camera,
}

impl Intersectable for Plane {
    fn intersect(&self, ray: Ray) -> Intersection {
        let mut result = Intersection::new();

        let denom = dot(self.normal, ray.direction);
        if abs(denom) > TOLERANCE {
            let t = (-self.d - dot(self.normal, ray.origin)) / denom;
            if t > MIN_HIT_DISTANCE && t < result.t {
                result.is_valid = true;
                result.t = t;
                result.normal = self.normal;
            }
        }

        return result;
    }

    fn box_clone(&self) -> Box<Intersectable> {
        Box::new((*self).clone())
    }
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: Ray) -> Intersection {
        let mut result = Intersection::new();

        let relative_origin = ray.origin - self.position;
        let a = dot(ray.direction, ray.direction);
        let b = 2.0 * dot(ray.direction, relative_origin);
        let c = dot(relative_origin, relative_origin) - square(self.r);
        let det = square(b) - 4.0 * a * c;

        if det > TOLERANCE {
            let det_sqrt = sqrt(det);
            let inv_2a = 1.0 / (2.0 * a);
            let t1 = (-b + det_sqrt) * inv_2a;
            let t2 = (-b - det_sqrt) * inv_2a;

            let t = if (t2 > TOLERANCE) && (t2 < t1) { t2 } else { t1 };
            if t > MIN_HIT_DISTANCE {
                result.is_valid = true;
                result.t = t;
                result.normal = safe_normalize(ray_point(ray, t) - self.position);
            }
        }

        return result;
    }

    fn box_clone(&self) -> Box<Intersectable> {
        Box::new((*self).clone())
    }
}

impl Intersectable for World {
    fn intersect(&self, ray: Ray) -> Intersection {

        let mut result = Intersection::new();

        for object in self.objects.iter() {
            let intersection = object.geometry.intersect(ray);
            if intersection.is_valid && intersection.t < result.t {
                result = intersection;
                result.material = object.material;
            }
        }

        result.position = ray_point(ray, result.t);

        return result;
    }

    fn box_clone(&self) -> Box<Intersectable> {
        Box::new((*self).clone())
    }
}

impl World {
    pub fn new(objects: Vec<Object>, camera: Camera) -> World {
        World {
            objects: objects,
            camera: camera,
        }
    }

    fn cast_ray(&self, ray: Ray) -> Color {
        let intersection = self.intersect(ray);

        let result: Color;
        let light_pos = Vector3::new(0.0, 0.0, 0.0);

        if intersection.is_valid {
            let light_vec = light_pos - intersection.position;
            let light_vec_length = length(light_vec);

            let shadow_ray = Ray::new(intersection.position, light_vec);
            let test = self.intersect(shadow_ray);

            if !test.is_valid || test.t > light_vec_length {
                let n = safe_normalize(intersection.normal);
                let v = light_vec / light_vec_length;
                let ndotl = saturate(dot(n, v));

                result = (intersection.material.albedo * ndotl) / 3.1415957;
            } else {
                result = Color::BLACK;
            }
        } else {
            result = Color::grey(0.2);
        }

        return result;
    }

    pub fn raytrace(&self, image: &Image, sender: Sender<(u32, u32, Color)>) {

        let width = image.width;
        let height = image.height;
        let x_slices = 2;
        let y_slices = 2;

        for i in 0..x_slices {
            let x_beg = i * (width / x_slices);
            let x_end = (i + 1) * (width / x_slices);

            for j in 0..y_slices {
                let y_beg = j * (height / y_slices);
                let y_end = (j + 1) * (height / y_slices);

                self.spawn_thread(&sender, x_beg, x_end, y_beg, y_end, image);
            }
        }
    }

    fn spawn_thread(&self, sender: &Sender<(u32, u32, Color)>,
                    x_slice_begin: u32, x_slice_end: u32,
                    y_slice_begin: u32, y_slice_end: u32, image: &Image)
    {
        let world = self.clone();
        let s = sender.clone();

        let width = image.width;
        let height = image.height;
        let samples = image.samples;

        thread::spawn(move || {
            world.raytrace_sub(s, x_slice_begin, x_slice_end,
                               y_slice_begin, y_slice_end,
                               width, height, samples);
        });
    }

    fn raytrace_sub(&self, sender: Sender<(u32, u32, Color)>,
                    x_slice_begin: u32, x_slice_end: u32,
                    y_slice_begin: u32, y_slice_end: u32,
                    width: u32, height: u32, samples: u32) {

        let inv_image_width = 1.0 / width as f32;
        let inv_image_height = 1.0 / height as f32;

        let between = Range::new(0.0, 1.0);
        let mut rng = rand::thread_rng();

        for y in y_slice_begin..y_slice_end {
            for x in x_slice_begin..x_slice_end {

                let mut color = Color::BLACK;
                for _s in 0..samples {
                    let _x = x as f32 + between.ind_sample(&mut rng);
                    let _y = y as f32 + between.ind_sample(&mut rng);

                    let u = _x * inv_image_width * 2.0 - 1.0;
                    let v = _y * inv_image_height * 2.0 - 1.0;

                    let ray = self.camera.get_ray(u, v);
                    color += self.cast_ray(ray);
                }

                let final_color = color / samples as f32;
                sender.send((x, y, final_color)).unwrap();
            }
        }
    }
}

unsafe impl Send for World {}
unsafe impl Sync for World {}

const MIN_HIT_DISTANCE: f32 = 1e-3;
const TOLERANCE: f32 = 1e-5;


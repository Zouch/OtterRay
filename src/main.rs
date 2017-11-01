use std::{u8, u32, f32};

use std::fs::File;
use std::io::BufWriter;

use std::string::String;
use std::vec::Vec;

mod math;
use math::*;

extern crate png;
use png::HasParameters;

extern crate rand;
use rand::Rng;
use rand::distributions::{IndependentSample, Range};


struct Image {
    width: u32,
    height: u32,
    data: Vec<Color>,
}

fn get_image_data(image: &Image) -> Vec<u8> {
    let size = image.data.len() * 4;
    let mut result: Vec<u8> = Vec::with_capacity(size);

    for c in image.data.iter() {
        result.push((c.red * 255.0) as u8);
        result.push((c.green * 255.0) as u8);
        result.push((c.blue * 255.0) as u8);
        result.push(255);
    }

    return result;
}

fn write_png(filepath: String, image: &Image) -> () {
    let file = File::create(filepath).unwrap();
    let ref mut bufwriter = BufWriter::new(file);

    let mut encoder = png::Encoder::new(bufwriter, image.width, image.height);
    encoder.set(png::ColorType::RGBA).set(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    let data = get_image_data(image);
    writer.write_image_data(&data).unwrap();
}

struct Intersection {
    t: f32,
    position: Vector3,
    normal: Vector3,
    material_index: u32,
    is_valid: bool
}

impl Intersection {
    fn new() -> Intersection {
        Intersection{
            t: f32::MAX,
            position: Vector3::new(0.0, 0.0, 0.0),
            normal: Vector3::new(0.0, 0.0, 0.0),
            material_index: 0,
            is_valid: false,
        }
    }
}

const MIN_HIT_DISTANCE: f32 = 1e-3;
const TOLERANCE: f32 = 1e-5;

trait Intersectable {
    fn intersect(&self, ray: Ray) -> Intersection;
}

#[derive(Debug)]
struct Plane { normal: Vector3, d: f32 }

#[derive(Debug)]
struct Sphere { position: Vector3, r: f32 }

struct Object {
    geometry: Box<Intersectable>,
    material_index: u32,
}

#[derive(Debug)]
struct Material {
    albedo: Color,
}

struct World {
    objects: Vec<Object>,
    materials: Vec<Material>,
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
}

impl Intersectable for World {
    fn intersect(&self, ray: Ray) -> Intersection {

        let mut result = Intersection::new();

        for object in self.objects.iter() {
            let intersection = object.geometry.intersect(ray);
            if intersection.is_valid && intersection.t < result.t {
                result = intersection;
                result.material_index = object.material_index;
            }
        }

        result.position = ray_point(ray, result.t);

        return result;
    }
}

impl World {
    fn cast_ray(&self, ray: Ray) -> Color {
        let intersection = self.intersect(ray);
        let albedo = self.materials[intersection.material_index as usize].albedo;

        let light_pos = Vector3::new(0.0, 0.0, 10.0);
        let ndotl = saturate(dot(safe_normalize(intersection.normal),
                                 safe_normalize(light_pos - intersection.position)));


        let result: Color;

        let shadow_ray = Ray::new(intersection.position, light_pos - intersection.position);
        let test = self.intersect(shadow_ray);

        if !test.is_valid {
            result = albedo * ndotl;
        } else {
            result = Color::BLACK;
        }

        return result;
    }
}

fn make_plane(n: Vector3, d: f32, mat_index: u32) -> Object {
    let plane = Plane { normal: n, d: d };
    let result = Object {
        geometry: Box::new(plane),
        material_index: mat_index
    };
    return result;
}

fn make_sphere(p: Vector3, r: f32, mat_index: u32) -> Object {
    let sphere = Sphere { position: p, r: r };
    let result = Object {
        geometry: Box::new(sphere),
        material_index: mat_index
    };
    return result;
}

struct Film {
    distance: f32,
    half_width: f32,
    half_height: f32,
    center: Vector3,
}

struct Camera {
    position: Vector3,
    x_axis: Vector3,
    y_axis: Vector3,
    z_axis: Vector3,

    film: Film,
}

impl Camera {
    fn new(image_width: u32, image_height: u32, film_distance: f32) -> Camera {
        let fw: f32;
        let fh: f32;

        if image_width > image_height {
            fw = 1.0;
            fh = fw * (image_height as f32) / (image_width as f32);
        } else if image_width < image_height {
            fh = 1.0;
            fw = fh * (image_width as f32) / (image_height as f32);
        } else {
            fw = 1.0;
            fh = 1.0;
        }

        let film = Film {
            distance: film_distance,
            half_width: fw * 0.5,
            half_height: fh * 0.5,
            center: Vector3::new(0.0, 0.0, 0.0),
        };

        Camera {
            position: Vector3::new(0.0, 0.0, 0.0),
            x_axis: Vector3::new(0.0, 0.0, 0.0),
            y_axis: Vector3::new(0.0, 0.0, 0.0),
            z_axis: Vector3::new(0.0, 0.0, 0.0),
            film: film,
        }
    }

    fn look_at(&mut self, position: Vector3, target: Vector3) {
        self.position = position;
        self.z_axis = safe_normalize(self.position - target);
        self.x_axis = safe_normalize(cross(Vector3::unit_z(), self.z_axis));
        self.y_axis = safe_normalize(cross(self.x_axis, self.z_axis));

        self.film.center = self.position - self.film.distance * self.z_axis;
    }

    fn get_ray(&self, u: f32, v: f32) -> Ray {
        let x_axis = u * self.film.half_width * self.x_axis;
        let y_axis = v * self.film.half_height * self.y_axis;
        let film_position = self.film.center + x_axis + y_axis;

        Ray::new(self.position, film_position - self.position)
    }
}

fn main()
{
    let mut image = Image {
        data: Vec::new(),
        width: 1024,
        height: 768
    };
    image.data.resize((image.width * image.height) as usize, Color::BLACK);

    let plane = make_plane(Vector3::new(0.0, 0.0, 1.0), 1.0, 1);
    let sphere = make_sphere(Vector3::new(0.0, 0.0, 0.0), 1.0, 2);

    let materials = vec![
        Material{ albedo: Color::new(0.1, 0.1, 0.1) },
        Material{ albedo: Color::new(0.0, 1.0, 0.0) },
        Material{ albedo: Color::new(0.0, 0.0, 1.0) },
    ];

    let world = World {
        objects: vec![plane, sphere],
        materials: materials,
    };

    let inv_image_width = 1.0 / image.width as f32;
    let inv_image_height = 1.0 / image.height as f32;

    let mut camera = Camera::new(image.width, image.height, 1.0);
    camera.look_at(Vector3::new(0.0, -10.0, 1.0), Vector3::zero());

    let between = Range::new(-1.0, 1.0);
    let mut rng = rand::thread_rng();

    for y in 0..image.height {
        for x in 0..image.width {

            let mut color = Color::BLACK;
            for _s in 0..64 {
                let _x = x as f32 + between.ind_sample(&mut rng);
                let _y = y as f32 + between.ind_sample(&mut rng);

                let u = _x * inv_image_width * 2.0 - 1.0;
                let v = _y * inv_image_height * 2.0 - 1.0;

                let ray = camera.get_ray(u, v);
                color += world.cast_ray(ray);
            }

            let final_color = color / 64.0;
            image.data[(y * image.width + x) as usize] = final_color;
        }
    }

    write_png("test.png".to_string(), &image);
}

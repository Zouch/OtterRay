use std::sync::mpsc::channel;
use std::time::Instant;

extern crate rand;
use rand::distributions::{IndependentSample, Range};

mod math;
use math::*;

mod utils;
use utils::Image;

mod raytracer;
use raytracer::*;

fn main()
{
    let mut image = Image::new(1920, 1080, 64);

    let materials = vec![
        Material::new(Color::grey(0.2)),
        Material::new(Color::RED),
        Material::new(Color::GREEN),
        Material::new(Color::BLUE),
        Material::new(Color::YELLOW),
        Material::new(Color::MAGENTA),
        Material::new(Color::CYAN),
        Material::new(Color::WHITE),
    ];

    let mut objects = Vec::new();

    let between = Range::new(0.0, 1.0);
    let mut rng = rand::thread_rng();

    objects.push(make_plane(Vector3::new(0.0, 0.0, 1.0), 1.0, 1));
    let mat_count = (materials.len() - 1) as f32;

    for _ in 0..50 {
        let mat_idx = (between.ind_sample(&mut rng) * mat_count + 1.0) as u32;
        let r = between.ind_sample(&mut rng) * 0.5;
        let p = Vector3::new(
            between.ind_sample(&mut rng) * 10.0 - 5.0,
            between.ind_sample(&mut rng) * 10.0 - 5.0,
            between.ind_sample(&mut rng) * 5.0,
        );

        objects.push(make_sphere(p, r, mat_idx));
    }

    let mut camera = Camera::new(image.width, image.height, 1.0);
    camera.look_at(Vector3::new(0.0, -15.0, 5.0), Vector3::new(0.0, 0.0, 2.5));

    let world = World::new(objects, materials, camera);

    let now = Instant::now();

    let (tx, rx) = channel();
    world.raytrace(&image, tx);

    loop {
        match rx.recv() {
            Ok((x, y, color)) => {
                image.set_pixel_color(x, y, color);
            },
            Err(_) => break,
        }
    }

    let t1 = now.elapsed();
    image.write_png("test.png".to_string());
    let t2 = now.elapsed() - t1;

    let t1_s = t1.as_secs() as f32 + (t1.subsec_nanos() as f32) * 1e-9;
    let t2_s = t2.as_secs() as f32 + (t2.subsec_nanos() as f32) * 1e-9;

    println!("Raytracing took {}s, writing image took {}s", t1_s, t2_s);
}

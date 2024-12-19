mod camera;
mod hittable;
mod ray;
mod sphere;
mod utils;

use camera::Camera;
use hittable::HittableList;
use nalgebra::Vector3;
use rayon::prelude::*;
use sphere::Sphere;
use std::sync::Arc;

fn random_double() -> f64 {
    rand::random::<f64>()
}

fn main() {
    // Image settings
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL: i32 = 100;
    const MAX_DEPTH: i32 = 8;

    // World setup
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(
        Vector3::new(0.0, -100.5, -1.0),
        100.0,
    )));
    let world = Arc::new(world);

    // Camera setup
    let camera = Arc::new(Camera::new(ASPECT_RATIO));

    // Create a vector to store all pixels
    let pixels: Vec<Vector3<f64>> = (0..IMAGE_HEIGHT)
        .into_par_iter()
        .rev()
        .flat_map(|j| {
            let world = Arc::clone(&world);
            let camera = Arc::clone(&camera);
            (0..IMAGE_WIDTH).into_par_iter().map(move |i| {
                let mut pixel_color = Vector3::new(0.0, 0.0, 0.0);
                for _ in 0..SAMPLES_PER_PIXEL {
                    let u = (i as f64 + random_double()) / (IMAGE_WIDTH - 1) as f64;
                    let v = (j as f64 + random_double()) / (IMAGE_HEIGHT - 1) as f64;
                    let r = camera.get_ray(u, v);
                    pixel_color += r.color(&world, MAX_DEPTH);
                }
                // Scale color by the number of samples
                pixel_color / SAMPLES_PER_PIXEL as f64
            })
        })
        .collect();

    // Output the image
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);
    pixels.iter().for_each(|pixel| {
        let r = utils::linear_to_gamma(pixel.x);
        let g = utils::linear_to_gamma(pixel.y);
        let b = utils::linear_to_gamma(pixel.z);
        let ir = (255.99 * r.clamp(0.0, 0.999)) as u32;
        let ig = (255.99 * g.clamp(0.0, 0.999)) as u32;
        let ib = (255.99 * b.clamp(0.0, 0.999)) as u32;
        println!("{} {} {}", ir, ig, ib);
    });
}

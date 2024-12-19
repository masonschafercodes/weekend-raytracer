mod camera;
mod hittable;
mod material;
mod ray;
mod sphere;
mod utils;

use camera::Camera;
use hittable::HittableList;
use indicatif::ProgressBar;
use material::{Dielectric, Lambertian, Metal};
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
    let lookfrom = Vector3::new(-2.0, 2.0, 1.0); // Camera position
    let lookat = Vector3::new(0.0, 0.0, -1.0); // Point camera looks at
    let vup = Vector3::new(0.0, 1.0, 0.0); // Camera's up vector
    let vfov = 20.0; // Vertical field of view in degrees

    const IMAGE_WIDTH: u32 = 1200;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL: i32 = 500;
    const MAX_DEPTH: i32 = 50;

    // World setup
    let mut world = HittableList::new();

    let ground_material = Arc::new(Lambertian::new(Vector3::new(0.5, 0.5, 0.5)));
    world.add(Box::new(Sphere::new(
        Vector3::new(0.0, -100.5, -1.0),
        100.0,
        ground_material,
    )));

    let center_material = Arc::new(Lambertian::new(Vector3::new(0.7, 0.3, 0.3)));
    world.add(Box::new(Sphere::new(
        Vector3::new(0.0, 0.0, -1.0),
        0.5,
        center_material,
    )));

    let left_material = Arc::new(Dielectric::new(1.5));
    world.add(Box::new(Sphere::new(
        Vector3::new(-1.0, 0.0, -1.0),
        0.5,
        left_material.clone(),
    )));
    world.add(Box::new(Sphere::new(
        Vector3::new(-1.0, 0.0, -1.0),
        -0.45,
        left_material.clone(),
    )));

    let right_material = Arc::new(Metal::new(Vector3::new(0.8, 0.6, 0.2), 0.0));
    world.add(Box::new(Sphere::new(
        Vector3::new(1.0, 0.0, -1.0),
        0.5,
        right_material,
    )));

    let world = Arc::new(world);

    // Camera setup
    let camera = Arc::new(Camera::new(lookfrom, lookat, vup, vfov, ASPECT_RATIO));

    let progress = ProgressBar::new((IMAGE_HEIGHT * IMAGE_WIDTH) as u64);

    // Create a vector to store all pixels
    let pixels: Vec<Vector3<f64>> = (0..IMAGE_HEIGHT)
        .into_par_iter()
        .rev()
        .flat_map(|j| {
            let world = Arc::clone(&world);
            let camera = Arc::clone(&camera);
            let progress = progress.clone();
            (0..IMAGE_WIDTH).into_par_iter().map(move |i| {
                let mut pixel_color = Vector3::new(0.0, 0.0, 0.0);
                for _ in 0..SAMPLES_PER_PIXEL {
                    let u = (i as f64 + random_double()) / (IMAGE_WIDTH - 1) as f64;
                    let v = (j as f64 + random_double()) / (IMAGE_HEIGHT - 1) as f64;
                    let r = camera.get_ray(u, v);
                    pixel_color += r.color(&world, MAX_DEPTH);
                }

                progress.inc(1);
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

    progress.finish();
}

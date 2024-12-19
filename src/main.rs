mod camera;
mod environment;
mod hittable;
mod material;
mod ray;
mod sphere;
mod texture;
mod utils;

use camera::Camera;
use core::time;
use environment::{Environment, SkyEnvironment};
use hittable::HittableList;
use image::{ImageBuffer, Rgb};
use indicatif::{ProgressBar, ProgressStyle};
use material::{Dielectric, Lambertian, Metal};
use nalgebra::Vector3;
use rayon::prelude::*;
use sphere::Sphere;
use std::{
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};
use texture::{CheckerTexture, MarbleTexture, SolidColor};

fn random_double() -> f64 {
    rand::random::<f64>()
}

fn main() {
    const IMAGE_WIDTH: u32 = 3840; // 4K resolution
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL: i32 = 100; // Much higher sampling for clean output
    const MAX_DEPTH: i32 = 50; // Deeper bounces for better caustics

    let lookfrom = Vector3::new(2.5, 2.0, 2.5); // Higher and further back
    let lookat = Vector3::new(0.0, 0.0, -1.0);
    let vup = Vector3::new(0.0, 1.0, 0.0);
    let aperture = 0.1;
    let dist_to_focus = (lookfrom - lookat).magnitude();
    let vfov = 20.0;

    // World setup
    let mut world = HittableList::new();

    let checker = Box::new(CheckerTexture::new(
        Vector3::new(0.2, 0.3, 0.1), // Dark green
        Vector3::new(0.9, 0.9, 0.9), // Light gray
        0.5,                         // Scale of the checker pattern
    ));
    let solid_orange = Box::new(
        SolidColor::new(Vector3::new(0.8, 0.4, 0.1)), // Orange
    );

    let ground_material = Arc::new(Lambertian::new(checker));
    world.add(Box::new(Sphere::new(
        Vector3::new(0.0, -100.5, -1.0),
        100.0,
        ground_material,
    )));

    let center_material = Arc::new(Lambertian::new(solid_orange));
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

    let marble_texture = Box::new(MarbleTexture::new(
        2.0,                            // scale adjusts the frequency of the marble pattern
        Vector3::new(0.95, 0.95, 0.95), // Base color
        Vector3::new(0.4, 0.3, 0.3),    // Vein color
    ));
    let marble_material = Arc::new(Lambertian::new(marble_texture));

    world.add(Box::new(Sphere::new(
        Vector3::new(-2.0, 0.0, -1.0),
        0.5,
        marble_material,
    )));

    let world = Arc::new(world);

    // Camera setup
    let camera = Arc::new(Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov, // fov
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    ));

    let progress = ProgressBar::new((IMAGE_HEIGHT * IMAGE_WIDTH) as u64);
    progress.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {percent}% {eta}")
            .expect("Failed to set progress bar style")
            .progress_chars("##-"),
    );

    let environment: Arc<dyn Environment> = Arc::new(SkyEnvironment::new(
        Vector3::new(0.3, 0.4, 0.6),
        Vector3::new(1.0, 0.95, 0.8),
        Vector3::new(2.0, 3.0, 1.0),
        0.015,
    ));

    let pixels: Vec<Vector3<f64>> = (0..IMAGE_HEIGHT)
        .into_par_iter()
        .rev()
        .flat_map(|j| {
            let world = Arc::clone(&world);
            let camera = Arc::clone(&camera);
            let progress = progress.clone();
            let environment = Arc::clone(&environment);
            (0..IMAGE_WIDTH).into_par_iter().map(move |i| {
                let mut pixel_color = Vector3::new(0.0, 0.0, 0.0);
                for _ in 0..SAMPLES_PER_PIXEL {
                    let u = (i as f64 + random_double()) / (IMAGE_WIDTH - 1) as f64;
                    let v = (j as f64 + random_double()) / (IMAGE_HEIGHT - 1) as f64;
                    let r = camera.get_ray(u, v);
                    pixel_color += r.color(&world, &environment, MAX_DEPTH);
                }

                progress.inc(1);
                pixel_color / SAMPLES_PER_PIXEL as f64
            })
        })
        .collect();

    let mut img = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    for (i, pixel) in pixels.iter().enumerate() {
        let x = i as u32 % IMAGE_WIDTH;
        let y = i as u32 / IMAGE_WIDTH;

        let r = (255.99 * utils::linear_to_gamma(pixel.x).clamp(0.0, 0.999)) as u8;
        let g = (255.99 * utils::linear_to_gamma(pixel.y).clamp(0.0, 0.999)) as u8;
        let b = (255.99 * utils::linear_to_gamma(pixel.z).clamp(0.0, 0.999)) as u8;

        img.put_pixel(x, y, Rgb([r, g, b]));
    }

    let time_for_render_image_name = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();
    let image_name = format!("render_{}.png", time_for_render_image_name);
    img.save(image_name).expect("Failed to save image");

    progress.finish();
}

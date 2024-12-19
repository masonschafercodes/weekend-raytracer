mod ray_math;

use ray_math::Camera;
use ray_math::HittableList;
use ray_math::Sphere;
use ray_math::Vec3;

use rayon::prelude::*;
use std::sync::Arc; // Add this import

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
    world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));
    let world = Arc::new(world); // Wrap world in Arc

    // Camera setup
    let camera = Arc::new(Camera::new(ASPECT_RATIO)); // Also wrap camera in Arc

    // Create a vector to store all pixels
    let pixels: Vec<Vec3> = (0..IMAGE_HEIGHT)
        .into_par_iter()
        .rev()
        .flat_map(|j| {
            let world = Arc::clone(&world); // Clone the Arc for this iteration
            let camera = Arc::clone(&camera);
            (0..IMAGE_WIDTH).into_par_iter().map(move |i| {
                let mut pixel_color = Vec3::ZERO;
                for _ in 0..SAMPLES_PER_PIXEL {
                    let u = (i as f64 + random_double()) / (IMAGE_WIDTH - 1) as f64;
                    let v = (j as f64 + random_double()) / (IMAGE_HEIGHT - 1) as f64;
                    let r = camera.get_ray(u, v);
                    pixel_color = pixel_color + r.color(&world, MAX_DEPTH);
                }
                pixel_color
            })
        })
        .collect();

    // Output the image
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);
    pixels
        .iter()
        .for_each(|pixel| pixel.write_color(SAMPLES_PER_PIXEL));
}

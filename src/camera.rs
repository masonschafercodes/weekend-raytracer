use nalgebra::Vector3;
use std::f64::consts::PI;

use crate::ray::Ray;

#[derive(Clone, Copy, Debug)]
pub struct Camera {
    origin: Vector3<f64>,
    lower_left_corner: Vector3<f64>,
    horizontal: Vector3<f64>,
    vertical: Vector3<f64>,
}

impl Camera {
    pub fn new(
        lookfrom: Vector3<f64>,
        lookat: Vector3<f64>,
        vup: Vector3<f64>,
        vfov: f64, // vertical field-of-view in degrees
        aspect_ratio: f64,
    ) -> Self {
        let theta = vfov * PI / 180.0;
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).normalize();
        let u = vup.cross(&w).normalize();
        let v = w.cross(&u);

        let origin = lookfrom;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w;

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin,
        )
    }
}

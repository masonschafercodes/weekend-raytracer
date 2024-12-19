use nalgebra::Vector3;
use rand::random;
use std::f64::consts::PI;

use crate::ray::Ray;

#[derive(Clone, Copy, Debug)]
pub struct Camera {
    origin: Vector3<f64>,
    lower_left_corner: Vector3<f64>,
    horizontal: Vector3<f64>,
    vertical: Vector3<f64>,
    u: Vector3<f64>, // Camera basis vectors
    v: Vector3<f64>,
    w: Vector3<f64>,
    lens_radius: f64, // For depth of field
}

impl Camera {
    pub fn new(
        lookfrom: Vector3<f64>,
        lookat: Vector3<f64>,
        vup: Vector3<f64>,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,   // Diameter of the lens
        focus_dist: f64, // Distance to the focus plane
    ) -> Self {
        let theta = vfov * PI / 180.0;
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).normalize();
        let u = vup.cross(&w).normalize();
        let v = w.cross(&u);

        let origin = lookfrom;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius: aperture / 2.0,
        }
    }

    fn random_in_unit_disk() -> Vector3<f64> {
        loop {
            let p = 2.0 * Vector3::new(random::<f64>(), random::<f64>(), 0.0)
                - Vector3::new(1.0, 1.0, 0.0);
            if p.magnitude_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * Self::random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
        )
    }
}

use crate::ray::Ray;
use nalgebra::Vector3;

pub trait Environment: Send + Sync {
    fn background_color(&self, ray: &Ray) -> Vector3<f64>;
}

pub struct GradientEnvironment {
    sky_top: Vector3<f64>,
    sky_bottom: Vector3<f64>,
}

impl GradientEnvironment {
    pub fn new(sky_top: Vector3<f64>, sky_bottom: Vector3<f64>) -> Self {
        Self {
            sky_top,
            sky_bottom,
        }
    }
}

impl Environment for GradientEnvironment {
    fn background_color(&self, ray: &Ray) -> Vector3<f64> {
        let unit_direction = ray.direction().normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * self.sky_bottom + t * self.sky_top
    }
}

pub struct SkyEnvironment {
    sky_color: Vector3<f64>,
    sun_color: Vector3<f64>,
    sun_direction: Vector3<f64>,
    sun_angular_size: f64,
}

impl SkyEnvironment {
    pub fn new(
        sky_color: Vector3<f64>,
        sun_color: Vector3<f64>,
        sun_direction: Vector3<f64>,
        sun_angular_size: f64,
    ) -> Self {
        Self {
            sky_color,
            sun_color,
            sun_direction: sun_direction.normalize(),
            sun_angular_size,
        }
    }
}

impl Environment for SkyEnvironment {
    fn background_color(&self, ray: &Ray) -> Vector3<f64> {
        let unit_direction = ray.direction().normalize();
        let sun_alignment = unit_direction.dot(&self.sun_direction);

        if sun_alignment > self.sun_angular_size.cos() {
            // Ray is hitting the sun
            self.sun_color
        } else {
            // Regular sky color with slight brightening near sun
            let sun_influence = (sun_alignment + 1.0) / 2.0;
            self.sky_color * (1.0 + 0.2 * sun_influence)
        }
    }
}

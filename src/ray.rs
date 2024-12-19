use nalgebra::Vector3;

use crate::hittable::{Hittable, HittableList};

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    origin: Vector3<f64>,
    direction: Vector3<f64>,
}

impl Ray {
    pub fn new(origin: Vector3<f64>, direction: Vector3<f64>) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f64) -> Vector3<f64> {
        self.origin + self.direction * t
    }

    pub fn origin(&self) -> Vector3<f64> {
        self.origin
    }

    pub fn direction(&self) -> Vector3<f64> {
        self.direction
    }

    pub fn color(&self, world: &HittableList, recursion_limit: i32) -> Vector3<f64> {
        if recursion_limit <= 0 {
            return Vector3::new(0.0, 0.0, 0.0);
        }

        if let Some(record) = world.hit(self, 0.001, f64::INFINITY) {
            let target = record.p + record.normal + Vector3::new(1.0, 1.0, 1.0);
            let ray = Ray::new(record.p, target - record.p);
            0.7 * ray.color(world, recursion_limit - 1) // Increased from 0.5 to 0.7
        } else {
            let unit_direction = self.direction.normalize();
            let t = 0.5 * (unit_direction.y + 1.0);
            (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0)
        }
    }
}

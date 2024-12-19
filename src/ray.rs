use std::sync::Arc;

use nalgebra::Vector3;

use crate::{
    environment::Environment,
    hittable::{Hittable, HittableList},
};

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

    pub fn color(
        &self,
        world: &HittableList,
        env: &Arc<dyn Environment>,
        recursion_limit: i32,
    ) -> Vector3<f64> {
        if recursion_limit <= 0 {
            return Vector3::new(0.0, 0.0, 0.0);
        }

        if let Some(record) = world.hit(self, 0.001, f64::INFINITY) {
            if let Some((scattered, attenuation)) = record.material.scatter(self, &record) {
                attenuation.component_mul(&scattered.color(world, env, recursion_limit - 1))
            } else {
                Vector3::new(0.0, 0.0, 0.0)
            }
        } else {
            env.background_color(self)
        }
    }
}

use std::sync::Arc;

use nalgebra::Vector3;

use crate::{
    material::{Lambertian, Material},
    ray::Ray,
    texture::SolidColor,
};

pub trait Hittable: Send + Sync {
    // Make it thread-safe
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

#[derive(Clone)]
pub struct HitRecord {
    pub p: Vector3<f64>,
    pub normal: Vector3<f64>,
    pub t: f64,
    pub front_face: bool,
    pub material: Arc<dyn Material>,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vector3<f64>) {
        self.front_face = r.direction().dot(&outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut temp_record = HitRecord {
            p: Vector3::new(0.0, 0.0, 0.0),
            normal: Vector3::new(0.0, 0.0, 0.0),
            t: 0.0,
            front_face: false,
            material: Arc::new(Lambertian::new(Box::new(
                SolidColor::new(Vector3::new(0.2, 0.3, 0.1)), // Dark green
            ))),
        };
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in self.objects.iter() {
            if let Some(record) = object.hit(r, t_min, closest_so_far) {
                hit_anything = true;
                closest_so_far = record.t;
                temp_record = record;
            }
        }

        if hit_anything {
            Some(temp_record)
        } else {
            None
        }
    }
}

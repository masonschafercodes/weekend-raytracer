use nalgebra::Vector3;
use rand::Rng;

use crate::{
    hittable::HitRecord,
    ray::Ray,
    texture::{SolidColor, Texture},
};

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

fn refract(uv: &Vector3<f64>, n: &Vector3<f64>, etai_over_etat: f64) -> Vector3<f64> {
    let uv = uv.normalize(); // Normalize the incoming direction
    let n = n.normalize(); // Normalize the normal
    let cos_theta = (-uv).dot(&n).min(1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -(1.0 - r_out_perp.magnitude_squared()).abs().sqrt() * n;
    (r_out_perp + r_out_parallel).normalize() // Normalize the result
}

pub trait Material: Send + Sync {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vector3<f64>)>;
}

pub struct Lambertian {
    albedo: Box<dyn Texture>,
}

impl Lambertian {
    pub fn new(albedo: Box<dyn Texture>) -> Self {
        Self { albedo }
    }

    pub fn from_color(color: Vector3<f64>) -> Self {
        Self {
            albedo: Box::new(SolidColor::new(color)),
        }
    }
}

pub struct Metal {
    albedo: Vector3<f64>,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vector3<f64>, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

pub struct Dielectric {
    ir: f64,
}

impl Dielectric {
    pub fn new(ir: f64) -> Self {
        Self { ir }
    }
}

fn random_in_unit_sphere() -> Vector3<f64> {
    let mut rng = rand::thread_rng();
    loop {
        let p = 2.0 * Vector3::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>())
            - Vector3::new(1.0, 1.0, 1.0);
        if p.magnitude_squared() < 1.0 {
            return p;
        }
    }
}

fn reflect(v: &Vector3<f64>, n: &Vector3<f64>) -> Vector3<f64> {
    v - 2.0 * v.dot(n) * n
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vector3<f64>)> {
        let scatter_direction = hit_record.normal + random_in_unit_sphere().normalize();
        let scattered = Ray::new(hit_record.p, scatter_direction);
        let attenuation = self.albedo.value(0.0, 0.0, &hit_record.p); // We'll add proper UV coordinates later
        Some((scattered, attenuation))
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vector3<f64>)> {
        let reflected = reflect(&ray_in.direction().normalize(), &hit_record.normal);
        let scattered = Ray::new(
            hit_record.p,
            reflected + self.fuzz * random_in_unit_sphere(),
        );
        if scattered.direction().dot(&hit_record.normal) > 0.0 {
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vector3<f64>)> {
        let refraction_ratio = if hit_record.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = ray_in.direction().normalize();
        let cos_theta = (-unit_direction).dot(&hit_record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction =
            if cannot_refract || reflectance(cos_theta, refraction_ratio) > rand::random::<f64>() {
                reflect(&unit_direction, &hit_record.normal)
            } else {
                refract(&unit_direction, &hit_record.normal, refraction_ratio)
            };

        let scattered = Ray::new(hit_record.p, direction);
        Some((scattered, Vector3::new(1.0, 1.0, 1.0)))
    }
}

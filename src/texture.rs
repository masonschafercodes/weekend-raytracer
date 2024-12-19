use nalgebra::Vector3;
use noise::NoiseFn;
use rand::random;

pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, p: &Vector3<f64>) -> Vector3<f64>;
}

pub struct SolidColor {
    color: Vector3<f64>,
}

impl SolidColor {
    pub fn new(color: Vector3<f64>) -> Self {
        Self { color }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: &Vector3<f64>) -> Vector3<f64> {
        self.color
    }
}

pub struct CheckerTexture {
    odd: Box<dyn Texture>,
    even: Box<dyn Texture>,
    scale: f64,
}

impl CheckerTexture {
    pub fn new(c1: Vector3<f64>, c2: Vector3<f64>, scale: f64) -> Self {
        Self {
            odd: Box::new(SolidColor::new(c1)),
            even: Box::new(SolidColor::new(c2)),
            scale,
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, _u: f64, _v: f64, p: &Vector3<f64>) -> Vector3<f64> {
        let x_int = (self.scale * p.x).floor() as i32;
        let y_int = (self.scale * p.y).floor() as i32;
        let z_int = (self.scale * p.z).floor() as i32;

        let is_even = (x_int + y_int + z_int) % 2 == 0;

        if is_even {
            self.even.value(_u, _v, p)
        } else {
            self.odd.value(_u, _v, p)
        }
    }
}

pub struct MarbleTexture {
    noise: noise::Perlin,
    scale: f64,
    color1: Vector3<f64>, // Base color
    color2: Vector3<f64>, // Vein color
}

impl MarbleTexture {
    pub fn new(scale: f64, color1: Vector3<f64>, color2: Vector3<f64>) -> Self {
        Self {
            noise: noise::Perlin::new(random()),
            scale,
            color1,
            color2,
        }
    }

    fn turbulence(&self, p: &Vector3<f64>, depth: i32) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = *p;
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight
                * self.noise.get([
                    temp_p.x * self.scale,
                    temp_p.y * self.scale,
                    temp_p.z * self.scale,
                ]);
            weight *= 0.5;
            temp_p *= 2.0;
        }

        accum.abs()
    }
}

impl Texture for MarbleTexture {
    fn value(&self, _u: f64, _v: f64, p: &Vector3<f64>) -> Vector3<f64> {
        let marble_pattern = (self.scale * p.x + 4.0 * self.turbulence(p, 7)).sin();

        let t = (marble_pattern + 1.0) / 2.0;
        let t = t.powf(1.5);

        self.color1 * (1.0 - t) + self.color2 * t
    }
}

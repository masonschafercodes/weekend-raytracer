use nalgebra::Vector3;

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

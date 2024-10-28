use nalgebra::Vector2;
use std::ops::AddAssign;

use crate::util::FixedDec;

#[derive(Clone, Copy)]
pub struct Zoom {
    scale: f32,
    mult: f32,
}

#[derive(Clone)]
pub struct Camera {
    pub pos: Vector2<FixedDec>,
    pub zoom: Zoom,
}

impl Camera {
    pub fn scale(&self, size: &Vector2<u32>) -> Vector2<f32> {
        let fsize: Vector2<f32> = size.cast();
        if size.x < size.y {
            Vector2::new(fsize.x / fsize.y, 1.0)
        } else {
            Vector2::new(1.0, fsize.y / fsize.x)
        }
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            pos: Vector2::zeros(),
            zoom: Zoom::new(0.0),
        }
    }
}

impl Zoom {
    pub fn new(scale: f32) -> Self {
        Self {
            scale,
            mult: mult(scale),
        }
    }
    pub fn mult(&self) -> f32 {
        self.mult
    }
}

impl AddAssign<f32> for Zoom {
    fn add_assign(&mut self, rhs: f32) {
        self.scale += rhs;
        self.mult = mult(self.scale);
    }
}

pub fn mult(scale: f32) -> f32 {
    (-scale).exp2()
}

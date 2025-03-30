use nalgebra::Vector2;
use std::ops::AddAssign;

use crate::util::FixedDec;

#[derive(Clone)]
pub struct Zoom {
    exp: f32,
    level: i32,
    mult: FixedDec,
    inv_mult: FixedDec,
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
    pub fn inv_scale(&self, size: &Vector2<u32>) -> Vector2<f32> {
        let fsize: Vector2<f32> = size.cast();
        if size.x < size.y {
            Vector2::new(fsize.y / fsize.x, 1.0)
        } else {
            Vector2::new(1.0, fsize.x / fsize.y)
        }
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            pos: Vector2::new(-0.5, 0.0).map(FixedDec::from),
            zoom: Zoom::new(0, 2.1),
        }
    }
}

impl Zoom {
    pub fn new(level: i32, scale: f32) -> Self {
        Self {
            exp: scale,
            level,
            mult: zoom_mult(level, scale),
            inv_mult: inv_zoom_mult(level, scale),
        }
    }
    pub fn mult(&self) -> &FixedDec {
        &self.mult
    }
    pub fn inv_mult(&self) -> &FixedDec {
        &self.inv_mult
    }
    pub fn level(&self) -> i32 {
        self.level
    }
    pub fn rel_zoom(&self) -> f32 {
        self.exp.exp2()
    }
}

impl AddAssign<f32> for Zoom {
    #[allow(clippy::suspicious_op_assign_impl)]
    fn add_assign(&mut self, rhs: f32) {
        self.exp -= rhs;
        while self.exp <= -0.5 {
            self.exp += 1.0;
            self.level += 1;
        }
        while self.exp > 0.5 {
            self.exp -= 1.0;
            self.level -= 1;
        }
        self.mult = zoom_mult(self.level, self.exp);
        self.inv_mult = inv_zoom_mult(self.level, self.exp);
    }
}

pub fn zoom_mult(level: i32, exp: f32) -> FixedDec {
    (FixedDec::from(1) >> level) * FixedDec::from(exp.exp2())
}

pub fn inv_zoom_mult(level: i32, exp: f32) -> FixedDec {
    (FixedDec::from(1) << level) * FixedDec::from(1.0 / exp.exp2())
}

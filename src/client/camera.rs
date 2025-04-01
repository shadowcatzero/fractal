use nalgebra::Vector2;
use std::ops::{AddAssign, Neg};

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
    pub size: Vector2<u32>,
}

impl Camera {
    pub fn world_pos(&self, screen_pos: Vector2<f32>) -> Vector2<FixedDec> {
        let mut pos = screen_pos
            .component_div(&self.size.cast())
            .add_scalar(-0.5)
            .component_mul(&self.stretch())
            .map(FixedDec::from);
        pos.y.negate();
        pos *= self.zoom.mult().clone();
        pos += &self.pos;
        pos
    }

    pub fn world_delta(&self, screen_delta: Vector2<f32>) -> Vector2<FixedDec> {
        let mut pos = screen_delta
            .component_div(&self.size.cast())
            .component_mul(&(self.stretch() * 1.5))
            .map(FixedDec::from);
        pos.y.negate();
        pos *= self.zoom.mult().clone();
        pos
    }

    pub fn stretch(&self) -> Vector2<f32> {
        let fsize: Vector2<f32> = self.size.cast();
        if self.size.x < self.size.y {
            Vector2::new(fsize.x / fsize.y, 1.0)
        } else {
            Vector2::new(1.0, fsize.y / fsize.x)
        }
    }

    pub fn inv_stretch(&self) -> Vector2<f32> {
        let fsize: Vector2<f32> = self.size.cast();
        if self.size.x < self.size.y {
            Vector2::new(fsize.y / fsize.x, 1.0)
        } else {
            Vector2::new(1.0, fsize.x / fsize.y)
        }
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            size: Vector2::zeros(),
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

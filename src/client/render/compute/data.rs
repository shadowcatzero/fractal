use nalgebra::Vector2;

use crate::util::FixedDec;

use super::Camera;

const VIEW_ALIGN: usize = 4 * 2;

pub struct ComputeView {
    pub bytes: Vec<u8>,
}

impl ComputeView {
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

impl Default for ComputeView {
    fn default() -> Self {
        let val = FixedDec::from_parts(false, 0, vec![0, 0, 0]);
        Self::new(true, Vector2::zeros(), Vector2::zeros(), 0, &val, &val, &val)
    }
}

impl ComputeView {
    fn new(
        reset: bool,
        dims: Vector2<u32>,
        stretch: Vector2<f32>,
        level: i32,
        scale: &FixedDec,
        x: &FixedDec,
        y: &FixedDec,
    ) -> Self {
        let mut bytes = Vec::new();
        bytes.extend((reset as u32).to_le_bytes());
        bytes.extend(level.to_le_bytes());
        bytes.extend(bytemuck::cast_slice(&[dims.x, dims.y]));
        bytes.extend(bytemuck::cast_slice(&[stretch.x, stretch.y]));
        scale.to_bytes(&mut bytes);
        x.to_bytes(&mut bytes);
        y.to_bytes(&mut bytes);
        let rem = bytes.len() % VIEW_ALIGN;
        if rem != 0 {
            bytes.extend((0..(VIEW_ALIGN - rem)).map(|_| 0));
        }
        Self { bytes }
    }

    pub fn from_camera(camera: &Camera, reset: bool, len: usize) -> Self {
        let mut x = camera.pos.x.clone();
        x.set_whole_len(1);
        x.set_dec_len(len as i32 - 1);
        let mut y = camera.pos.y.clone();
        y.set_whole_len(1);
        y.set_dec_len(len as i32 - 1);

        let stretch = camera.stretch();
        let mut scale = camera.zoom.mult().clone();
        scale.set_precision(len);

        Self::new(reset, camera.size, stretch, camera.zoom.level(), &scale, &x, &y)
    }
}

impl PartialEq for ComputeView {
    fn eq(&self, other: &Self) -> bool {
        self.bytes[1..] == other.bytes[1..]
    }
}

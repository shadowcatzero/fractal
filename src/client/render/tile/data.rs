use nalgebra::Vector2;

use crate::util::FixedDec;

use super::Camera;

const VIEW_ALIGN: usize = 4 * 2;

pub struct View {
    pub bytes: Vec<u8>,
}

impl View {
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

impl Default for View {
    fn default() -> Self {
        let val = FixedDec::from_parts(false, 0, vec![0, 0, 0]);
        Self::new(Vector2::zeros(), 0, &val, &val, &val)
    }
}

impl View {
    fn new(stretch: Vector2<f32>, level: i32, scale: &FixedDec, x: &FixedDec, y: &FixedDec) -> Self {
        let mut bytes = Vec::new();
        bytes.extend(bytemuck::cast_slice(&[stretch.x, stretch.y]));
        bytes.extend(level.to_le_bytes());
        scale.to_bytes(&mut bytes);
        x.to_bytes(&mut bytes);
        y.to_bytes(&mut bytes);
        let rem = bytes.len() % VIEW_ALIGN;
        if rem != 0 {
            bytes.extend((0..(VIEW_ALIGN - rem)).map(|_| 0));
        }
        Self{ bytes }
    }

    pub fn from_camera_size(camera: &Camera, size: &Vector2<u32>) -> Self {
        let mut x = camera.pos.x.clone();
        x.set_whole_len(1);
        x.set_dec_len(2);
        let mut y = camera.pos.y.clone();
        y.set_whole_len(1);
        y.set_dec_len(2);

        let fsize: Vector2<f32> = size.cast();
        let stretch = if size.x < size.y {
            Vector2::new(fsize.x / fsize.y, 1.0)
        } else {
            Vector2::new(1.0, fsize.y / fsize.x)
        };

        let mut scale = camera.zoom.mult().clone();
        scale.set_precision(3);

        Self::new(stretch, camera.zoom.level(), &scale, &x, &y)
    }
}

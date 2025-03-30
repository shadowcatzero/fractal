use nalgebra::Vector2;

use crate::client::render::CHUNK_POW;

use super::{Camera, CHUNK_WIDTH};

#[repr(C, align(8))]
#[derive(Clone, Copy, Default, PartialEq)]
pub struct WindowView {
    pub stretch: Vector2<f32>,
    pub pos: Vector2<f32>,
    pub rendered_chunks: Vector2<u32>,
}

unsafe impl bytemuck::Pod for WindowView {}
unsafe impl bytemuck::Zeroable for WindowView {}

impl WindowView {
    pub fn from_camera_size(camera: &Camera, size: &Vector2<u32>) -> Self {
        let visible_chunks = (size * 2 / CHUNK_WIDTH).add_scalar(1);
        let rendered_chunks = Vector2::new(
            visible_chunks.x.next_power_of_two(),
            visible_chunks.y.next_power_of_two(),
        );
        let adj_zoom = camera.zoom.level() - CHUNK_POW as i32;
        let pos = camera.pos.zip_map(&rendered_chunks, |pos, rc| {
            let p = (pos << adj_zoom).with_lens(1, 1);
            let (pw, pd) = p.split_whole_dec();
            let mut chunk = (pw.parts().first().unwrap_or(&0) & (rc - 1)) as f32;
            if pw.is_neg() {
                chunk = rc as f32 - chunk;
            }
            let dec = f32::from(pd);
            chunk + dec
        });

        let stretch = size.cast::<f32>() * camera.zoom.rel_zoom() / (CHUNK_WIDTH as f32);

        Self {
            pos,
            stretch,
            rendered_chunks,
        }
    }
}

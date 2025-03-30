use std::collections::HashSet;

use nalgebra::Vector2;

use crate::{client::camera::Camera, util::FixedDec};

use super::{output::WindowView, CHUNK_POW};

#[derive(Default)]
pub struct ChunkView {
    pub render: WindowView,
    pub chunk_queue: HashSet<Vector2<FixedDec>>,
    pub visible_chunks: HashSet<Vector2<FixedDec>>,
    pub snapshot: Option<Camera>,
}

impl ChunkView {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(&mut self, camera: &Camera, size: &Vector2<u32>, snapshot: bool) {
        if snapshot {
            self.snapshot = Some(camera.clone());
        }
        let render = WindowView::from_camera_size(camera, self.snapshot.as_ref(), size, snapshot);

        if self.render == render {
            return;
        }
        self.render = render;

        let corner_offset = ((size / 2).cast() * camera.zoom.rel_zoom())
            .map(|x| FixedDec::from(x) >> camera.zoom.level());
        let bot_left = &camera.pos - &corner_offset;
        let top_right = &camera.pos + &corner_offset;
        let mult = FixedDec::one() >> (CHUNK_POW as i32 - camera.zoom.level());
        let blc = bot_left
            .component_mul(&Vector2::from_element(mult.clone()))
            .map(FixedDec::floor);
        let trc = top_right
            .component_mul(&Vector2::from_element(mult))
            .map(FixedDec::floor);

        let mut visible = HashSet::new();
        let mut x = blc.x.clone();
        while x <= trc.x {
            let mut y = blc.y.clone();
            while y <= trc.y {
                visible.insert(Vector2::new(x.clone(), y.clone()));
                y += FixedDec::one();
            }
            x += FixedDec::one();
        }

        let new = visible
            .difference(&self.visible_chunks)
            .cloned()
            .collect::<Vec<_>>();
        let old = self
            .visible_chunks
            .difference(&visible)
            .cloned()
            .collect::<Vec<_>>();
        self.chunk_queue.retain(|p| !old.contains(p));
        self.chunk_queue.extend(new);
        self.visible_chunks = visible;
    }
}

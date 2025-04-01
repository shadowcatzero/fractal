use std::collections::HashSet;

use nalgebra::Vector2;

use crate::client::camera::Camera;

use super::output::WindowView;

#[derive(Default)]
pub struct ChunkView {
    pub render: WindowView,
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
    }
}

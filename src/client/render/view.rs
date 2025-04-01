use crate::client::camera::Camera;

use super::output::WindowView;

// TODO: move this out; this is not needed rn
#[derive(Default)]
pub struct ChunkView {
    pub render: WindowView,
    pub snapshot: Option<Camera>,
}

impl ChunkView {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(&mut self, camera: &Camera, snapshot: bool) {
        if snapshot {
            self.snapshot = Some(camera.clone());
        }
        let render = WindowView::from_camera(camera, self.snapshot.as_ref(), snapshot);

        if self.render == render {
            return;
        }
        self.render = render;
    }
}

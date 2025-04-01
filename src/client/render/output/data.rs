use nalgebra::Vector2;

use super::Camera;

#[repr(C, align(8))]
#[derive(Clone, Copy, Default, PartialEq)]
pub struct WindowView {
    pub stretch: Vector2<f32>,
    pub pos: Vector2<f32>,
    pub snapshot: u32,
}

unsafe impl bytemuck::Pod for WindowView {}
unsafe impl bytemuck::Zeroable for WindowView {}

impl WindowView {
    pub fn from_camera(camera: &Camera, ss_cam: Option<&Camera>, snapshot: bool) -> Self {
        let (pos, stretch) = if let Some(ss_cam) = ss_cam {
            let s_mult = camera.stretch().component_div(&ss_cam.stretch());
            let aspect = camera.inv_stretch().component_mul(&s_mult) * 2.0;
            let s = s_mult * f32::from(camera.zoom.mult() * ss_cam.zoom.inv_mult());
            (
                ((&camera.pos - &ss_cam.pos) * ss_cam.zoom.inv_mult().clone())
                    .map(f32::from)
                    .component_mul(&aspect),
                s,
            )
        } else {
            (Vector2::default(), Vector2::default())
        };

        Self {
            pos,
            stretch,
            snapshot: snapshot as u32,
        }
    }
}

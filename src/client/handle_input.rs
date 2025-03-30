use std::time::Duration;

use winit::keyboard::KeyCode as K;

use crate::util::FixedDec;

use super::Client;

impl Client<'_> {
    pub fn handle_input(&mut self, delta: Duration) {
        let Client { input, camera, .. } = self;
        let per_sec = delta.as_secs_f32();

        if input.scroll_delta != 0.0 {
            camera.zoom += input.scroll_delta / 5.0;
        }

        let speed = FixedDec::from(per_sec * 0.5) * camera.zoom.mult();
        if input.pressed(K::KeyW) {
            camera.pos.y += &speed;
        }
        if input.pressed(K::KeyA) {
            camera.pos.x -= &speed;
        }
        if input.pressed(K::KeyS) {
            camera.pos.y -= &speed;
        }
        if input.pressed(K::KeyD) {
            camera.pos.x += &speed;
        }
    }
}

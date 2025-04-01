use std::time::Duration;

use winit::{event::MouseButton, keyboard::KeyCode as K};

use crate::util::FixedDec;

use super::Client;

pub struct InputHandling {
    pub snapshot: bool,
}

impl InputHandling {
    pub fn new() -> Self {
        Self { snapshot: false }
    }
}

impl Client<'_> {
    pub fn handle_input(&mut self, delta: Duration) {
        let Client {
            input,
            camera,
            handling,
            ..
        } = self;
        if delta > Duration::from_secs_f32(0.5) {
            // skip input handling if lag spike so you don't go flying
            return;
        }
        let per_sec = delta.as_secs_f32();

        if input.scroll_delta != 0.0 {
            let old_pos = camera.world_pos(input.mouse_pos);
            camera.zoom += input.scroll_delta / 5.0;
            let new_pos = camera.world_pos(input.mouse_pos);
            camera.pos += old_pos - new_pos;
        }

        if input.mouse_pressed(MouseButton::Left)
            && (input.mouse_delta.x != 0.0 || input.mouse_delta.y != 0.0)
        {
            camera.pos -= camera.world_delta(input.mouse_delta);
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
        if input.just_pressed(K::KeyQ) || input.mouse_just_pressed(MouseButton::Right) {
            handling.snapshot = true;
        }
    }
}

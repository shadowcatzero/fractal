use std::collections::HashSet;

use nalgebra::Vector2;
use winit::{
    event::{DeviceEvent, ElementState, MouseButton, MouseScrollDelta, WindowEvent},
    keyboard::{KeyCode, PhysicalKey},
};

pub struct Input {
    pub mouse_pos: Vector2<f32>,
    pub mouse_delta: Vector2<f32>,

    pressed: HashSet<KeyCode>,
    just_pressed: HashSet<KeyCode>,

    mouse_pressed: HashSet<MouseButton>,
    mouse_just_pressed: HashSet<MouseButton>,
    mouse_just_released: HashSet<MouseButton>,

    pub scroll_delta: f32,
}

impl Input {
    pub fn new() -> Self {
        Self {
            mouse_pos: Vector2::zeros(),
            mouse_delta: Vector2::zeros(),
            pressed: HashSet::new(),
            just_pressed: HashSet::new(),
            mouse_pressed: HashSet::new(),
            mouse_just_pressed: HashSet::new(),
            mouse_just_released: HashSet::new(),
            scroll_delta: 0.0,
        }
    }
    pub fn update_device(&mut self, event: DeviceEvent) {
        match event {
            DeviceEvent::MouseWheel { delta } => {
                self.scroll_delta = match delta {
                    MouseScrollDelta::LineDelta(_, v) => v,
                    MouseScrollDelta::PixelDelta(v) => (v.y / 2.0) as f32,
                };
            }
            DeviceEvent::MouseMotion { delta } => {
                self.mouse_delta += Vector2::new(delta.0 as f32, delta.1 as f32);
            }
            _ => (),
        }
    }

    pub fn update_window(&mut self, event: WindowEvent) {
        match event {
            WindowEvent::KeyboardInput { event, .. } => {
                let code = if let PhysicalKey::Code(code) = event.physical_key {
                    code
                } else {
                    return;
                };
                match event.state {
                    ElementState::Pressed => {
                        self.just_pressed.insert(code);
                        self.pressed.insert(code);
                    }
                    ElementState::Released => {
                        self.pressed.remove(&code);
                    }
                };
            }
            WindowEvent::CursorLeft { .. } => {
                self.clear();
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.mouse_pos = Vector2::new(position.x as f32, position.y as f32);
            }
            WindowEvent::MouseInput { button, state, .. } => match state {
                ElementState::Pressed => {
                    self.mouse_just_pressed.insert(button);
                    self.mouse_pressed.insert(button);
                }
                ElementState::Released => {
                    self.mouse_pressed.remove(&button);
                    self.mouse_just_released.insert(button);
                }
            },
            WindowEvent::MouseWheel { delta, .. } => {
                self.scroll_delta = match delta {
                    MouseScrollDelta::LineDelta(_, v) => v,
                    MouseScrollDelta::PixelDelta(v) => (v.y / 2.0) as f32,
                };
            }
            _ => (),
        }
    }

    pub fn end(&mut self) {
        self.scroll_delta = 0.0;
        self.mouse_delta = Vector2::zeros();
        self.just_pressed.clear();
        self.mouse_just_pressed.clear();
        self.mouse_just_released.clear();
    }

    pub fn clear(&mut self) {
        self.pressed.clear();
        self.mouse_pressed.clear();
        self.end();
    }

    #[allow(dead_code)]
    pub fn pressed(&self, key: KeyCode) -> bool {
        self.pressed.contains(&key)
    }

    #[allow(dead_code)]
    pub fn just_pressed(&self, key: KeyCode) -> bool {
        self.just_pressed.contains(&key)
    }

    #[allow(dead_code)]
    pub fn mouse_pressed(&self, button: MouseButton) -> bool {
        self.mouse_pressed.contains(&button)
    }

    #[allow(dead_code)]
    pub fn mouse_just_pressed(&self, button: MouseButton) -> bool {
        self.mouse_just_pressed.contains(&button)
    }

    #[allow(dead_code)]
    pub fn mouse_just_released(&self, button: MouseButton) -> bool {
        self.mouse_just_released.contains(&button)
    }
}

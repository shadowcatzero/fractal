use std::{sync::Arc, time::Instant};

use camera::Camera;
use handle_input::InputHandling;
use input::Input;
use render::Renderer;
use winit::{
    event::WindowEvent,
    window::{Window, WindowAttributes},
};

mod app;
mod camera;
mod handle_input;
mod input;
mod render;

pub use app::*;

pub struct Client<'a> {
    window: Arc<Window>,
    camera: Camera,
    input: Input,
    exit: bool,
    prev_update: Instant,
    renderer: Renderer<'a>,
    handling: InputHandling,
}

impl Client<'_> {
    pub fn new(event_loop: &winit::event_loop::ActiveEventLoop) -> Self {
        let window = Arc::new(
            event_loop
                .create_window(WindowAttributes::default())
                .expect("failed to create window"),
        );
        let renderer = Renderer::new(window.clone());
        Self {
            window,
            camera: Camera::default(),
            input: Input::new(),
            exit: false,
            prev_update: Instant::now(),
            renderer,
            handling: InputHandling::new(),
        }
    }

    pub fn update(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.exit {
            event_loop.exit();
        }

        let now = Instant::now();
        self.handle_input(now - self.prev_update);
        self.input.end();

        self.prev_update = now;
    }

    pub fn window_event(&mut self, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => self.exit = true,
            WindowEvent::Resized(size) => {
                self.renderer.resize(size);
                self.camera.size = *self.renderer.size();
            }
            WindowEvent::RedrawRequested => {
                self.renderer.render(&self.camera, self.handling.snapshot);
                self.handling.snapshot = false;
                self.window.request_redraw();
            }
            _ => (),
        }
        self.input.update_window(event);
    }
}

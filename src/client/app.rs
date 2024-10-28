use winit::{application::ApplicationHandler, event_loop::ControlFlow};

use super::Client;

pub struct ClientApp<'a> {
    client: Option<Client<'a>>,
}

impl ClientApp<'_> {
    pub fn new() -> Self {
        Self { client: None }
    }
}

impl ApplicationHandler for ClientApp<'_> {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.client.is_none() {
            self.client = Some(Client::new(event_loop));
        }
        event_loop.set_control_flow(ControlFlow::Poll);
    }

    fn window_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        if let Some(c) = self.client.as_mut() {
            c.window_event(event)
        }
    }

    fn device_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _device_id: winit::event::DeviceId,
        event: winit::event::DeviceEvent,
    ) {
        if let Some(c) = self.client.as_mut() {
            c.input.update_device(event)
        }
    }

    fn about_to_wait(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if let Some(c) = self.client.as_mut() {
            c.update(event_loop)
        }
    }
}

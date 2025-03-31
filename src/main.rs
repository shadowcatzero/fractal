#![feature(bigint_helper_methods)]
#![feature(int_roundings)]

use client::ClientApp;

mod client;
mod util;

fn main() {
    let event_loop = winit::event_loop::EventLoop::new().expect("Failed to create event loop");
    event_loop
        .run_app(&mut ClientApp::new())
        .expect("Failed to run event loop");
}

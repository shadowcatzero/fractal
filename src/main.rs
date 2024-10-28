#![feature(bigint_helper_methods)]

use client::ClientApp;
use util::FixedDec;

mod client;
mod util;

fn main() {
    let a = FixedDec::from(0.75);
    println!("a = {}", a);
    let b = FixedDec::from(1.75);
    println!("b = {}", b);
    println!("a + b = {}", &a + &b);
    let c = FixedDec::from(1.0 / 16.0);
    println!("c = {}", c);
    println!("a + c = {}", &a + &c);
    println!("-a = {}", -&a);
    println!("b - a = {}", &b - &a);
    println!("-c = {}", -&c);
    // let event_loop = winit::event_loop::EventLoop::new().expect("Failed to create event loop");
    // event_loop
    //     .run_app(&mut ClientApp::new())
    //     .expect("Failed to run event loop");
}

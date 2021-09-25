mod processing;

#[macro_use]
extern crate glium;

use glium::{glutin, Surface};

fn main() {
	let event_loop = glutin::event_loop::EventLoop::new();
	let wb = glutin::window::WindowBuilder::new();
	let cb = glutin::ContextBuilder::new();
	let display = glium::Display::new(wb, cb, &event_loop).unwrap();

	event_loop.run( move |event, _target, control_flow| {

		let mut target = display.draw();
		target.clear_color(0.0, 0.0, 1.0, 1.0);
		target.finish().unwrap();

		let next_frame_time = std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
		glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

		processing::process_event(event, control_flow);
	});
}

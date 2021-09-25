#[macro_use]
extern crate glium;

use glium::glutin::window::WindowId;
use glium::glutin::event::Event;
use glium::glutin::event::WindowEvent;
use glium::glutin::event_loop::ControlFlow;
use glium::glutin;

fn process_window_event (window_id: WindowId, event: WindowEvent, control_flow: &mut ControlFlow) {
	match event {
		WindowEvent::Resized(_) => {}
		WindowEvent::Moved(_) => {}
		WindowEvent::CloseRequested => {
			*control_flow = glutin::event_loop::ControlFlow::Exit;
			return;
		}
		WindowEvent::Destroyed => {}
		WindowEvent::DroppedFile(_) => {}
		WindowEvent::HoveredFile(_) => {}
		WindowEvent::HoveredFileCancelled => {}
		WindowEvent::ReceivedCharacter(_) => {}
		WindowEvent::Focused(_) => {}
		WindowEvent::KeyboardInput { .. } => {}
		WindowEvent::ModifiersChanged(_) => {}
		WindowEvent::CursorMoved { .. } => {}
		WindowEvent::CursorEntered { .. } => {}
		WindowEvent::CursorLeft { .. } => {}
		WindowEvent::MouseWheel { .. } => {}
		WindowEvent::MouseInput { .. } => {}
		WindowEvent::TouchpadPressure { .. } => {}
		WindowEvent::AxisMotion { .. } => {}
		WindowEvent::Touch(_) => {}
		WindowEvent::ScaleFactorChanged { .. } => {}
		WindowEvent::ThemeChanged(_) => {}
	}
}

fn process_event(ev: Event<()>, control_flow: &mut ControlFlow) {
	match ev {
		Event::NewEvents(_) => {},
		Event::WindowEvent { window_id, event} => {
			process_window_event(window_id, event, control_flow);
			return;
		},
		Event::DeviceEvent { .. } => {},
		Event::UserEvent(_) => {},
		Event::Suspended => {},
		Event::Resumed => {},
		Event::MainEventsCleared => {},
		Event::RedrawRequested(_) => {},
		Event::RedrawEventsCleared => {},
		Event::LoopDestroyed => {}
	}
}

fn main() {
	use glium::glutin;

	let mut event_loop = glutin::event_loop::EventLoop::new();
	let wb = glutin::window::WindowBuilder::new();
	let cb = glutin::ContextBuilder::new();
	let display = glium::Display::new(wb, cb, &event_loop).unwrap();

	event_loop.run( move |event, _target, control_flow| {
		let next_frame_time = std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
		glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
		process_event(event, control_flow);
	});
}

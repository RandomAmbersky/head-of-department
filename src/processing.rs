	use glium::glutin::window::WindowId;
	use glium::glutin::event::Event;
	use glium::glutin::event::WindowEvent;
	use glium::glutin::event_loop::ControlFlow;

	fn process_window_event (_window_id: WindowId, event: WindowEvent, control_flow: &mut ControlFlow) {
		match event {
			WindowEvent::Resized(_) => {}
			WindowEvent::Moved(_) => {}
			WindowEvent::CloseRequested => {
				*control_flow = glium::glutin::event_loop::ControlFlow::Exit;
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

	pub fn process_event(ev: Event<()>, control_flow: &mut ControlFlow) {
		match ev {
			Event::NewEvents(_) => {},
			Event::WindowEvent { window_id, event } => {
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

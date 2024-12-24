use std::sync::Arc;

use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};

pub struct StateApplication {
    window: Option<Arc<Window>>,
}

impl StateApplication {
    pub fn new() -> Self {
        Self { window: None }
    }

    pub fn window(&self) -> Arc<Window> {
        self.window.clone().unwrap()
    }
}

impl ApplicationHandler for StateApplication {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = {
            let window = event_loop
                .create_window(Window::default_attributes().with_title("Hello!"))
                .unwrap();
            Arc::new(window)
        };
        self.window = Some(window)
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let window = &self.window.as_mut().unwrap();

        if window.id() == window_id {
            match event {
                WindowEvent::CloseRequested => {
                    event_loop.exit();
                }
                WindowEvent::Resized(_) => {}
                WindowEvent::RedrawRequested => {}
                _ => {}
            }
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        let window = &self.window.as_mut().unwrap();
        window.request_redraw();
    }
}

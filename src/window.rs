use winit::application::ApplicationHandler;
use winit::event::{ElementState, KeyEvent, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::{Key, NamedKey};
use winit::window::{Window, WindowId};

use crate::graphics::State;

pub struct StateApplication {
    state: Option<State>,
}

impl StateApplication {
    pub fn new() -> Self {
        Self { state: None }
    }
}

impl ApplicationHandler for StateApplication {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = {
            event_loop
                .create_window(Window::default_attributes().with_title("Hello!"))
                .unwrap()
        };
        self.state = Some(State::new(window));
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let window = match &self.state {
            Some(state) => state.window(),
            None => return,
        };

        if window.id() == window_id {
            match event {
                WindowEvent::CloseRequested => {
                    event_loop.exit();
                }
                WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            logical_key: key,
                            state: ElementState::Pressed,
                            ..
                        },
                    ..
                } => match key.as_ref() {
                    Key::Named(NamedKey::Escape) => {
                        event_loop.exit();
                    }
                    Key::Named(NamedKey::Space) => {
                        let state = match &mut self.state {
                            Some(state) => state,
                            None => return,
                        };
                        state.switch_pipeline()
                    }
                    _ => (),
                },
                WindowEvent::Resized(size) => {
                    let state = match &mut self.state {
                        Some(state) => state,
                        None => return,
                    };
                    state.resize(size)
                }
                WindowEvent::CursorMoved { .. } => {
                    let state = match &mut self.state {
                        Some(state) => state,
                        None => return,
                    };
                    state.input(&event)
                }
                WindowEvent::RedrawRequested => {
                    let state = match &mut self.state {
                        Some(state) => state,
                        None => return,
                    };
                    state.window().request_redraw();

                    match state.render() {
                        Ok(_) => {}
                        Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                            state.resize(state.size())
                        }
                        Err(wgpu::SurfaceError::OutOfMemory) => {
                            log::error!("OutOfMemory");
                            event_loop.exit()
                        }
                        Err(wgpu::SurfaceError::Timeout) => {
                            log::warn!("Surface timeout")
                        }
                    };
                }
                _ => {}
            }
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        let window = self.state.as_ref().unwrap().window();
        window.request_redraw();
    }
}

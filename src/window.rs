use anyhow::Context;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};

use crate::game::{self, GameState};
use crate::graphics::State;

pub struct StateApplication {
    state: Option<game::GameState>,
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
        let renderer = State::new(window);

        match &mut self.state {
            Some(state) => {
                state.update_renderer(renderer);
            }
            None => self.state = Some(GameState::new(renderer)),
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        let state = match &mut self.state {
            Some(state) => state,
            None => return,
        };

        // Is this basically sampling of input? The game state does not progress here, rather it
        // does in about_to_wait.
        state.input(&event);

        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(_size) => {}
            WindowEvent::RedrawRequested => {
                // Not sure about these.
                state.render().context("when rendering").unwrap();
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        let state = match &mut self.state {
            Some(state) => state,
            None => return,
        };
        state.update();

        if state.exit() {
            event_loop.exit();
        }
    }
}

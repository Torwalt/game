use std::time::{Duration, Instant};

use anyhow::Context;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};

use crate::game::{self, GameState};
use crate::graphics::State;

pub struct Config {
    max_frame_time: Duration,
    target_frame_time: Duration,
}

impl Config {
    pub(crate) fn new(fps: u32, max_frame_time: Duration) -> Self {
        Self {
            target_frame_time: Duration::from_secs_f64(1. / fps as f64),
            max_frame_time,
        }
    }
}

pub struct StateApplication {
    state: Option<game::GameState>,
    accumulated_time: Duration,
    instant: Instant,
    config: Config,
}

impl StateApplication {
    pub fn new(config: Config) -> Self {
        Self {
            state: None,
            accumulated_time: Duration::ZERO,
            instant: Instant::now(),
            config,
        }
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

        let mut elapsed = self.instant.elapsed();
        self.instant = Instant::now();

        if elapsed > self.config.max_frame_time {
            elapsed = self.config.max_frame_time;
        }

        self.accumulated_time += elapsed;

        let mut _keys_updated = false;

        while self.accumulated_time > self.config.target_frame_time {
            state.update();

            if state.exit() {
                event_loop.exit();
                return;
            }

            self.accumulated_time = self
                .accumulated_time
                .saturating_sub(self.config.target_frame_time);

            let _blending_factor =
                self.accumulated_time.as_secs_f64() / self.config.target_frame_time.as_secs_f64();

            state.render().unwrap();
        }
    }
}

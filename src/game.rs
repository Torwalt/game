use anyhow::{Ok, Result};
use winit::event::{Event, WindowEvent};

use crate::graphics;

use self::input::Input;

mod input;

pub struct GameState {
    renderer: graphics::State,
    input: Input,
    exit: bool,
}

impl GameState {
    pub fn new(renderer: graphics::State) -> Self {
        let input = Input::new();

        Self {
            renderer,
            input,
            exit: false,
        }
    }

    pub fn update_renderer(&mut self, renderer: graphics::State) {
        self.renderer = renderer
    }

    pub fn update(&mut self) {
        if self
            .input
            .is_logical_key_pressed(winit::keyboard::NamedKey::Escape)
        {
            self.exit = true;
        }
    }

    pub fn render(&mut self) -> Result<()> {
        self.renderer.render()?;
        Ok(())
    }

    pub fn input(&mut self, event: &WindowEvent) {
        self.input.process_event(event);
    }

    // NOTE: Right now no user events. When there are such, I can make `Event` generic on my user
    // event.
    pub fn handle(&self, _event: Event<()>) {}

    pub fn exit(&self) -> bool {
        self.exit
    }
}

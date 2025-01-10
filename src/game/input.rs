use std::collections::HashMap;

use winit::{
    dpi::PhysicalPosition,
    event::{ElementState, MouseButton, WindowEvent},
    keyboard::{Key, KeyCode, NamedKey, PhysicalKey},
};

/// Input state of a mouse button/keyboard key.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum InputState {
    /// The button has just been pressed.
    Pressed,
    /// The button is being held down.
    Down,
    /// The button has just been released.
    ///
    /// Note that it means that the key has **just** been released, **not** that it isn't held.
    Released,
}

impl InputState {
    pub fn is_pressed(&self) -> bool {
        matches!(self, InputState::Pressed)
    }

    pub fn is_any_down(&self) -> bool {
        matches!(self, InputState::Pressed | InputState::Down)
    }

    pub fn is_released(&self) -> bool {
        matches!(self, InputState::Released)
    }
}

pub struct Input {
    cursor_pos: PhysicalPosition<f64>,
    physical_keys: HashMap<KeyCode, InputState>,
    logical_keys: HashMap<NamedKey, InputState>,
    mouse_button: HashMap<MouseButton, InputState>,
}

impl From<ElementState> for InputState {
    #[inline]
    fn from(value: ElementState) -> Self {
        match value {
            ElementState::Pressed => InputState::Pressed,
            ElementState::Released => InputState::Released,
        }
    }
}

impl Input {
    pub fn new() -> Self {
        Self {
            cursor_pos: PhysicalPosition::new(0.0, 0.0),
            physical_keys: HashMap::default(),
            logical_keys: HashMap::default(),
            mouse_button: HashMap::default(),
        }
    }

    pub fn process_event(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::KeyboardInput { event, .. } if !event.repeat => {
                if let PhysicalKey::Code(key_code) = event.physical_key {
                    self.physical_keys.insert(key_code, event.state.into());
                }

                if let Key::Named(key) = event.logical_key {
                    self.logical_keys.insert(key, event.state.into());
                }
            }
            _ => {}
        };
    }

    pub fn is_physical_key_pressed(&self, k: KeyCode) -> bool {
        self.physical_keys
            .get(&k)
            .map_or(false, InputState::is_pressed)
    }

    pub fn is_logical_key_pressed(&self, k: NamedKey) -> bool {
        self.logical_keys
            .get(&k)
            .map_or(false, InputState::is_pressed)
    }
}

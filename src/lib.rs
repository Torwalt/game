use anyhow::Result;
use winit::event_loop::EventLoop;

use self::graphics::State;
use self::window::StateApplication;

mod graphics;
mod window;

pub async fn run() -> Result<()> {
    let event_loop = EventLoop::new()?;
    let mut input = StateApplication::new();
    event_loop.run_app(&mut input).unwrap();
    println!("loop runs!");
    State::new(input.window());

    Ok(())
}

struct Game {
    input: StateApplication,
    graphics: State,
}

impl Game {
    fn g_loop(&mut self) {}
}

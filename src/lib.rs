use std::time::Duration;

use anyhow::Result;
use winit::event_loop::{ControlFlow, EventLoop};

use self::window::{Config, StateApplication};

mod game;
mod graphics;
mod window;

pub async fn run() -> Result<()> {
    let config = Config::new(60, Duration::from_millis(17));
    let mut app = StateApplication::new(config);
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.run_app(&mut app).unwrap();

    Ok(())
}

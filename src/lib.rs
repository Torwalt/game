use anyhow::Result;
use winit::event_loop::{ControlFlow, EventLoop};

use self::window::StateApplication;

mod graphics;
mod window;

pub async fn run() -> Result<()> {
    let mut app = StateApplication::new();
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.run_app(&mut app).unwrap();

    Ok(())
}


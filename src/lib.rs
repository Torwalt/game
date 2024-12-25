use anyhow::Result;
use winit::event_loop::EventLoop;

use self::window::StateApplication;

mod graphics;
mod window;

pub async fn run() -> Result<()> {
    let mut app = StateApplication::new();
    let event_loop = EventLoop::new()?;
    event_loop.run_app(&mut app).unwrap();

    Ok(())
}

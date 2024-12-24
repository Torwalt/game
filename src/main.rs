use anyhow::Result;
use game::run;

fn main() -> Result<()> {
    pollster::block_on(run())?;
    Ok(())
}

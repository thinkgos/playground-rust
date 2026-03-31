pub mod app;
pub mod event;
pub mod ui;

use anyhow::Result;
use app::App;

fn main() -> Result<()> {
    ratatui::run(|terminal| App::new().run(terminal))?;
    Ok(())
}

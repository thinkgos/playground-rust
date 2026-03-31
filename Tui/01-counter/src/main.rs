pub mod app;
pub mod event;
pub mod ui;

use anyhow::Result;
use app::App;

fn main() -> Result<()> {
    // Create an application.
    let mut app = App::new();
    ratatui::run(|terminal| app.run(terminal))?;
    Ok(())
}

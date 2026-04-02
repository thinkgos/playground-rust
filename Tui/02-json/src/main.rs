pub mod app;
pub mod ui;

use anyhow::Result;

use app::App;

fn main() -> Result<()> {
    let mut app = App::new();
    match app.run()? {
        true => println!("{}", app.json()?),
        false => {}
    }
    Ok(())
}

pub mod app;
pub mod ui;

use anyhow::Result;

use app::App;

fn main() -> Result<()> {
    let res: Result<Option<String>> = {
        let mut app = App::new();
        let res = app.run()?;
        if res { Ok(Some(app.json()?)) } else { Ok(None) }
    };
    match res {
        Ok(Some(js)) => println!("{}", js),
        Err(e) => println!("{}", e),
        _ => {}
    }
    Ok(())
}

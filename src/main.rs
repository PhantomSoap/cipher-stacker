use std::io;
pub mod app;
pub mod ciphermod;
pub mod events;
pub mod ui;

use app::App;
use ratatui::run;

fn main() -> io::Result<()> {
    run(|terminal| {
        let mut app = App::new();
        app.run(terminal)
    })
}

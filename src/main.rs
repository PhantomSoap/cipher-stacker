use std::io;
pub mod app;
pub mod ciphermod;
pub mod events;
pub mod ui;

use app::App;

fn main() -> io::Result<()> {
    ratatui::run(|terminal| {
        let mut app = App::new();
        app.run(terminal)
    })
}

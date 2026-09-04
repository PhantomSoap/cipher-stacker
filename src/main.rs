#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

use std::io;
pub mod app;


pub mod cipherviews;
pub mod ciphertype;
pub mod ciphername;
pub mod components;
pub mod layouts;




use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::execute;
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode};


pub use components::plaintext::Plaintext;
pub use components::ciphertext::Ciphertext;

pub use components::history::History;

pub use crate::ciphertype::CipherType;
pub use crate::ciphername::CipherName;
pub use crate::components::cipher_stack::{CipherEdit,CipherStack};
pub use crate::cipherviews::{affine_ui::AffineView, atbash_ui::AtbashView, caesar_ui::CaesarView, cipherview::{AppCipher, CipherView}, rail_fence_ui::RailfenceView, vigenere_ui::VigenereView};


pub use app::App;
use ratatui::{Terminal, backend::CrosstermBackend};


pub enum Message {
    AddCipher(CipherName, Option<usize>),
    RemoveCipher(Option<usize>),
    EditCipher(CipherEdit),
    NextInStack,
    PreviousInStack,
    CipherPlaintext,
    DecipherCiphertext,
    Exit,
    Reset,
    GoHome,
    NextFocus,

}


fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout,EnableMouseCapture,EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let res = App::new().run(&mut terminal);
    
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    )?;
    res?;

    Ok(())
}

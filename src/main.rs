#![warn(clippy::pedantic)] // Catches subtle code smells and stylistic issues
#![warn(clippy::nursery)]

use std::io::{self, stdout};
pub mod app;
pub mod cipher_stack;

pub mod cipherviews;
pub mod plaintext;
pub mod ciphertext;
pub mod history;

pub mod panels;



use crossterm::cursor::{Hide, Show};
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::execute;
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode};
pub use panels::CipherAdder;

pub use plaintext::Plaintext;
pub use ciphertext::Ciphertext;

pub use history::History;
use ratatui::backend::CrosstermBackend;
use crate::cipher_stack::{CipherEdit, CipherName,CipherStack,CipherType};
pub use crate::cipherviews::cipherview::{AppCipher, CipherView};
pub use crate::panels::EditingPanel;
pub use app::App;
pub use crate::cipherviews::affine_ui::AffineView;
pub use crate::cipherviews::atbash_ui::AtbashView;
pub use crate::cipherviews::caesar_ui::CaesarView;
pub use crate::cipherviews::rail_fence_ui::RailfenceView;
pub use crate::cipherviews::vigenere_ui::VigenereView;

use ratatui::{Terminal, run};


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

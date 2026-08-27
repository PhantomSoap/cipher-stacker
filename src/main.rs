#![warn(clippy::pedantic)] // Catches subtle code smells and stylistic issues
#![warn(clippy::nursery)]

use std::io;
pub mod app;
pub mod cipher_stack;

pub mod cipherviews;
pub mod plaintext;
pub mod ciphertext;
pub mod history;
pub mod control_panel;

pub use control_panel::ControlPanel;
pub use plaintext::Plaintext;
pub use ciphertext::Ciphertext;
pub use cipher_stack::{CipherStack,CipherType};
pub use history::History;
pub use crate::cipherviews::cipherview::{AppCipher, CipherView};
pub use app::App;

use crossterm::event::KeyCode;
use ratatui::run;


pub enum Message {
    AddCipher(CipherType, Option<usize>),
    RemoveCipher(Option<usize>),
    Exit,
    Reset,
    StopCiphering,
    StartCiphering(usize),
    LookAtCipher(CipherType),
    PushChar(char),
    PopChar,
    GoHome,
    EditCipher(usize, KeyCode),
    NextCipher(CipherType),
    PreviousCipher(CipherType),
    NextInStack,
    PreviousInStack,
    None,
    NextFocus,
}


fn main() -> io::Result<()> {
    run(|terminal| {
        let mut app = App::new();
        app.run(terminal)
    })
}

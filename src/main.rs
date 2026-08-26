#![warn(clippy::pedantic)] // Catches subtle code smells and stylistic issues
#![warn(clippy::nursery)]

use std::io;
pub mod app;
pub mod cipher_stack;
pub mod events;
pub mod ui;
pub mod plaintext;
pub mod ciphertext;
pub mod history;


use app::App;
use crossterm::event::KeyCode;
use ratatui::run;

use crate::cipher_stack::CipherType;
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
}


fn main() -> io::Result<()> {
    run(|terminal| {
        let mut app = App::new();
        app.run(terminal)
    })
}

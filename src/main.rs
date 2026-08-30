#![warn(clippy::pedantic)] // Catches subtle code smells and stylistic issues
#![warn(clippy::nursery)]

use std::io;
pub mod app;
pub mod cipher_stack;

pub mod cipherviews;
pub mod plaintext;
pub mod ciphertext;
pub mod history;
pub mod cipher_adder;
pub mod cipher_editer;



pub use cipher_adder::CipherAdder;

pub use plaintext::Plaintext;
pub use ciphertext::Ciphertext;

pub use history::History;
use crate::cipher_stack::{CipherEdit, CipherName,CipherStack,CipherType};
pub use crate::cipherviews::cipherview::{AppCipher, CipherView};
pub use crate::cipher_editer::EditingPanel;
pub use app::App;
pub use crate::cipherviews::affine_ui::AffineView;
pub use crate::cipherviews::atbash_ui::AtbashView;
pub use crate::cipherviews::caesar_ui::CaesarView;
pub use crate::cipherviews::rail_fence_ui::RailfenceView;
pub use crate::cipherviews::vigenere_ui::VigenereView;

use ratatui::run;


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
    run(|terminal| {
        let mut app = App::new();
        app.run(terminal)
    })
}

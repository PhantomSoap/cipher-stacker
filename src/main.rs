#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

use std::io;
pub mod app;

pub mod ciphername;
pub mod ciphertype;
pub mod cipherviews;
pub mod components;
pub mod layouts;


use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};

pub use components::ciphertext::Ciphertext;
pub use components::inputtext::InputText;
use ratatui::layout::Rect;

use crate::app::Focus;
pub use crate::ciphername::CipherName;
pub use crate::ciphertype::CipherType;
pub use crate::cipherviews::{
    affine_ui::AffineView,
    atbash_ui::AtbashView,
    caesar_ui::CaesarView,
    cipherview::{AppCipher, CipherView},
    rail_fence_ui::RailfenceView,
    vigenere_ui::VigenereView,
};
pub use crate::components::cipher_stack::{CipherEdit, CipherStack};

pub use app::App;
use ratatui::{Terminal, backend::CrosstermBackend};

pub enum Message {
    EditCipher(CipherEdit),
    CipherInputText,
    Exit,
    Reset,
    GoHome,
    NextFocus,
    Focus(Focus)
}
const INSTRUCTIONS : [&'static str; 6] = [
    
    "[Up/Down] scroll up/down\n[Left/Right] Next Cipher to add\n[+] Add Cipher\n[Enter] Edit Selected\n[Space] Toggle History",
    "",
    "",
    "",
    "",
    "",
];

const CIPHER_INSTRUCTIONS : [&'static str; 5] = [
    "[Left/Right] Shift\n[Enter] return",
    "[Left/Right] Change a\n[Down/Up] Change b\n[Enter] return",
    "[Up/Down] Change key\n[Enter] return",
    "\n[Enter] return",
    "\n[Enter] return",
];

pub fn contains(area: Rect, x: u16, y: u16) -> bool {
    x >= area.x
        && x < area.x + area.width
        && y >= area.y
        && y < area.y + area.height
}
fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen,EnableMouseCapture)?;
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

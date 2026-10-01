#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

use std::cell::RefCell;
use std::io;
use std::rc::Rc;
pub mod app;

pub mod ciphername;
pub mod ciphertype;
pub mod cipherviews;
pub mod components;
pub mod layouts;
pub mod theme;



pub use components::ciphertext::Ciphertext;
pub use components::inputtext::InputText;
use ratatui::Terminal;
use ratatui::layout::Rect;
use ratzilla::{ DomBackend, WebRenderer};

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

pub enum Message {
    CipherText,
    Exit,
    Reset,
    GoHome,
    NextFocus,
    Focus(Focus),
}
const INSTRUCTIONS: [&'static str; 6] = [
    "[Up/Down] scroll up/down\n[Left/Right] Next Cipher to add\n[+] Add Cipher\n[Enter] Edit Selected\n[Space] Toggle History",
    "",
    "",
    "",
    "",
    "",
];

const CIPHER_INSTRUCTIONS: [&'static str; 5] = [
    "[Left/Right] Shift\n[Enter] return",
    "[Down/Up] Change a\n[Left/Right] Change b\n[Enter] return",
    "[Up/Down] Change key\n[Enter] return",
    "\n[Enter] return",
    "\n[Enter] return",
];

pub fn contains(area: Rect, x: u16, y: u16) -> bool {
    x >= area.x && x < area.x + area.width && y >= area.y && y < area.y + area.height
}
fn main() -> io::Result<()> {
    let app = Rc::new(RefCell::new(App::new()));
    let backend = DomBackend::new()?;
    let mut terminal = Terminal::new(backend)?;

    terminal.on_key_event({
        let app_clone_key = app.clone();
        move |key_event| {
            
            let mut app = app_clone_key.borrow_mut();
            
            if let Some(msg) = app.handle_keys(key_event) {
                app.update(msg);
            }

            
        }
    })?;

    terminal.on_mouse_event({
        let app_clone_mouse = app.clone();
        move |mouse_event| {
            
            let mut app = app_clone_mouse.borrow_mut();
            if let Some(msg) = app.handle_mouse(mouse_event) {
                app.update(msg);
            }
            
        }
    })?;

    terminal.draw_web(move |f| {
        let app_clone_draw = app.clone();
        let mut app = app_clone_draw.borrow_mut();
        app.draw(f)
        
    });

    
    Ok(())
}

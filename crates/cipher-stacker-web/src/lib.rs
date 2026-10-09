#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

use std::cell::RefCell;

use std::rc::Rc;
pub mod app;

pub mod ciphername;
pub mod ciphertype;
pub mod cipherviews;
pub mod components;
pub mod layouts;
pub mod theme;

pub use components::inputtext::InputText;
pub use components::output_text::OutputText;
use ratatui::Terminal;
use ratzilla::{DomBackend, WebRenderer};

use wasm_bindgen::{JsCast, JsError, JsValue};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::ClipboardEvent;

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
#[derive(Debug, Clone, Copy)]

pub enum Process {
    Encrypt,
    Decrypt,
}
pub enum Message {
    CipherText,
    Exit,
    Reset,
    GoHome,
    NextFocus,
    Focus(Focus),
    SwitchProcess(Process),
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
#[wasm_bindgen(start)]
fn run() -> Result<(), JsValue> {
    let app = Rc::new(RefCell::new(App::new()));
    let backend = DomBackend::new().map_err(|e| JsError::new(&e.to_string()))?;
    let mut terminal = Terminal::new(backend).map_err(|e| JsError::new(&e.to_string()))?;

    let app_clone_key = app.clone();
    terminal.on_key_event({
        move |key_event| {
            let mut app = app_clone_key.borrow_mut();

            if let Some(msg) = app.handle_keys(key_event) {
                app.update(msg);
            }
        }
    }).map_err(|e| JsError::new(&e.to_string()))?;
    let app_clone_mouse = app.clone();
    terminal.on_mouse_event({
        move |mouse_event| {
            let mut app = app_clone_mouse.borrow_mut();
            if let Some(msg) = app.handle_mouse(mouse_event) {
                app.update(msg);
            }
        }
    }).map_err(|e| JsError::new(&e.to_string()))?;
    let app_clone_draw = app.clone();
    terminal.draw_web(move |f| {
        let mut app = app_clone_draw.borrow_mut();
        app.draw(f)
    });

    let app_clone_paste = app.clone();

    let paste_callback = Closure::<dyn FnMut(ClipboardEvent)>::new(move |event: ClipboardEvent| {
        if let Some(clipboard_data) = event.clipboard_data() {
            if let Ok(text) = clipboard_data.get_data("text/plain") {
                let mut app = app_clone_paste.borrow_mut();

                if let Some(msg) = app.handle_paste(&text) {
                    app.update(msg);
                }
            }
        }

        event.prevent_default();
    });

    let _jsvalue = web_sys::window()
        .unwrap()
        .add_event_listener_with_callback("paste", paste_callback.as_ref().unchecked_ref());

    paste_callback.forget();

    Ok(())
}

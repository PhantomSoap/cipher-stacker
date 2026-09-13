use crossterm::event::KeyEvent;
use ratatui::{Frame, layout::Rect};
use ratatui_themekit::ThemeData;

use crate::Message;

pub mod cipher_stack;
pub mod ciphertext;
pub mod inputtext;

pub trait Component {
    fn handle_key_events(&mut self, key: KeyEvent) -> Option<Message>;
    fn draw(&self, frame: &mut Frame, area: Rect, focus: bool, t: ThemeData);
    fn update(&mut self, msg: Message) -> Option<Message>;
}

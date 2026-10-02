use ratatui::{Frame, layout::Rect};
use ratzilla::event::KeyEvent;

use crate::{Message, theme::Theme};

pub mod cipher_stack;
pub mod output_text;
pub mod inputtext;
pub mod theme_change;

pub trait Component {
    fn handle_key_events(&mut self, key: KeyEvent) -> Option<Message>;
    fn draw(&self, frame: &mut Frame, area: Rect, focus: bool, t: Theme);
    fn update(&mut self, msg: Message) -> Option<Message>;
}

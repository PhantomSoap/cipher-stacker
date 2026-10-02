use ratatui::{
    Frame, layout::Rect, widgets::{Block, Paragraph, Wrap},
};
use ratzilla::event::{KeyCode, KeyEvent};

use crate::{Message, components::Component, theme::Theme};
pub struct OutputText {
    pub text: String,
    pub scroll: u16,
}

impl OutputText {
    pub fn new(text: String) -> Self {
        Self { text, scroll: 0 }
    }
}
impl Component for OutputText {
    fn draw(&self, frame: &mut Frame, area: Rect, focus: bool, t: Theme) {
        let block = Block::bordered().title("Ciphertext").border_style(if focus {t.ring} else {t.border});

        let widget = Paragraph::new(self.text.as_str())
            .wrap(Wrap { trim: true })
            .block(block)
            .scroll((self.scroll, 0));

        frame.render_widget(widget, area);
    }

    fn handle_key_events(&mut self, key: KeyEvent) -> Option<Message> {
        

        match key.code {
            KeyCode::Esc => Some(Message::Exit),
            KeyCode::Up if self.scroll != 0 => {
                self.scroll -= 1;
                None
            }
            KeyCode::Down => {
                self.scroll += 1;
                None
            }
            KeyCode::Backspace => {
                self.text.pop();
                None
            }
            KeyCode::Tab => Some(Message::NextFocus),
            _ => None,
        }
    }

    fn update(&mut self, _msg: Message) -> Option<Message> {
        None
    }
}

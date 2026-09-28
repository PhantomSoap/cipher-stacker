use ratatui::{
    Frame, layout::Rect, widgets::{Block, Paragraph, Wrap},
};
use ratzilla::event::{KeyCode, KeyEvent};

use super::Component;
use crate::{Message, theme::Theme};

pub struct InputText {
    pub text: String,
    pub scroll: u16,
}
impl InputText {
    pub fn new(text: String) -> Self {
        Self { text, scroll: 0 }
    }
}
impl Component for InputText {
    fn draw(&self, frame: &mut Frame, area: Rect, focus: bool, t: Theme) {
        let block = Block::bordered().title("Plaintext").border_style(if focus {t.ring} else {t.border});

        let widget = Paragraph::new(format!("{}", self.text))
            .wrap(Wrap { trim: false })
            .block(block)
            .scroll((0, self.scroll));

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
                Some(Message::CipherText)
            }
            KeyCode::Char(c) => {
                self.text.push(c);
                Some(Message::CipherText)
            }
            KeyCode::Tab => Some(Message::NextFocus),

            _ => None,
        }
    }

    fn update(&mut self, _msg: Message) -> Option<Message> {
        None
    }
}

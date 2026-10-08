use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Paragraph, Wrap},
};
use crossterm::event::{KeyCode, KeyEvent};

use crate::{Message, Process, components::Component, theme::Theme};
pub struct OutputText {
    pub text: String,
    pub scroll: u16,
    pub process: Process,
}

impl OutputText {
    pub fn new(text: String, process: Process) -> Self {
        Self {
            text,
            scroll: 0,
            process,
        }
    }
}
impl Component for OutputText {
    fn draw(&self, frame: &mut Frame, area: Rect, focus: bool, t: Theme) {
        let block = Block::bordered()
            .title(if let Process::Encrypt = self.process {
                "Ciphertext"
            } else {
                "Plaintext"
            })
            .border_style(if focus { t.ring } else { t.border });

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

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    Frame, layout::Rect, style::{Color, Style}, widgets::{Block, Paragraph, Wrap},
};

use super::Component;
use crate::Message;

pub struct Plaintext {
    pub text: String,
    pub scroll: u16,
}
impl Plaintext {
    pub fn new(text: String) -> Self {
        Self { text, scroll: 0 }
    }
}
impl Component for Plaintext {
    fn draw(&self, frame: &mut Frame, area: Rect, focus: bool) {
        let style = if focus {Style::default().fg(Color::Blue)} else {Style::default()};
        let widget = 
            Paragraph::new(format!("{}", self.text))
                .wrap(Wrap { trim: false })
                .block(
                    Block::bordered()
                        .title_top("Plaintext")
                        .border_style(style),
                )
                .scroll((
                    0,self.scroll
                ));
        
        frame.render_widget(widget, area);
    }

    fn handle_key_events(&mut self, key: KeyEvent) -> Option<Message> {
        if  key.kind == KeyEventKind::Release {
            return None;
        }

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
                Some(Message::CipherPlaintext)
            }
            KeyCode::Char(c) => {
                self.text.push(c);
                Some(Message::CipherPlaintext)
            }
            KeyCode::Tab => Some(Message::NextFocus),

            _ => None,
        }
    }

    fn update(&mut self, _msg: Message) -> Option<Message> {
        None
    }
}

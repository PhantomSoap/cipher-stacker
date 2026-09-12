use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    Frame, layout::Rect, style::{Color, Style}, widgets::{Block, Paragraph, Wrap},
};
use ratatui_themekit::{ThemeData, ThemeExt};

use crate::{Message, components::Component};
pub struct Ciphertext {
    pub text: String,
    pub scroll: u16,
}

impl Ciphertext {
    pub fn new(text: String) -> Self {
        Self { text, scroll: 0 }
    }
}
impl Component for Ciphertext {
    fn draw(&self, frame: &mut Frame, area: Rect, focus: bool,t : ThemeData) {
        let block = t.block("Ciphertext").focused(focus).build();

        let widget = 
            Paragraph::new(self.text.as_str())
                .wrap(Wrap { trim: true })
                .block(
                    block
                )
                .scroll((
                    self.scroll,
                    0,
                ));
        
        frame.render_widget(widget, area);
    }

    fn handle_key_events(&mut self, key: KeyEvent) -> Option<Message> {
        if let KeyEventKind::Release = key.kind {
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
                None
            }
            KeyCode::Char(c) => {
                self.text.push(c);
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


use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, MouseEvent};
use ratatui::{Frame, layout::Rect, style::{Color, Style}, text::Text, widgets::{Block, Paragraph, Wrap}};

use crate::Message;
pub struct Ciphertext {
    pub text : String,
    pub scroll : usize,
}

impl Ciphertext {
    pub fn new(text : String) -> Self {
        Self {
            text,
            scroll : 0
        }
    }

    pub fn draw(&self,frame : &mut Frame,area : Rect) {
        let widget = Paragraph::new(format!("Ciphertext: {}",self.text))
            .wrap(Wrap { trim : true})
            .block(Block::bordered())//.border_style(Color::Blue))
            .scroll((self.scroll as u16,0));
        frame.render_widget(widget, area);
    }

    pub fn handle_key_events(&mut self,key : KeyEvent)  -> Option<Message>{
        if let KeyEventKind::Release =  key.kind {
            return None
        }

        match key.code {
            KeyCode::Esc => {Some(Message::Exit)},
            KeyCode::Backspace => {
                self.text.pop();
                Some(Message::DecipherCiphertext)
            },
            KeyCode::Char(c) => {
                self.text.push(c);
                Some(Message::DecipherCiphertext)
            },
            KeyCode::Tab => Some(Message::NextFocus),

            _ => None
        }
    }

    pub fn handle_mouse_events(&mut self, m : MouseEvent) {
        match m.kind {
            crossterm::event::MouseEventKind::ScrollDown => {self.scroll+=1},
            crossterm::event::MouseEventKind::ScrollUp if self.scroll !=0=> {self.scroll-=1},
            crossterm::event::MouseEventKind::ScrollLeft => {},
            crossterm::event::MouseEventKind::ScrollRight => {},
            _ => {}
        }
    }
}
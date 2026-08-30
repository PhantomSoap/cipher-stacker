

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, MouseEvent};
use ratatui::{Frame, layout::Rect, text::Text, widgets::Paragraph};

use crate::{Message, cipher_stack::{CipherName, CipherType}};


pub struct CipherAdder {
    cipher : CipherName,
    scroll : u16,
}

impl CipherAdder {
    pub fn new() -> Self {
        Self {
            cipher : CipherName::Caesar,
            scroll : 0,
        }
    }
    pub fn draw(&self,frame : &mut Frame, area : Rect) {
        let panel = Paragraph::new(Text::from(format!("____________\n| {:?} | <+> Add Cipher |\n------------------",&self.cipher)));
        frame.render_widget(panel, area);
    }

    pub fn handle_key_events(&mut self,key : KeyEvent)  -> Option<Message>{
        if let KeyEventKind::Release =  key.kind {
            return None
        }

        match key.code {
            KeyCode::Esc => {Some(Message::Exit)},
            KeyCode::Char('+') => {Some(Message::AddCipher(self.cipher,None))}
            KeyCode::Up => {
                self.cipher.next(); 
                None
            },
            KeyCode::Down  => {
                self.cipher.previous(); 
                None
            }  
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
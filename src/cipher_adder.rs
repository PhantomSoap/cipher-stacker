

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, MouseEvent};

use crate::{Message, cipher_stack::CipherType};


pub struct CipherAdder {
    cipher : CipherType,
    scroll : u16,
}

impl CipherAdder {
    pub fn new() -> Self {
        Self {
            cipher : CipherType::Caeser(0),
            scroll : 0,
        }
    }

    pub fn handle_key_events(&mut self,key : KeyEvent)  -> Option<Message>{
        if let KeyEventKind::Release =  key.kind {
            return None
        }

        match key.code {
            KeyCode::Esc => {Some(Message::Exit)},
            KeyCode::Char('+') => {Some(Message::AddCipher(self.cipher.default(),None))}
            KeyCode::Up => {
                self.cipher = self.cipher.next(); 
                None
            },
            KeyCode::Down  => {
                self.cipher = self.cipher.previous(); 
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
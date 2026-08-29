

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, MouseEvent};

use crate::{Message, cipher_stack::CipherType};


pub struct CipherAdder {
    cipher : Option<CipherType>,
    scroll : u16,
}

impl CipherAdder {
    pub fn new() -> Self {
        Self {
            cipher : None,
            scroll : 0,
        }
    }

    pub fn handle_key_events(&mut self,key : KeyEvent)  -> Option<Message>{
        if let KeyEventKind::Release =  key.kind {
            return None
        }

        match key.code {
            KeyCode::Esc => {Some(Message::Exit)},
            KeyCode::Char('+') if let Some(cipher) = &self.cipher => {Some(Message::AddCipher(cipher.default(),None))}
            KeyCode::Up if let Some(cipher) = &mut self.cipher => {
                *cipher = cipher.next(); 
                None
            },
            KeyCode::Down if let Some(cipher) = &mut self.cipher => {
                *cipher = cipher.previous(); 
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
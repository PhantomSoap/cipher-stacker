use std::{
    fmt::{self, Formatter},
    mem::discriminant,
};

use cifers::{Cipher,Caeser,Vigenere,Railfence,Affine};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, MouseEvent};
use ratatui::{Frame, layout::Rect, style::Color, widgets::{Block, List, ListItem, ListState}};

use crate::Message;

#[derive(Debug, Clone, PartialEq)]
pub enum CipherType {
    Caeser(i8),
    Vigenere(String),
    RailFence(u8),
    Atbash,
    Affine(u8, u8),
}

#[derive(Debug)]
pub struct CipherStack {
    pub ciphers: Vec<CipherType>,
    pub selected: Option<usize>,
    pub scroll : usize,
}

impl CipherStack {
    pub fn default() -> CipherStack {
        Self::new()
    }
    pub fn new() -> CipherStack {
        CipherStack {
            ciphers: Vec::new(),
            selected: None,
            scroll : 0,
        }
    }

    pub fn stack_cipher(&mut self, text: &str, ciphertext: &mut String) -> Vec<String> {
        let mut history: Vec<String> = Vec::new();
        history.push(text.to_string());
        let mut working_cipher = text.to_string();
        if self.ciphers.is_empty() {
            *ciphertext = working_cipher;
            return history;
        };

        for cipher in &self.ciphers {
            match cipher {
                CipherType::Caeser(shift) => {
                    working_cipher = Caeser::new(*shift as i32).encipher(&working_cipher);

                    history.push(working_cipher.to_string());
                }
                CipherType::Vigenere(code) => {
                    if !code.is_empty() {
                        working_cipher = Vigenere::new(code.clone())
                            .encipher(&working_cipher)
                    }
                    history.push(working_cipher.clone());
                }
                CipherType::RailFence(key) => {
                    if !(*key < 2 || *key >= working_cipher.len() as u8) {
                        working_cipher = Railfence::new(*key as u8)
                            .encipher(&working_cipher)
                            
                    };
                    history.push(working_cipher.clone());
                }
                CipherType::Atbash => {
                    working_cipher = Affine::atbash().encipher(&working_cipher);
                    history.push(working_cipher.clone());
                }
                CipherType::Affine(a, b) => {
                    working_cipher = Affine::new(*a as i32, *b as i32)
                        .encipher(&working_cipher);
                    history.push(working_cipher.clone());
                }
            };
        }
        *ciphertext = working_cipher;
        history
    }
}

impl CipherStack {
    pub fn next(&self, cipher: &CipherType) -> (CipherType, Option<usize>) {
        if let Some(index) = self
            .ciphers
            .iter()
            .enumerate()
            .filter(|&(_, val)| discriminant(val) == discriminant(&cipher.next()))
            .nth_back(0)
            .map(|(indx, _)| indx)
        {
            (self.ciphers.get(index).unwrap().clone(), Some(index))
        } else {
            (cipher.next(), None)
        }
    }

    pub fn previous(&self, cipher: &CipherType) -> (CipherType, Option<usize>) {
        if let Some(index) = self
            .ciphers
            .iter()
            .enumerate()
            .filter(|&(_, val)| discriminant(val) == discriminant(&cipher.previous()))
            .nth_back(0)
            .map(|(indx, _)| indx)
        {
            (self.ciphers.get(index).unwrap().clone(), Some(index))
        } else {
            (cipher.previous(), None)
        }
    }
}
impl CipherType {
    pub fn first() -> Self {
        Self::Caeser(0)
    }
    pub fn next(&self) -> CipherType {
        match self {
            CipherType::Caeser(_) => CipherType::Vigenere("".to_string()),
            CipherType::Vigenere(_) => CipherType::RailFence(1),
            CipherType::RailFence(_) => CipherType::Atbash,
            CipherType::Atbash => CipherType::Affine(1, 0),
            CipherType::Affine(_, _) => CipherType::Caeser(0),
        }
    }
    pub fn previous(&self) -> CipherType {
        match self {
            CipherType::Caeser(_) => CipherType::Affine(1, 0),
            CipherType::Vigenere(_) => CipherType::Caeser(0),
            CipherType::RailFence(_) => CipherType::Vigenere("".to_string()),
            CipherType::Atbash => CipherType::RailFence(1),
            CipherType::Affine(_, _) => CipherType::Atbash,
        }
    }

    pub fn default(&self) -> CipherType {
        match self {
            CipherType::Caeser(_) => CipherType::Caeser(0),
            CipherType::Vigenere(_) => CipherType::Vigenere("".to_string()),
            CipherType::RailFence(_) => CipherType::RailFence(1),
            CipherType::Atbash => CipherType::Atbash,
            CipherType::Affine(_, _) => CipherType::Affine(1, 0),
        }
    }

    pub fn instructions(&self) -> String {
        match self {
            CipherType::Caeser(_) => String::from("<- Shift ->"),
            CipherType::Vigenere(_) => String::from("Type a Keyword"),
            CipherType::RailFence(_) => String::from("<Up> Increment Key <Down> Decrement Key"),
            CipherType::Atbash => String::from(""),
            CipherType::Affine(_, _) => {
                String::from("<- Shift -> | <Up> Increment Multiplyer <Down Decrement Multiplyer>")
            }
        }
    }

    pub fn name(&self) -> String {
        match self {
            CipherType::Caeser(_) => String::from("Caesar Cipher"),
            CipherType::Vigenere(_) => String::from("Vigenere Cipher"),
            CipherType::RailFence(_) => String::from("RailFence Cipher"),
            CipherType::Atbash => String::from("Atbash Cipher"),
            CipherType::Affine(_, _) => String::from("Affine Cipher"),
        }
    }
}

impl fmt::Display for CipherType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            CipherType::Caeser(shift) => {
                write!(f, "Caeser Cipher; Shift: {}", shift)
            }
            CipherType::Vigenere(code) if !code.is_empty() => {
                write!(f, "Vigenere Cipher; Keyword: {}", code)
            }
            CipherType::Vigenere(_code) => write!(f, "Vigenere Cipher;"),
            CipherType::RailFence(key) => {
                write!(f, "RailFence Cipher; Key: {}", key)
            }
            CipherType::Atbash => write!(f, "Atbash Cipher"),
            CipherType::Affine(a, b) => write!(f, "Affine Cipher; Multiplyer: {}, Shift: {}", a, b),
        }
    }
}

impl CipherStack {
    pub fn draw(&self,frame : &mut Frame, area : Rect) {
        let mut state = ListState::default();
        state.select(self.selected);
        let list = List::new(
            self.ciphers
            .iter()
            .map(
                |cipher| ListItem::from(format!("{:?}",cipher))
            )
        ).highlight_style(Color::Blue).block(Block::bordered());
        frame.render_stateful_widget(list, area, &mut state);
    }

    pub fn handle_key_events(&mut self,key : KeyEvent)  -> Option<Message>{
        if let KeyEventKind::Release =  key.kind {
            return None
        }

        match key.code {
            KeyCode::Esc => {Some(Message::Exit)},
            KeyCode::Tab => Some(Message::NextFocus),
            KeyCode::Up if let Some(index) = self.selected  => {
                if index !=0 {
                    Some(Message::PreviousInStack)
                } else {
                    None
                }
            },
            KeyCode::Down if let Some(index) = self.selected  => {
                if index !=self.ciphers.len()-1 {
                    Some(Message::NextInStack)
                } else {
                    None
                }
            },

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

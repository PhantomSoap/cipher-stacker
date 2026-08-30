use std::{
    fmt::{self, Formatter}
};

use cifers::{Cipher,Caeser,Vigenere,Railfence,Affine};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, MouseEvent};
use ratatui::{Frame, layout::Rect, style::Color, widgets::{Block, List, ListItem, ListState}};

use crate::Message;
#[derive(Debug,Clone,Copy)]
pub enum CipherEdit {
    PushChar(char),
    Popchar,
    Up,
    Down,
    Left,
    Right,
}
#[derive(Debug,Clone, Copy)]
pub enum CipherName {
    Caesar,
    Vigenere,
    Railfence,
    Atbash,
    Affine,
}

impl CipherName {
    pub fn next(&mut self) {
        *self = match self {
            CipherName::Caesar => CipherName::Vigenere,
            CipherName::Vigenere => CipherName::Railfence,
            CipherName::Railfence => CipherName::Atbash,
            CipherName::Atbash => CipherName::Affine,
            CipherName::Affine => CipherName::Caesar,
        }
    }

    pub fn previous(&mut self) {
        *self = match self {
            CipherName::Caesar => CipherName::Affine,
            CipherName::Vigenere => CipherName::Caesar,
            CipherName::Railfence => CipherName::Vigenere,
            CipherName::Atbash => CipherName::Railfence,
            CipherName::Affine => CipherName::Atbash,
        }
    }

    pub fn into_ciphertype(&self) -> CipherType {
        match self {
            CipherName::Caesar => CipherType::Caeser(0),
            CipherName::Vigenere => CipherType::Vigenere("".to_string()),
            CipherName::Railfence => CipherType::RailFence(1),
            CipherName::Atbash => CipherType::RailFence(1),
            CipherName::Affine => CipherType::Affine(1, 0),
        }
    }
}
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
                     
                    working_cipher = Railfence::new(*key as u8 % working_cipher.len() as u8)
                        .encipher(&working_cipher);
                            
                    
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


impl CipherType {
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

    pub fn into_ciphername(&self) -> CipherName {
        match self {
            CipherType::Caeser(_) => CipherName::Caesar,
            CipherType::Vigenere(_) => CipherName::Vigenere,
            CipherType::RailFence(_) => CipherName::Railfence,
            CipherType::Atbash => CipherName::Atbash,
            CipherType::Affine(_, _) => CipherName::Affine,
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
    pub fn draw(&self,frame : &mut Frame, area : Rect,focus : bool) {
        let mut state = ListState::default();
        state.select(self.selected);
        let list = if focus {
            List::new(
            self.ciphers
            .iter()
            .map(
                |cipher| ListItem::from(format!("{:?}",cipher))
            )
            ).highlight_style(Color::Blue).block(Block::bordered().border_style(Color::Blue))
        } else {
            List::new(
            self.ciphers
            .iter()
            .map(
                |cipher| ListItem::from(format!("{:?}",cipher))
            )
            ).highlight_style(Color::Blue).block(Block::bordered())
        };
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

    pub fn update(&mut self,msg : &Message) -> Option<Message> {
        match msg {
            Message::AddCipher(ciphername, Some(index)) => {
                self.ciphers.insert(*index, ciphername.into_ciphertype());
                self.selected = Some(*index);
                Some(Message::CipherPlaintext)
            },
            Message::AddCipher(ciphername, None) => {
                self.ciphers.push(ciphername.into_ciphertype());
                self.selected = Some(self.ciphers.len() - 1);
                Some(Message::CipherPlaintext)
            },
            
            Message::RemoveCipher(Some(index)) => {
                let _removed = self.ciphers.remove(*index);
                self.selected = if self.ciphers.len() !=0 {Some(self.ciphers.len()-1)} else {None};
                Some(Message::CipherPlaintext)
            },
            Message::RemoveCipher(None) => {
                if let Some(removed) = self.ciphers.pop() {
                    self.selected = if self.ciphers.len() !=0 {Some(self.ciphers.len()-1)} else {None};
                    
                }
                Some(Message::CipherPlaintext)
            },
            Message::EditCipher(edit) => {
                if let Some(index) = self.selected {
                    match edit {
                        CipherEdit::PushChar(chr) if let CipherType::Vigenere(code) = &mut self.ciphers[index]  => {
                            code.push(chr.to_ascii_uppercase());
                        },
                        CipherEdit::Popchar if let CipherType::Vigenere(code) = &mut self.ciphers[index] => {
                            code.pop();
                        },
                        CipherEdit::Up => {
                            match &mut self.ciphers[index] {
                                CipherType::Affine(a,_b ) => {
                                    let mut shift = *a;
                                    while !(*a == 26) && !(shift == 26) {
                                        if !((shift + 1) % 2 == 0) && !((shift + 1) % 13 == 0) {
                                            *a = shift + 1;
                                            break;
                                        } else {
                                            shift += 1;
                                        }
                                    }
                                },
                                CipherType::RailFence(key)  => {
                                    *key +=1
                                },
                                _ => {}
                            }
                        },
                        CipherEdit::Down => {
                            match &mut self.ciphers[index] {
                                CipherType::Affine(a,_b ) => {
                                    let mut shift = *a;
                                    while !(*a == 0) && !(shift == 0) {
                                        if !((shift - 1) % 2 == 0) && !((shift - 1) % 13 == 0) {
                                            *a = shift - 1;
                                            break;
                                        } else {
                                            shift -= 1;
                                        }
                                    }
                                },
                                CipherType::RailFence(key) if *key !=1 => {
                                    *key -=1
                                },
                                _ => {}
                            }
                        },
                        CipherEdit::Left => {
                            match &mut self.ciphers[index] {
                                CipherType::Caeser(shift) => {
                                    *shift = ((*shift - 1) % 26 + 26) % 26;
                                },
                                CipherType::Affine(_a,b ) if !(*b == 0) => {
                                    *b -= 1;
                                } 
                                _ => {}
                            }
                        },
                        CipherEdit::Right => {
                            match &mut self.ciphers[index] {
                                CipherType::Caeser(shift) => {
                                    *shift = ((*shift + 1) % 26 + 26) % 26;
                                },
                                CipherType::Affine(_a,b ) if !(*b == 25) => {
                                    *b += 1;
                                } 

                                
                                _ => {}
                            }
                        },
                        _ => {}
                    }
                }
                Some(Message::CipherPlaintext)
            },
            Message::NextInStack => {
                if let Some(index) = &mut self.selected {
                    *index += 1;
                }
                None
            },
            Message::PreviousInStack if let Some(index) = &mut self.selected=> {
                if *index !=0 {
                    *index -= 1;
                }
                None
            }
            _ => {None}
        }

    }
}

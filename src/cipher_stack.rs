

use cifers::{Cipher,Caeser,Vigenere,Railfence,Affine};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, MouseEvent};
use ratatui::{Frame, layout::{Constraint, Layout, Rect}, style::{Color, Style}, text::{Line, Span, Text}, widgets::{Block, List, ListItem, ListState, Paragraph}};
use crate::{CipherType,CipherName};
use crate::Message::{self, AddCipher, RemoveCipher};
#[derive(Debug,Clone,Copy)]
pub enum CipherEdit {
    PushChar(char),
    Popchar,
    Up,
    Down,
    Left,
    Right,
}





#[derive(Debug)]
pub struct CipherStack {
    pub ciphers: Vec<CipherType>,
    pub selected: Option<usize>,
    pub cipher_to_add : CipherName,
    pub editing : Option<usize>,
    pub scroll : usize,
}

impl CipherStack {
    pub fn new() -> CipherStack {
        CipherStack {
            ciphers: Vec::new(),
            selected: None,
            editing : None,
            cipher_to_add : CipherName::Caesar,
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




impl CipherStack {
    pub fn draw(&self,frame : &mut Frame, area : Rect,focus : bool) {
        let split = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(8),
        ]).split(area);
        let adding_panel = if focus {
            Paragraph::new(
                Text::from(
                    Line::from(
                        vec![
                            Span::raw("| "),
                            Span::styled(format!("{:?}",&self.cipher_to_add), Style::default().fg(Color::Black).bg(Color::White)),
                            Span::raw(" |"),
                            Span::styled("<+>",Color::Blue),
                            Span::raw(" to add cipher |")
                        ]
                    )
                )
            ).block(Block::bordered().border_style(Color::Blue).title_top("Add Cipher"))
        } else {
            Paragraph::new(
                Text::from(
                    Line::from(
                        vec![
                            Span::raw("| "),
                            Span::styled(format!("{:?}",&self.cipher_to_add), Style::default().fg(Color::Black).bg(Color::White)),
                            Span::raw(" |"),
                            Span::styled("<+>",Color::Blue),
                            Span::raw(" to add cipher |")
                        ]
                    )
                )
            ).block(Block::bordered().title_top("Add Cipher"))
        };
        frame.render_widget(adding_panel, split[0]);
        let mut state = ListState::default();
        state.select(self.selected);
        let list = if focus {
            List::new(
            self.ciphers
            .iter()
            .map(
                |cipher| ListItem::from(format!("{:?}",cipher))
            )
            ).highlight_style(Color::Blue).block(Block::bordered().border_style(Color::Blue).title_top("Ciphers").title_bottom(format!("{:?}",self.selected)))
        } else {
            List::new(
            self.ciphers
            .iter()
            .map(
                |cipher| ListItem::from(format!("{:?}",cipher))
            )
            ).highlight_style(Color::Blue).block(Block::bordered().title_top("Ciphers").title_bottom(format!("{:?}",self.selected)))
        };
        frame.render_stateful_widget(list, split[1], &mut state);
    }

    pub fn handle_key_events(&mut self,key : KeyEvent)  -> Option<Message>{
        if let KeyEventKind::Release =  key.kind {
            return None
        }

        match key.code {
            KeyCode::Esc => {Some(Message::Exit)},
            KeyCode::Tab => Some(Message::NextFocus),
            KeyCode::Char('-') => {
                Some(RemoveCipher(self.selected))   
            }
            KeyCode::Char('+') => Some(AddCipher(self.cipher_to_add, self.selected)),
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
            KeyCode::Right => {self.cipher_to_add.next(); None},
            KeyCode::Left => {self.cipher_to_add.previous(); None},

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
                None
            },
            Message::AddCipher(ciphername, None) => {
                self.ciphers.push(ciphername.into_ciphertype());
                self.selected = Some(self.ciphers.len() - 1);
                None
            },
            
            Message::RemoveCipher(Some(index)) => {
                let _removed = self.ciphers.remove(*index);
                self.selected = if self.ciphers.len() !=0 {Some(self.ciphers.len()-1)} else {None};
                None
            },
            Message::RemoveCipher(None) => {
                if let Some(_removed) = self.ciphers.pop() {
                    self.selected = if self.ciphers.len() !=0 {Some(self.ciphers.len()-1)} else {None};
                    
                }
                None
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
                None
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

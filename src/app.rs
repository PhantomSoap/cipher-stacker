use crate::Message;
use crate::{CipherStack, CipherType};
use crate::CipherAdder;
use crate::Plaintext;
use crate::Ciphertext;
use crate::History;
use crate::AppCipher;
use crossterm::event::{self, Event, KeyCode};
use ratatui::layout::{ Constraint, Direction, Layout};
use ratatui::widgets::Block;
use ratatui::{DefaultTerminal, Frame, layout::Rect};
use std::io;


#[derive(Debug)]

pub enum Focus {
    Plaintext,
    Ciphertext,
    CipherStack,
    AddCipher,
    History,

}

impl Focus {
    pub fn next(&self) -> Self {
        match self {
            Focus::Plaintext => Focus::AddCipher,
            Focus::AddCipher => Focus::Ciphertext,
            Focus::Ciphertext => Focus::CipherStack,
            Focus::CipherStack => Focus::History,
            Focus::History => Focus::Plaintext,
        }
    }
}





pub struct App {
    pub plaintext: Plaintext,
    pub ciphertext: Ciphertext,
    pub stack: CipherStack,
    pub exit: bool,
    pub history: History,
    pub cipherview : Option<AppCipher>,
    pub adding_panel : CipherAdder,
    pub focus : Focus,
    
}
pub struct AppLayout {
    plaintext : Rect, 
    ciphertext : Rect, 
    cipherstack : Rect,
    history : Rect,
    cipherview : Rect,
}

impl AppLayout {
    pub fn build(area : Rect) -> Self {
        let layouts = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(1),
                Constraint::Length(30),
                Constraint::Length(15),         
            ]).split(area);

        let middles = Layout::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .constraints([
                    Constraint::Percentage(20),
                    Constraint::Percentage(60),
                    Constraint::Percentage(20),
                ]).split(layouts[1]);

        let bottoms = Layout::default()
                    .direction(Direction::Horizontal)
                    .margin(1)
                    .constraints([
                        Constraint::Percentage(20),
                        Constraint::Percentage(60),
                        Constraint::Percentage(20),
                    ]).split(layouts[2]);

        let footer_lines = Layout::default()
                        .direction(Direction::Vertical)
                        .margin(1)
                        .constraints([
                            Constraint::Length(4),
                            Constraint::Length(4),
                            Constraint::Length(4),
                        ]).split(bottoms[1]);

        Self {
            plaintext: footer_lines[0],
            ciphertext: footer_lines[2],
            cipherstack: bottoms[0],
            history: bottoms[2],
            cipherview : middles[1],
        }
    }
}
impl App {
    pub fn new() -> App {
        App {
            plaintext: Plaintext::new(String::from("ExampleText")),
            ciphertext: Ciphertext::new(String::from("ExampleText")),
            stack: CipherStack::new(),
            exit: false,
            history: History::default(),
            focus : Focus::Plaintext,
            cipherview : None,
            adding_panel : CipherAdder::new(),
            

            
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;

            if let Some(msg) = self.handle_input_events()? {
                self.update(msg);
            }

            self.history.list = self
                .stack
                .stack_cipher(&self.plaintext.text, &mut self.ciphertext.text);
        }

        Ok(())
    }

    pub fn handle_input_events(&mut self)  -> io::Result<Option<Message>>{
        match event::read()? {
            
            Event::Key(key_event) => {
                match self.focus {
                    Focus::Plaintext => {
                        Ok(self.plaintext.handle_key_events(key_event))
                        
                    },
                    Focus::Ciphertext => {
                        Ok(self.ciphertext.handle_key_events(key_event))
                        
                    },
                    Focus::CipherStack => {
                        Ok(self.stack.handle_key_events(key_event))
                        
                    },
                    Focus::AddCipher => {
                        Ok(self.adding_panel.handle_key_events(key_event))
                        
                    },
                    Focus::History => Ok(self.history.handle_key_events(key_event)),
                }
            },
            Event::Mouse(mouse_event) => {
                match self.focus {
                    Focus::Plaintext => {
                        self.plaintext.handle_mouse_events(mouse_event);
                        Ok(None)
                    },
                    Focus::Ciphertext => {
                        self.ciphertext.handle_mouse_events(mouse_event);
                        Ok(None)
                    },
                    Focus::CipherStack => {
                        self.stack.handle_mouse_events(mouse_event);
                        Ok(None)
                    },
                    Focus::AddCipher => {Ok(None)},
                    Focus::History => {
                        self.history.handle_mouse_events(mouse_event);
                        Ok(None)
                    },
        }
            },
            Event::Paste(_) => {Ok(None)}
            _ => {Ok(None)}
        }
        
    }

    pub fn draw(&self, frame: &mut Frame) {
        let areas = AppLayout::build(frame.area()); 
        frame.render_widget(Block::bordered().title(format!("{:?}",self.focus)), frame.area());
        if let Some(cipherview) = &self.cipherview {
            cipherview.draw(frame,areas.cipherview);
        } else {
            frame.render_widget(Block::bordered(),areas.cipherview)
        }
        self.plaintext.draw(frame,areas.plaintext);
        self.ciphertext.draw(frame,areas.ciphertext);
        self.stack.draw(frame,areas.cipherstack);
        self.history.draw(frame,areas.history,&self.stack);
    }

    pub fn edit_cipher(&mut self, index: usize, key: KeyCode) {
        let cipher = self.stack.ciphers.get_mut(index).unwrap();
        match cipher {
            CipherType::Caeser(shift) => match key {
                KeyCode::Right => {
                    *shift = ((*shift + 1) % 26 + 26) % 26;
                }
                KeyCode::Left => {
                    *shift = ((*shift - 1) % 26 + 26) % 26;
                }
                _ => {}
            },
            CipherType::Vigenere(code) => match key {
                KeyCode::Char(chr) if chr.is_alphabetic() => {
                    code.push(chr.to_ascii_uppercase());
                }
                KeyCode::Backspace => {
                    code.pop();
                }
                _ => {}
            },
            CipherType::RailFence(ckey) => match key {
                KeyCode::Down if !(*ckey <= 1) => *ckey -= 1,
                KeyCode::Up if !(*ckey == self.plaintext.text.len() as u8) => {
                    *ckey += 1;
                }
                _ => {}
            },
            CipherType::Atbash => {}
            CipherType::Affine(a, b) => match key {
                KeyCode::Up => {
                    let mut shift = *a;
                    while !(*a == 26) && !(shift == 26) {
                        if !((shift + 1) % 2 == 0) && !((shift + 1) % 13 == 0) {
                            *a = shift + 1;
                            break;
                        } else {
                            shift += 1;
                        }
                    }
                }
                KeyCode::Down => {
                    let mut shift = *a;
                    while !(*a == 0) && !(shift == 0) {
                        if !((shift - 1) % 2 == 0) && !((shift - 1) % 13 == 0) {
                            *a = shift - 1;
                            break;
                        } else {
                            shift -= 1;
                        }
                    }
                }
                KeyCode::Left if !(*b == 0) => {
                    *b -= 1;
                }
                KeyCode::Right if !(*b == 25) => {
                    *b += 1;
                }
                _ => {}
            },
        }
    }

    pub fn update(&mut self, msg: Message) {
        match msg {
            Message::AddCipher(cipher_type, Some(index)) => {
                self.stack.ciphers.insert(index, cipher_type.default());
                self.stack.selected = Some(index)
            }
            Message::AddCipher(cipher_type, None) => {
                self.stack.ciphers.push(cipher_type.default());
                self.stack.selected = Some(self.stack.ciphers.len() - 1)
            }
            Message::RemoveCipher(Some(index)) => {
                let removed = self.stack.ciphers.remove(index);
                self.stack.selected = None
            }
            Message::RemoveCipher(None) => {
                if let Some(removed) = self.stack.ciphers.pop() {
                    self.stack.selected = None
                }
            }
            Message::Exit => self.exit(),
            Message::Reset => self.exit(),
            Message::StopCiphering => {
                
            }
            Message::StartCiphering(index) => {
                self.stack.selected = Some(index)
            }
            Message::PushChar(c) => self.plaintext.text.push(c),
            Message::PopChar => {
                self.plaintext.text.pop();
            }
            Message::GoHome => {},
            Message::EditCipher(index, key_code) => self.edit_cipher(index, key_code),
            Message::NextCipher(cipher) => {
                let (next_cipher, index_opt) = self.stack.next(&cipher);
                self.stack.selected = index_opt
            }
            Message::PreviousCipher(cipher) => {
                let (next_cipher, index_opt) = self.stack.previous(&cipher);
                self.stack.selected = index_opt
            }
            Message::NextInStack => {
                if let Some(index) = &mut self.stack.selected {
                    *index += 1;
                }
            }
            Message::PreviousInStack if let Some(index) = &mut self.stack.selected=> {
                if *index !=0 {
                    *index -= 1;
                }
            }
            Message::LookAtCipher(cipher) => {},
            Message::NextFocus => {self.focus = self.focus.next()},
            _ => {},
        }
    }
    pub fn exit(&mut self) {
        self.exit = true;
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}


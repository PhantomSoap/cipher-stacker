use crate::cipher_stack::{CipherStack, CipherType};

use crossterm::event::KeyCode;
use ratatui::{DefaultTerminal, Frame, buffer::Buffer, layout::Rect, widgets::Widget};
use std::io;

pub enum Message {
    AddCipher(CipherType, Option<usize>),
    RemoveCipher(Option<usize>),
    Exit,
    Reset,
    StopCiphering,
    StartCiphering(usize),
    LookAtCipher(CipherType),
    PushChar(char),
    PopChar,
    GoHome,
    EditCipher(usize, KeyCode),
    NextCipher(CipherType),
    PreviousCipher(CipherType),
    NextInStack,
    PreviousInStack,
    None,
}
#[derive(Debug)]
pub enum AppState {
    CurrentlyEditingCiphers(usize),
    EditingText(Option<CipherType>),
}
pub enum Focus {
    Plaintext,
    Ciphertext,
    CipherStack,
    Cipher,
    History,

}
pub struct Plaintext {
    pub text : String,
    pub scroll : usize,
}
impl Plaintext {
    pub fn new(text : String) -> Self {
        Self {
            text,
            scroll : 0
        }
    }
}
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
}
#[derive(Default)]
pub struct History {
    pub list : Vec<String>,
    pub scroll : usize,
}



pub struct App {
    pub plaintext: Plaintext,
    pub ciphertext: Ciphertext,
    pub stack: CipherStack,
    pub state: AppState,
    pub exit: bool,
    pub history: History,
    pub focus : Focus,
    
}

impl App {
    pub fn new() -> App {
        App {
            plaintext: Plaintext::new(String::from("ExampleText")),
            ciphertext: Ciphertext::new(String::from("ExampleText")),
            stack: CipherStack::new(),
            state: AppState::EditingText(None),
            exit: false,
            history: History::default(),
            focus : Focus::Plaintext,
            
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;

            self.update(self.handle_key_events()?);

            self.history.list = self
                .stack
                .stack_cipher(&self.plaintext.text, &mut self.ciphertext.text);
        }

        Ok(())
    }

    pub fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
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
            Message::None => {}
            Message::AddCipher(cipher_type, Some(index)) => {
                self.stack.ciphers.insert(index, cipher_type.default());
                self.state = AppState::CurrentlyEditingCiphers(index);
                self.stack.selected = Some(index)
            }
            Message::AddCipher(cipher_type, None) => {
                self.stack.ciphers.push(cipher_type.default());
                self.state = AppState::CurrentlyEditingCiphers(self.stack.ciphers.len() - 1);
                self.stack.selected = Some(self.stack.ciphers.len() - 1)
            }
            Message::RemoveCipher(Some(index)) => {
                let removed = self.stack.ciphers.remove(index);
                self.state = AppState::EditingText(Some(removed.default()));
                self.stack.selected = None
            }
            Message::RemoveCipher(None) => {
                if let Some(removed) = self.stack.ciphers.pop() {
                    self.state = AppState::EditingText(Some(removed.default()));
                    self.stack.selected = None
                }
            }
            Message::Exit => self.exit(),
            Message::Reset => self.exit(),
            Message::StopCiphering => {
                if let AppState::CurrentlyEditingCiphers(index) = self.state {
                    let cipher = self.stack.ciphers[index].clone();
                    self.state = AppState::EditingText(Some(cipher));
                    self.stack.selected = Some(index);
                }
            }
            Message::StartCiphering(index) => {
                self.state = AppState::CurrentlyEditingCiphers(index);
                self.stack.selected = Some(index)
            }
            Message::PushChar(c) => self.plaintext.text.push(c),
            Message::PopChar => {
                self.plaintext.text.pop();
            }
            Message::GoHome => self.state = AppState::EditingText(None),
            Message::EditCipher(index, key_code) => self.edit_cipher(index, key_code),
            Message::NextCipher(cipher) => {
                let (next_cipher, index_opt) = self.stack.next(&cipher);
                self.state = AppState::EditingText(Some(next_cipher));
                self.stack.selected = index_opt
            }
            Message::PreviousCipher(cipher) => {
                let (next_cipher, index_opt) = self.stack.previous(&cipher);
                self.state = AppState::EditingText(Some(next_cipher));
                self.stack.selected = index_opt
            }
            Message::NextInStack => {
                if let Some(index) = &mut self.stack.selected {
                    *index += 1;
                    self.state = AppState::EditingText(Some(self.stack.ciphers[*index].clone()));
                }
            }
            Message::PreviousInStack => {
                if let Some(index) = &mut self.stack.selected {
                    *index -= 1;
                    self.state = AppState::EditingText(Some(self.stack.ciphers[*index].clone()));
                }
            }
            Message::LookAtCipher(cipher) => self.state = AppState::EditingText(Some(cipher)),
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
impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        crate::ui::ui::render(self, area, buf);
        //crate::ui::ui_area::UiArea::new(area).render_borders(buf);
    }
}

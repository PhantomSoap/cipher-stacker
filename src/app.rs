use crate::ciphermod::{CipherText, CipherType};

use crossterm::event::KeyCode;
use ratatui::{DefaultTerminal, Frame, buffer::Buffer, layout::Rect, widgets::Widget};
use std::io;

pub enum Message {
    AddCipher(CipherType,Option<usize>),
    RemoveCipher(Option<usize>),
    Exit,
    Reset,
    StopCiphering,
    StartCiphering(usize),
    PushChar(char),
    PopChar,
    LookAtCipher(CipherType),
    GoHome,
    EditCipher(usize,KeyCode),
    NextCipher,
    NextInStack,
    PreviousCipher,
    PreviousInStack,
    None
}
#[derive(Debug)]
pub enum AppState {
    CurrentlyEditingCiphers(usize),
    EditingText(Option<CipherType>),
}

pub struct App {
    pub text: CipherText,
    pub state: AppState,
    pub exit: bool,
    pub history: Vec<String>,
}

impl App {
    pub fn new() -> App {
        App {
            text: CipherText::new(),
            state: AppState::EditingText(None),
            exit: false,
            history: Vec::new(),
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;

            self.handle_key_events()?;

            self.history = self.text.cipher();
        }

        Ok(())
    }

    pub fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    pub fn edit_cipher(&mut self,index : usize, key : KeyCode) {
        let cipher = self.text.ciphers.get_mut(index).unwrap();
        match cipher {
            CipherType::Caeser(shift) => {
                match key {
                    KeyCode::Right => {
                        *shift = ((*shift + 1) % 26 + 26) % 26;
                    }
                    KeyCode::Left => {
                        *shift = ((*shift - 1) % 26 + 26) % 26;
                    }
                    _ => {}
                }
            },
            CipherType::Vigenere(code) => {
                match key {
                    KeyCode::Char(chr) if chr.is_alphabetic() => {
                        code.push(chr.to_ascii_uppercase());
                    }
                    KeyCode::Backspace => {
                        code.pop();
                    }
                    _ => {}
                }
            },
            CipherType::RailFence(ckey) => {
                match key {
                    KeyCode::Down if !(*ckey <= 1) => *ckey -= 1,
                    KeyCode::Up if !(*ckey == self.text.ciphered.len() as u8) => {
                        *ckey += 1;
                    }
                    _ => {}
                }
            },
            CipherType::Atbash => {

            },
            CipherType::Affine(a, b) => {
                match key {
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
                }
            },
        }
    }

    pub fn update(&mut self, msg : Message) {
        match msg {
            Message::AddCipher(cipher_type, _) => todo!(),
            Message::RemoveCipher(Some(index)) => {self.text.ciphers.remove(index);},
            Message::RemoveCipher(None) => {self.text.ciphers.pop();},
            Message::Exit => self.exit(),
            Message::Reset => self.exit(),
            Message::StopCiphering => {
                if let AppState::CurrentlyEditingCiphers(index) = self.state {
                    let cipher = self.text.ciphers[index].clone();
                    self.state = AppState::EditingText(Some(cipher));
                     
                }
            },
            Message::StartCiphering(index) => {
                self.state = AppState::CurrentlyEditingCiphers(index);
            },
            Message::PushChar(c) => self.text.text.push(c),
            Message::PopChar => {self.text.text.pop();},
            Message::LookAtCipher(cipher_type) => todo!(),
            Message::GoHome => self.state = AppState::EditingText(None),
            Message::EditCipher(index, key_code) => self.edit_cipher(index, key_code),
            Message::None => {},
            Message::NextCipher => todo!(),
            Message::NextInStack => todo!(),
            Message::PreviousCipher => todo!(),
            Message::PreviousInStack => todo!(),
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
        crate::ui::render(self, area, buf);
    }
}

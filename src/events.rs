use crate::app::{App, AppState};
use crate::ciphermod::CipherType;
use crossterm::event::{self, Event, KeyCode};
use std::io;
enum Message {
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
}
impl App {
    pub fn handle_key_events(&mut self) -> io::Result<()> {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                return Ok(());
            }
            if  KeyCode::Esc == key.code {
                self.exit();
            }
            match &mut self.state {
                AppState::EditingText(None) => match key.code {
                    KeyCode::Tab => {
                        self.state = AppState::EditingText(Some(CipherType::Caeser(0)));
                    }
                    KeyCode::Char('-') => {
                        self.text.ciphers.pop();
                    }
                    KeyCode::Char(chr) if chr.is_alphabetic() => {
                        self.text.text.push(chr);
                    }
                    KeyCode::Backspace => {
                        self.text.text.pop();
                    }
                    _ => {}
                },
                AppState::EditingText(Some(cipher)) => match key.code {
                    KeyCode::Char('-') => {
                        self.text.ciphers.pop();
                        self.text.selected = None;
                    }
                    KeyCode::Char('+') => {
                        self.text.ciphers.push(cipher.default());
                        self.text.selected = Some(self.text.ciphers.len() - 1);
                        self.state = AppState::CurrentlyEditingCiphers(self.text.ciphers.len() - 1);
                    }
                    KeyCode::Char(chr) if chr.is_alphabetic() => {
                        self.text.text.push(chr);
                    }
                    KeyCode::Backspace => {
                        self.text.text.pop();
                    }
                    KeyCode::Tab => {
                        let (cipher, selected_opt) = self.text.next(cipher);
                        self.state = AppState::EditingText(Some(cipher));
                        self.text.selected = selected_opt;
                    }
                    KeyCode::BackTab => {
                        let (cipher, selected_opt) = self.text.previous(cipher);
                        self.state = AppState::EditingText(Some(cipher));
                        self.text.selected = selected_opt;
                    }
                    KeyCode::Enter => {
                        if let Some(index) = self.text.selected {
                            self.state = AppState::CurrentlyEditingCiphers(index);
                        }
                    }
                    KeyCode::Left if let Some(index) = self.text.selected => {
                        if !(index == 0) && self.text.ciphers.get(index - 1).is_some() {
                            self.text.selected = Some(index - 1);
                            self.state = AppState::EditingText(Some(
                                self.text.ciphers.get(index - 1).unwrap().clone(),
                            ));
                        }
                    }
                    
                    KeyCode::Right
                        if let Some(index) = self.text.selected
                            && self.text.ciphers.get(index + 1).is_some() =>
                    {
                        self.text.selected = Some(index + 1);
                        self.state = AppState::EditingText(Some(
                            self.text.ciphers.get(index + 1).unwrap().clone(),
                        ));
                    }

                    _ => {}
                },
                AppState::CurrentlyEditingCiphers(index) => {
                    match key.code {
                        KeyCode::Tab => {
                            let cipher = self
                                .text
                                .ciphers
                                .get(*index)
                                .expect("Index in state is invalid");
                            self.text.selected = Some(*index);
                            self.state = AppState::EditingText(Some(self.text.next(cipher).0));
                            return Ok(());
                        }
                        KeyCode::BackTab => {
                            let cipher = self
                                .text
                                .ciphers
                                .get(*index)
                                .expect("Index in state is invalid");
                            self.text.selected = Some(*index);
                            self.state = AppState::EditingText(Some(self.text.previous(cipher).0));
                            return Ok(());
                        }
                        KeyCode::Char('-') => {
                            let removed = self.text.ciphers.remove(*index);
                            self.state = AppState::EditingText(Some(removed.default()));
                            return Ok(());
                        }
                        KeyCode::Enter => {
                            let cipher = self
                                .text
                                .ciphers
                                .get(*index)
                                .expect("Index in state is invalid");

                            self.state = AppState::EditingText(Some(cipher.clone()));
                            return Ok(());
                        }

                        _ => {}
                    }
                    match self
                        .text
                        .ciphers
                        .get_mut(*index)
                        .expect("Index in State was invalid {index}")
                    {
                        CipherType::Caeser(shift) => match key.code {
                            KeyCode::Right => {
                                *shift = ((*shift + 1) % 26 + 26) % 26;
                            }
                            KeyCode::Left => {
                                *shift = ((*shift - 1) % 26 + 26) % 26;
                            }
                            _ => {}
                        },
                        CipherType::Vigenere(code) => match key.code {
                            KeyCode::Char(chr) if chr.is_alphabetic() => {
                                code.push(chr.to_ascii_uppercase());
                            }
                            KeyCode::Backspace => {
                                code.pop();
                            }
                            _ => {}
                        },
                        CipherType::RailFence(ckey) => match key.code {
                            KeyCode::Down if !(*ckey <= 1) => *ckey -= 1,
                            KeyCode::Up if !(*ckey == self.text.ciphered.len() as u8) => {
                                *ckey += 1;
                            }
                            _ => {}
                        },
                        CipherType::Atbash => {}
                        CipherType::Affine(a, b) => match key.code {
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
            }
        }
        Ok(())
    }
}

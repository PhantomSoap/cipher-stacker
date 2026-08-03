use crate::app::{App, AppState};
use crate::ciphermod::CipherType;
use crossterm::event::{self, Event, KeyCode};
use std::io;
use std::mem::discriminant;

impl App {
    pub fn handle_key_events(&mut self) -> io::Result<()> {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                return Ok(());
            }
            match key.code {
                KeyCode::Esc => match &mut self.state {
                    AppState::EditingText(None) => self.exit(),
                    AppState::EditingText(Some(_cipher)) => {
                        self.state = AppState::EditingText(None)
                    }
                    AppState::CurrentlyEditingCiphers(indx) => {
                        self.state = AppState::EditingText(Some(
                            self.text.ciphers.get_mut(*indx).unwrap().clone(),
                        ));
                    }
                },
                KeyCode::Enter => match &mut self.state {
                    AppState::EditingText(None) => {
                        self.state = AppState::EditingText(Some(CipherType::Caeser(0)))
                    }
                    AppState::EditingText(Some(cipher)) => {
                        if let Some(index) = self
                            .text
                            .ciphers
                            .iter()
                            .rposition(|c| discriminant(c) == discriminant(cipher))
                        {
                            self.state = AppState::CurrentlyEditingCiphers(index)
                        } else {
                            self.text.ciphers.push(cipher.clone());
                            self.state =
                                AppState::CurrentlyEditingCiphers(self.text.ciphers.len() - 1);
                        }
                    }
                    AppState::CurrentlyEditingCiphers(_indx) => {}
                },
                KeyCode::Tab => match &mut self.state {
                    AppState::EditingText(None) => {
                        self.state = AppState::EditingText(Some(CipherType::Caeser(0)))
                    }
                    AppState::EditingText(Some(cipher)) => {
                        self.state = AppState::EditingText(Some(cipher.next()))
                    }
                    AppState::CurrentlyEditingCiphers(indx) => {
                        let cipher = self.text.ciphers.get(*indx).unwrap();
                        self.state = AppState::EditingText(Some(self.text.next(cipher)))
                    }
                },
                KeyCode::Char('-') => match &mut self.state {
                    AppState::EditingText(None) => {}
                    AppState::EditingText(Some(_cipher)) => {}
                    AppState::CurrentlyEditingCiphers(indx) => {
                        let cipher = self.text.ciphers.get(*indx).unwrap().clone();
                        self.text.ciphers.remove(*indx);
                        self.state = AppState::EditingText(Some(cipher));
                    }
                },
                KeyCode::Char(chr) if chr.is_alphabetic() => match &mut self.state {
                    AppState::EditingText(_) => {
                        self.text.text.push(chr.to_uppercase().next().unwrap());
                    }

                    AppState::CurrentlyEditingCiphers(indx) => {
                        let cipher = self.text.ciphers.get_mut(*indx).unwrap();
                        match cipher {
                            CipherType::Vigenere(code) => code.push(chr),
                            _ => {}
                        }
                    }
                },
                KeyCode::Backspace => match &mut self.state {
                    AppState::EditingText(None) => {
                        self.text.text.pop();
                    }
                    AppState::EditingText(Some(_cipher)) => {
                        self.text.text.pop();
                    }
                    AppState::CurrentlyEditingCiphers(indx) => {
                        let cipher = self.text.ciphers.get_mut(*indx).unwrap();
                        match cipher {
                            CipherType::Vigenere(code) => {
                                code.pop();
                            }
                            _ => {}
                        }
                    }
                },
                KeyCode::Right => {
                    if let AppState::CurrentlyEditingCiphers(indx) = self.state {
                        let cipher = self.text.ciphers.get_mut(indx).unwrap();
                        match cipher {
                            CipherType::Caeser(shift) => {
                                *shift = ((*shift + 1) % 26 + 26) % 26;
                            }
                            CipherType::Affine(a, _) => {
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

                            _ => {}
                        }
                    }
                }
                KeyCode::Left => {
                    if let AppState::CurrentlyEditingCiphers(indx) = self.state {
                        let cipher = self.text.ciphers.get_mut(indx).unwrap();
                        match cipher {
                            CipherType::Caeser(shift) => {
                                *shift = ((*shift - 1) % 26 + 26) % 26;
                            }
                            CipherType::Affine(a, _) => {
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

                            _ => {}
                        }
                    }
                }
                KeyCode::Up => {
                    if let AppState::CurrentlyEditingCiphers(indx) = self.state {
                        let cipher = self.text.ciphers.get_mut(indx).unwrap();
                        match cipher {
                            CipherType::RailFence(key) => {
                                if !(*key == self.text.ciphered.len() as i32) {
                                    *key += 1
                                }
                            }
                            CipherType::Affine(_, b) if !(*b == 25) => *b += 1,

                            _ => {}
                        }
                    }
                }
                KeyCode::Down => {
                    if let AppState::CurrentlyEditingCiphers(indx) = self.state {
                        let cipher = self.text.ciphers.get_mut(indx).unwrap();
                        match cipher {
                            CipherType::RailFence(key) => {
                                if !(*key <= 2) {
                                    *key -= 1
                                }
                            }
                            CipherType::Affine(_, b) if !(*b == 0) => *b -= 1,

                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }
}

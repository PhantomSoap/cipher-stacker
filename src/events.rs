
use crate::app::{App, AppState,Message};
use crate::ciphermod::CipherType;
use crossterm::event::{self, Event, KeyCode};
use std::io;

impl App {
    pub fn handle_key_events(&self) -> io::Result<Message> {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                return Ok(Message::None)
            }
            if  KeyCode::Esc == key.code {
                return Ok(Message::Exit)
            }
            match &self.state {
                AppState::EditingText(None) => match key.code {
                    KeyCode::Tab => return Ok(Message::LookAtCipher(CipherType::Caeser(0))),
                    KeyCode::Char('-') => return Ok(Message::RemoveCipher(None)),
                    KeyCode::Char(chr) if chr.is_alphabetic() => return Ok(Message::PushChar(chr)),
                    KeyCode::Backspace => return Ok(Message::PopChar),
                    _ => return Ok(Message::None)
                },
                AppState::EditingText(Some(cipher)) => match key.code {
                    KeyCode::Char('-') => return Ok(Message::RemoveCipher(None)),
                    KeyCode::Char('+') => return Ok(Message::AddCipher(cipher.default(), None)),
                    KeyCode::Char(chr) if chr.is_alphabetic() => return Ok(Message::PushChar(chr)),
                    KeyCode::Backspace => return Ok(Message::PopChar),
                    KeyCode::Tab => return Ok(Message::NextCipher(cipher.clone())),
                    KeyCode::BackTab => return Ok(Message::PreviousCipher(cipher.clone())),
                    KeyCode::Enter if let Some(index) = self.text.selected =>  return Ok(Message::StartCiphering(index)),
                    KeyCode::Left if let Some(index) = self.text.selected 
                    && self.text.ciphers.get(index - 1).is_some() => return Ok(Message::PreviousInStack),
                    
                    KeyCode::Right 
                    if let Some(index) = self.text.selected
                            && self.text.ciphers.get(index + 1).is_some() => return Ok(Message::NextInStack),
                        

                    _ => return Ok(Message::None),
                },
                AppState::CurrentlyEditingCiphers(index) => {
                    let cipher = self
                        .text
                        .ciphers
                        .get(*index)
                        .expect("Index in State was invalid {index}");

                    match key.code {
                        KeyCode::Tab => return Ok(Message::NextCipher(cipher.clone())),
                        KeyCode::BackTab => return Ok(Message::PreviousCipher(cipher.clone())),
                        KeyCode::Char('-') => return Ok(Message::RemoveCipher(None)),
                        KeyCode::Enter => return Ok(Message::LookAtCipher(cipher.clone())),

                        _ => return Ok(Message::EditCipher(*index, key.code)),
                    }
                    
                    
                }
            }
        } {
            Ok(Message::None)
        }
    }
}

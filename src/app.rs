use crate::Message;
use crate::cipher_stack::{CipherStack, CipherType};
use crate::plaintext::Plaintext;
use crate::ciphertext::Ciphertext;
use crate::history::History;
use crate::ui::ciphers_ui::cipherview::CipherView;
use crossterm::event::KeyCode;
use ratatui::{DefaultTerminal, Frame, buffer::Buffer, layout::Rect, widgets::Widget};
use std::io;


#[derive(Debug)]
pub enum AppState {
    EditingCipher(usize),
    EditingText,
    Home,
}
pub enum Focus {
    Plaintext,
    Ciphertext,
    CipherStack,
    Cipher,
    History,

}

impl Focus {
    pub fn next(&self) -> Self {
        match self {
            Focus::Plaintext => todo!(),
            Focus::Ciphertext => todo!(),
            Focus::CipherStack => todo!(),
            Focus::Cipher => todo!(),
            Focus::History => todo!(),
        }
    }
}





pub struct App {
    pub plaintext: Plaintext,
    pub ciphertext: Ciphertext,
    pub stack: CipherStack,
    pub state: AppState,
    pub exit: bool,
    pub history: History,
    pub cipherview : Option<Box<dyn CipherView>>,
    pub focus : Focus,
    
}
pub struct AppLayout {
    plaintext : Rect, //3
    ciphertext : Rect, //3
    cipherstack : Rect,
    history : Rect,
}
impl App {
    pub fn new() -> App {
        App {
            plaintext: Plaintext::new(String::from("ExampleText")),
            ciphertext: Ciphertext::new(String::from("ExampleText")),
            stack: CipherStack::new(),
            state: AppState::Home,
            exit: false,
            history: History::default(),
            focus : Focus::Plaintext,
            cipherview : None,
            
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
        //self.cipherview.draw(frame,)
        //self.plaintext.draw(frame,)
        //self.ciphertext.draw(frame,)
        //self.stack.draw(frame,),
        //self.history.draw(frame,),

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
                self.state = AppState::EditingCipher(index);
                self.stack.selected = Some(index)
            }
            Message::AddCipher(cipher_type, None) => {
                self.stack.ciphers.push(cipher_type.default());
                self.state = AppState::EditingCipher(self.stack.ciphers.len() - 1);
                self.stack.selected = Some(self.stack.ciphers.len() - 1)
            }
            Message::RemoveCipher(Some(index)) => {
                let removed = self.stack.ciphers.remove(index);
                self.state = AppState::EditingText;
                self.stack.selected = if !(self.stack.ciphers.len() == 0) {Some(self.stack.ciphers.len()-1)} else {None};
            }
            Message::RemoveCipher(None) => {
                if let Some(removed) = self.stack.ciphers.pop() {
                    self.state = AppState::EditingText;
                    self.stack.selected = if !(self.stack.ciphers.len() == 0) {Some(self.stack.ciphers.len()-1)} else {None};
                }
            }
            Message::Exit => self.exit(),
            Message::Reset => self.exit(),
            Message::StopCiphering => {
                if let AppState::EditingCipher(index) = self.state {
                    let cipher = self.stack.ciphers[index].clone();
                    self.state = AppState::EditingText;
                    self.stack.selected = Some(index);
                }
            }
            Message::StartCiphering(index) => {
                self.state = AppState::EditingCipher(index);
                self.stack.selected = Some(index)
            }
            Message::PushChar(c) => self.plaintext.text.push(c),
            Message::PopChar => {
                self.plaintext.text.pop();
            }
            Message::GoHome => self.state = AppState::Home,
            Message::EditCipher(index, key_code) => self.edit_cipher(index, key_code),
            Message::NextCipher(cipher) => {
                let (next_cipher, index_opt) = self.stack.next(&cipher);
                self.state = AppState::EditingText;
                self.stack.selected = index_opt
            }
            Message::PreviousCipher(cipher) => {
                let (next_cipher, index_opt) = self.stack.previous(&cipher);
                self.state = AppState::EditingText;
                self.stack.selected = index_opt
            }
            Message::NextInStack => {
                if let Some(index) = &mut self.stack.selected {
                    *index += 1;
                    self.state = AppState::EditingText;
                }
            }
            Message::PreviousInStack => {
                if let Some(index) = &mut self.stack.selected {
                    *index -= 1;
                    self.state = AppState::EditingText;
                }
            }
            Message::LookAtCipher(cipher) => self.state = AppState::EditingText,
            Message::NextFocus => self.focus = self.focus.next(),
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

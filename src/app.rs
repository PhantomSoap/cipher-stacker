use crate::{AppCipher, CipherStack, Ciphertext, Message, Plaintext, layouts::AppLayout};

use crate::components::Component;
use crossterm::event::{self, Event};
use ratatui::text::Line;
use ratatui::{DefaultTerminal, Frame, widgets::Block};

use std::io;

#[derive(Debug)]

pub enum Focus {
    Plaintext,
    Ciphertext,
    CipherStack,
    View
}

impl Focus {
    pub fn next(&self) -> Self {
        match self {
            Focus::Plaintext => Focus::CipherStack,
            Focus::CipherStack => Focus::Ciphertext,
            Focus::Ciphertext => Focus::View,
            Focus::View => Focus::Plaintext,
            
        }
    }
}

pub struct App {
    pub plaintext: Plaintext,
    pub ciphertext: Ciphertext,
    pub stack: CipherStack,
    pub exit: bool,
    pub cipherview: Option<AppCipher>,
    pub focus: Focus,
}

impl App {
    pub fn new() -> App {
        App {
            plaintext: Plaintext::new(String::from("ExampleText")),
            ciphertext: Ciphertext::new(String::from("ExampleText")),
            stack: CipherStack::new(),
            exit: false,
            focus: Focus::Plaintext,
            cipherview: None,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;

            if let Some(msg) = self.handle_input_events()? {
                self.update(msg);
            }
            self.stack
                .stack_cipher(&self.plaintext.text, &mut self.ciphertext.text);
        }

        Ok(())
    }

    pub fn handle_input_events(&mut self) -> io::Result<Option<Message>> {
        match event::read()? {
            Event::Key(key_event) => match self.focus {
                Focus::Plaintext => Ok(self.plaintext.handle_key_events(key_event)),
                Focus::Ciphertext => Ok(self.ciphertext.handle_key_events(key_event)),
                Focus::CipherStack => Ok(self.stack.handle_key_events(key_event)),
                Focus::View if let Some(view) = &mut self.cipherview => Ok(view.handle_key_events(key_event)),
                Focus::View => Ok(Some(Message::NextFocus)),
            },
            _ => Ok(None),
        }
    }

    pub fn draw(&mut self, frame: &mut Frame) {
        let areas = AppLayout::build(frame.area());
        frame.render_widget(
            Block::bordered().title_bottom(Line::from(format!("{:?}", self.focus)).centered()),
            frame.area(),
        );
        self.update_cipherview();
        if let Some(cipherview) = &self.cipherview {
            cipherview.draw(frame, areas.cipherview,if let Focus::View = self.focus {
                true
            } else {
                false
            },);
        } else {
            frame.render_widget(Block::bordered(), areas.cipherview);
        }

        self.plaintext.draw(
            frame,
            areas.plaintext,
            if let Focus::Plaintext = self.focus {
                true
            } else {
                false
            },
        );
        self.ciphertext.draw(
            frame,
            areas.ciphertext,
            if let Focus::Ciphertext = self.focus {
                true
            } else {
                false
            },
        );
        self.stack.draw(
            frame,
            areas.cipherstack,
            if let Focus::CipherStack = self.focus {
                true
            } else {
                false
            },
        );
    }

    pub fn update_cipherview(&mut self) {
        if let Some(cipherview) = &mut self.cipherview {
            if let Some(index) = self.stack.selected {
                cipherview.assign(index, &self.stack.ciphers[index], &self.plaintext.text)
            } else {
                self.cipherview = None;
            }
        } else {
            if let Some(index) = self.stack.selected {
                self.cipherview = Some(AppCipher::new(
                    index,
                    &self.stack.ciphers[index],
                    &self.plaintext.text,
                ));
            }
        }
    }

    pub fn update(&mut self, msg: Message) -> Option<Message> {
        match msg {
            
            Message::EditCipher(_) => self.stack.update(msg),
            Message::CipherPlaintext => None,
            Message::Exit => {
                self.exit();
                None
            }
            Message::Reset => {
                self.exit();
                None
            }
            Message::GoHome => {
                self.exit();
                None
            }
            Message::NextFocus => {
                if let Focus::View = self.focus.next() {
                    if let Some(_view) = &self.cipherview {
                        self.focus = self.focus.next();
                    } else {
                        self.focus = self.focus.next().next();
                    }
                } else {
                    self.focus = self.focus.next();
                }
                None
            }
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

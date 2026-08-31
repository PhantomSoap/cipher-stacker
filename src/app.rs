
use crate::{EditingPanel, Message};

use crate::CipherStack;
use crate::CipherAdder;
use crate::Plaintext;
use crate::Ciphertext;
use crate::History;
use crate::AppCipher;
use crossterm::event::{self, Event};
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
    EditCipher,
    History,

}

impl Focus {
    pub fn next(&self) -> Self {
        match self {
            Focus::Plaintext => Focus::AddCipher,
            Focus::AddCipher => Focus::CipherStack,
            Focus::Ciphertext => Focus::History,
            Focus::CipherStack => Focus::EditCipher,
            Focus::EditCipher => Focus::Ciphertext,
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
    pub editing_panel : EditingPanel,
    pub focus : Focus,
    
}
pub struct AppLayout {
    plaintext : Rect, 
    ciphertext : Rect, 
    cipherstack : Rect,
    history : Rect,
    cipherview : Rect,
    adding_panel : Rect,
    _editing_panel : Rect,
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
                            Constraint::Length(4),
                        ]).split(bottoms[1]);

        Self {
            plaintext: footer_lines[0],
            ciphertext: footer_lines[3],
            cipherstack: bottoms[0],
            history: bottoms[2],
            cipherview : middles[1],
            adding_panel : footer_lines[1],
            _editing_panel : footer_lines[2]
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
            editing_panel : EditingPanel::new(),
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
                    Focus::EditCipher => Ok(self.editing_panel.handle_key_events(key_event))
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
                    Focus::EditCipher => Ok(None),
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

    pub fn draw(&mut self, frame: &mut Frame) {
        let areas = AppLayout::build(frame.area()); 
        frame.render_widget(Block::bordered().title(format!("{:?}",self.focus)), frame.area());
        self.update_cipherview();
        if let Some(cipherview) = &self.cipherview {
            cipherview.draw(frame,areas.cipherview);
        } else {
            frame.render_widget(Block::bordered(),areas.cipherview);
        }
        
        self.plaintext.draw(frame,areas.plaintext,if let Focus::Plaintext = self.focus {true} else {false});
        self.ciphertext.draw(frame,areas.ciphertext,if let Focus::Ciphertext = self.focus {true} else {false});
        self.stack.draw(frame,areas.cipherstack,if let Focus::CipherStack = self.focus {true} else {false});
        self.history.draw(frame,areas.history,&self.stack,if let Focus::History = self.focus {true} else {false});
        self.adding_panel.draw(frame,areas.adding_panel,if let Focus::History = self.focus {true} else {false})
    }


    pub fn update_cipherview(&mut self) {
        if let Some(cipherview) = &mut self.cipherview {
            if let Some(index) = self.stack.selected {
                
                cipherview.assign(index,&self.stack.ciphers[index],&self.plaintext.text)
                
                    
                
            } else {
                self.cipherview = None;
            }
        } else {
            if let Some(index) = self.stack.selected {
                self.cipherview = Some(AppCipher::new(index,&self.stack.ciphers[index],&self.plaintext.text));
            }
        }
    }

    

    pub fn update(&mut self, msg: Message) -> Option<Message> {
        match msg {
            Message::AddCipher(_, _) => self.stack.update(&msg),
            Message::RemoveCipher(_) => self.stack.update(&msg),
            Message::EditCipher(_) => self.stack.update(&msg),
            Message::NextInStack => self.stack.update(&msg),
            Message::PreviousInStack => self.stack.update(&msg),
            Message::CipherPlaintext => None,
            Message::DecipherCiphertext => None,
            Message::Exit => {self.exit(); None},
            Message::Reset => {self.exit(); None},
            Message::GoHome => {self.exit(); None},
            Message::NextFocus => {self.focus = self.focus.next(); None},
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


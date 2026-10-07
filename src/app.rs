use crate::Process;
use crate::components::theme_change::ThemeChanger;
use crate::{AppCipher, CipherStack, OutputText, InputText, Message, layouts::AppLayout};

use crate::components::Component;
use ratatui::layout::{Position, Rect};
use ratatui::widgets::Block;
use ratatui::{Frame};
use ratzilla::event::{KeyEvent, MouseButton, MouseEvent, MouseEventKind};

#[derive(Debug)]

pub enum Focus {
    InputText,
    OutputText,
    CipherStack,
    Theme,
    View,
}

impl Focus {
    pub fn next(&self) -> Self {
        match self {
            Focus::InputText => Focus::CipherStack,
            Focus::CipherStack => Focus::OutputText,
            Focus::OutputText => Focus::Theme,
            Focus::Theme => Focus::View,
            Focus::View => Focus::InputText,
        }
    }
}

pub struct App {
    pub input_text: InputText,
    pub output_text: OutputText,
    pub stack: CipherStack,
    pub exit: bool,
    pub cipherview: Option<AppCipher>,
    pub focus: Focus,
    pub layouts: AppLayout,
    pub theme: ThemeChanger,
    pub process : Process
}

impl App {
    pub fn new() -> App {
        App {
            process : Process::Encrypt,
            input_text: InputText::new(String::from("ExampleText"),Process::Encrypt),
            output_text: OutputText::new(String::from("ExampleText"),Process::Decrypt),
            stack: CipherStack::new(crate::Process::Encrypt),
            exit: false,
            focus: Focus::InputText,
            cipherview: None,
            layouts: AppLayout::build(Rect::new(0, 0, 0, 0)),
            theme: ThemeChanger::new(),
        }
    }

    

    pub fn handle_paste(&mut self, text : &str) -> Option<Message> {
        if let Focus::InputText = self.focus {
            self.input_text.text.push_str(text);
            Some(Message::CipherText)
        } else {
            None
        }

    }

    pub fn handle_keys(&mut self,key_event : KeyEvent) -> Option<Message> {
        match self.focus {
                Focus::InputText => self.input_text.handle_key_events(key_event),
                Focus::OutputText => self.output_text.handle_key_events(key_event),
                Focus::CipherStack => self.stack.handle_key_events(key_event),
                Focus::View if let Some(view) = &mut self.cipherview => {
                    view.handle_key_events(key_event)
                }
                Focus::View => Some(Message::NextFocus),
                Focus::Theme => self.theme.handle_key_events(key_event)
            }
    }

    pub fn handle_mouse(&mut self,m : MouseEvent) -> Option<Message> {
        let (col, row) = match m.kind {
                    MouseEventKind::ButtonDown(MouseButton::Left) => (m.col, m.row),
                    _ => return None,
                };
                if self.layouts.plaintext.contains(Position {x : col,y : row}) {
                    Some(Message::Focus(Focus::InputText))
                } else if self.layouts.cipherstack.contains(Position {x : col,y : row}) {
                    Some(Message::Focus(Focus::CipherStack))
                } else if self.layouts.cipherview.contains(Position {x : col,y : row}) {
                    Some(Message::Focus(Focus::View))
                } else if self.layouts.theme_editor.contains(Position {x : col,y : row}) {
                    Some(Message::Focus(Focus::Theme))
                } else if self.layouts.output_text.contains(Position {x : col,y : row}){
                    Some(Message::Focus(Focus::OutputText))
                } else {
                    None
                }
    }

    pub fn draw(&mut self, frame: &mut Frame) {
        self.layouts = AppLayout::build(frame.area());

        self.update_cipherview();
        if let Some(cipherview) = &self.cipherview {
            cipherview.draw(
                frame,
                self.layouts.cipherview,
                if let Focus::View = self.focus {
                    true
                } else {
                    false
                },
                self.theme.get_theme(),
            );
        } else {
            frame.render_widget(Block::bordered().title("").border_style(self.theme.get_theme().border), self.layouts.cipherview);
        }

        self.input_text.draw(
            frame,
            self.layouts.plaintext,
            if let Focus::InputText = self.focus {
                true
            } else {
                false
            },
            self.theme.get_theme(),
        );
        self.output_text.draw(
            frame,
            self.layouts.output_text,
            if let Focus::OutputText = self.focus {
                true
            } else {
                false
            },
            self.theme.get_theme(),
        );
        self.stack.draw(
            frame,
            self.layouts.cipherstack,
            if let Focus::CipherStack = self.focus {
                true
            } else {
                false
            },
            self.theme.get_theme(),
        );
        self.theme.draw(
            frame,
            self.layouts.theme_editor,
            if let Focus::Theme = self.focus {
                true
            } else {
                false
            },
            self.theme.get_theme(),
        )
    }

    pub fn update_cipherview(&mut self) {
        if let Some(cipherview) = &mut self.cipherview {
            if let Some(index) = self.stack.selected {
                cipherview.assign(index, &self.stack.ciphers[index], &self.input_text.text)
            } else {
                self.cipherview = None;
            }
        } else {
            if let Some(index) = self.stack.selected {
                self.cipherview = Some(AppCipher::new(
                    index,
                    &self.stack.ciphers[index],
                    &self.input_text.text,
                ));
            }
        }
    }

    pub fn update(&mut self, msg: Message) -> Option<Message> {
        match msg {
            Message::SwitchProcess(p) => {self.process = p; None}
            Message::CipherText => {
                self.stack
                .stack_cipher(&self.input_text.text, &mut self.output_text.text);
                None
            }
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
            Message::Focus(f) => {
                self.focus = f;
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

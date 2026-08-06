use crate::ciphermod::{CipherText, CipherType};

use ratatui::{DefaultTerminal, Frame, buffer::Buffer, layout::Rect, widgets::Widget};
use std::io;

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

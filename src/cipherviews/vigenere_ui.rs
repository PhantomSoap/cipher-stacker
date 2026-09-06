use ratatui::{
    layout::Rect, style::{Color, Style}, widgets::{Block, Paragraph},
};

use crate::CipherView;
pub struct VigenereView {
    pub code: String,
}
impl VigenereView {
    pub fn new(code: String) -> Self {
        Self { code }
    }
}
impl CipherView for VigenereView {
    fn draw(&self, frame: &mut ratatui::prelude::Frame, area: Rect,focus : bool,scroll : (u16,u16)) {
        let vigenere_grid = format!("Vigenere Cipher\nCode: '{}'\n", self.code);
        let style = if focus {Style::default().fg(Color::Blue)} else {Style::default()};

        frame.render_widget(Paragraph::new(vigenere_grid).block(Block::bordered().border_style(style)).scroll(scroll), area);
    }
}

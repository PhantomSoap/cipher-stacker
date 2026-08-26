use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, Paragraph, Widget},
};

use crate::ui::ciphers_ui::cipherview::CipherView;
pub struct VigenereView<'a> {
    pub code : &'a String
}
impl<'a> VigenereView<'a> {
    pub fn new(code : &'a String) -> Self {
        Self {
            code,
        }
    }
} 
impl<'a> CipherView for VigenereView<'a> {
    fn draw(&self,frame : &mut ratatui::prelude::Frame,area : Rect) {
        let vigenere_grid = format!("Vigenere Cipher\nCode: '{}'\n",self.code);

        frame.render_widget(Paragraph::new(vigenere_grid).block(Block::bordered()), area);
    }
}
pub fn render_vigenere(code: &String, area: Rect, buf: &mut Buffer) {
    let vigenere_grid = format!("Vigenere Cipher\nCode: '{code}'\n");

    Paragraph::new(vigenere_grid).centered().render(area, buf);
    Block::bordered().render(area, buf);
}

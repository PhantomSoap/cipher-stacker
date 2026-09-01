use ratatui::{
    layout::Rect,
    widgets::{Block, Paragraph},
};

use crate::CipherView;
pub struct VigenereView {
    pub code : String
}
impl VigenereView {
    pub fn new(code :  String) -> Self {
        Self {
            code,
        }
    }
} 
impl CipherView for VigenereView {
    fn draw(&self,frame : &mut ratatui::prelude::Frame,area : Rect) {
        let vigenere_grid = format!("Vigenere Cipher\nCode: '{}'\n",self.code);

        frame.render_widget(Paragraph::new(vigenere_grid).block(Block::bordered()), area);
    }
}


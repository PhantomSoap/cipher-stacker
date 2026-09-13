use ratatui::{
    layout::Rect,
    widgets::Paragraph,
};
use ratatui_themekit::{ThemeData, ThemeExt};

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
    fn draw(
        &self,
        frame: &mut ratatui::prelude::Frame,
        area: Rect,
        focus: bool,
        scroll: (u16, u16),
        t: ThemeData,
    ) {
        let vigenere_grid = format!("Vigenere Cipher\nCode: '{}'\n", self.code);

        frame.render_widget(
            Paragraph::new(vigenere_grid)
                .block(t.block("").focused(focus).build())
                .scroll(scroll),
            area,
        );
    }
}

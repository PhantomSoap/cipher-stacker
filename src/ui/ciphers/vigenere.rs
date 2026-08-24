use ratatui::{
    buffer::Buffer, layout::Rect, widgets::{Block, Paragraph, Widget},
};

pub fn render_vigenere(code: &String, area: Rect, buf: &mut Buffer) {
    let vigenere_grid = format!("Vigenere Cipher\nCode: '{code}'\n");

    Paragraph::new(vigenere_grid).centered().render(area, buf);
    Block::bordered().render(area,buf);

}

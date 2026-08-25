use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::Text,
    widgets::{Block, Paragraph, Widget},
};

pub fn render_atbash(area: Rect, buf: &mut Buffer) {
    let atbasher = format!(
        "Atbash Cipher
        {}
        | A | B | C | D | E | F | G | H | I | J | K | L | M | N | O | P | Q | R | S | T | U | V | W | X | Y | Z |
        | ↓   ↓   ↓   ↓   ↓   ↓   ↓   ↓   ↓   ↓   ↓   ↓   ↓   ↓   ↓   ↓   ↓   ↓   ↓   ↓   ↓   ↓   ↓   ↓   ↓   ↓ |
        | Z | Y | X | W | V | U | T | S | R | Q | P | O | N | M | L | K | J | I | H | G | F | E | D | C | B | A |
        {}
        ",

        "_".repeat(105),
        "‾".repeat(105),  
    );
    Paragraph::new(Text::from(atbasher))
        .centered()
        .render(area, buf);

    Block::bordered().render(area, buf);
}

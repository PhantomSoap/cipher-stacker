use ratatui::{
    Frame, layout::Rect, style::{Color, Style}, text::Text, widgets::{Block, Paragraph},
};

use crate::CipherView;
#[derive(Default)]
pub struct AtbashView {}
impl CipherView for AtbashView {
    fn draw(&self, frame: &mut Frame, area: Rect,focus : bool,scroll : (u16,u16)) {
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
    let style = if focus {Style::default().fg(Color::Blue)} else {Style::default()};

    frame.render_widget(
        Paragraph::new(Text::from(atbasher))
            .block(Block::bordered().border_style(style))
            .scroll(scroll),
        area,
        )
    }
}

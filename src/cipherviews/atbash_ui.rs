use ratatui::{
    Frame,
    layout::Rect,
    text::Text,
    widgets::Paragraph,
};
use ratatui_themekit::{ThemeData, ThemeExt};

use crate::CipherView;
#[derive(Default)]
pub struct AtbashView {}
impl CipherView for AtbashView {
    fn draw(&self, frame: &mut Frame, area: Rect, focus: bool, scroll: (u16, u16), t: ThemeData) {
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

        frame.render_widget(
            Paragraph::new(Text::from(atbasher))
                .block(t.block("").focused(focus).build())
                .scroll(scroll),
            area,
        )
    }
}

use ratatui::{
    Frame,
    layout::Rect,
    style::Stylize,
    text::{Line, Span, Text},
    widgets::{Block, Paragraph},
};

use crate::CipherView;

pub struct AffineView {
    pub a: u8,
    pub b: u8,
    pub text: String,
}

impl AffineView {
    pub fn new(a: u8, b: u8, text: &str) -> Self {
        Self {
            a,
            b,
            text: String::from(text),
        }
    }
}
impl CipherView for AffineView {
    fn draw(&self, frame: &mut Frame, area: Rect) {
        let a = self.a;
        let b = self.b;
        let mut affine_table = Text::from(format!(
            "Affine Cipher\na: {a} | b: {b}\n\n({a})(x) + {b} Mod 26\n\n"
        ));

        for chr in self.text.chars() {
            if chr.is_alphabetic() {
                let base = if chr.is_ascii_uppercase() { b'A' } else { b'a' };
                let x = chr as u8 - base;
                let mut letter_line: Vec<Span> = Vec::new();
                letter_line.push(Span::raw("| "));
                letter_line.push(Span::raw(format!("{chr}")).yellow());
                letter_line.push(Span::raw(" |"));
                letter_line.push(Span::raw(format!(
                    "| {x:02} | ({a})({x:02}) + {b:02} Mod 26 | {:02} | ",
                    (x as u16 * a as u16 + b as u16) % 26
                )));
                letter_line.push(
                    Span::raw(format!(
                        "{}",
                        (((x as u16 * a as u16 + b as u16) % 26 + base as u16) as u8 as char)
                    ))
                    .yellow(),
                );
                letter_line.push(Span::raw(" |"));
                affine_table.push_line(Line::from(letter_line));
            } else {
                affine_table.push_line(Line::from(format!(
                    "| {chr} || 00 | (0)(00) + 00 Mod 26 | 00 | {chr} |"
                )));
            }
        }
        frame.render_widget(
            Paragraph::new(affine_table)
                .centered()
                .block(Block::bordered()),
            area,
        )
    }
}

use ratatui::{buffer::Buffer, layout::Rect, style::Stylize, text::{Line, Span, Text}, widgets::{Paragraph, Widget}};

pub fn render_affine(text: &str, shift: u8, multiplyer: u8, area: Rect, buf: &mut Buffer) {
    let mut affine_table = Text::from(format!(
        "Affine Cipher\nShift: {shift} | multiplyer: {multiplyer}\n\n({multiplyer})(x) + {shift} Mod 26\n\n"
    ));

    for chr in text.chars() {
        let num = chr as u8 - b'A';
        let mut letter_line: Vec<Span> = Vec::new();
        letter_line.push(Span::raw("| "));
        letter_line.push(Span::raw(format!("{chr}")).yellow());
        letter_line.push(Span::raw(" |"));
        letter_line.push(Span::raw(format!(
            "| {num:02} | ({multiplyer})({num:02}) + {shift:02} Mod 26 | {:02} | ",
            (num as u16 * multiplyer as u16 + shift as u16) % 26
        )));
        letter_line.push(
            Span::raw(format!(
                "{}",
                (((num * multiplyer + shift) % 26 + b'a') as char).to_uppercase()
            ))
            .yellow(),
            
        );
        letter_line.push(Span::raw(" |"));
        affine_table.push_line(Line::from(letter_line));
    }
    Paragraph::new(affine_table).centered().render(area, buf);
    
}
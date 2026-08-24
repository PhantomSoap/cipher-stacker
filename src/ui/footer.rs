use ratatui::{buffer::Buffer, style::Stylize, text::{Line, Span, Text}, widgets::{Paragraph, Widget, Wrap}};

use crate::{app::App, ui::ui_area::UiArea};

pub fn render_footer(state: Text<'_>, app: &App, area: UiArea, buf: &mut Buffer) {
    let cipher_list = if let Some(index) = app.stack.selected {
        let mut cipher_line = Vec::new();
        cipher_line.push(Span::raw("["));
        
        for (indx, cipher) in app.stack.ciphers.iter().enumerate() {
            if index == indx {
                cipher_line.push(Span::raw(format!("{cipher:?}")).blue());
            } else {
                cipher_line.push(Span::raw(format!("{cipher:?}")));
            }

            if indx != app.stack.ciphers.len() - 1 {
                cipher_line.push(Span::raw(", "));
            }
        }
        cipher_line.push(Span::raw("]"));
        Line::from(cipher_line)
    } else {
        Line::from(format!("{:?}", app.stack.ciphers))
    };

    state.render(area.instructions, buf);
    
    let mut history_text = Text::default();
    history_text.push_line(Line::from("History:"));
    history_text.push_line(Line::from(format!(
        "Plainttext -> {}",
        app.history.first().unwrap_or(&String::new())
    )));

    for (index, cipher) in app.stack.ciphers.iter().enumerate() {
        if let Some(hist_item) = app.history.get(index + 1) {
            history_text.push_line(Line::from(format!("{cipher:?} -> {hist_item}")));
        }
    }
    Paragraph::new(Line::from(format!("PlainText: {}", app.plaintext)))
        .wrap(Wrap { trim: true })
        .render(area.plaintext, buf);
    Paragraph::new(Line::from(format!("CipherText: {}", app.ciphertext)))
        .wrap(Wrap { trim: true })
        .render(area.ciphertext, buf);
    Paragraph::new(cipher_list)
        .wrap(Wrap { trim: true })
        .render(area.cipher_list, buf);
    Paragraph::new(history_text)
        .centered()
        .wrap(Wrap { trim: true })
        .render(area.history, buf);
}
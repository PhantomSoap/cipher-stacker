use crate::app::{App, AppState};
use crate::ciphermod::CipherType;
use crate::ui::ui_area::UiArea;
use ciphers::{Caesar, Cipher};
use ratatui::text::{Span, Text};
use ratatui::widgets::Wrap;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{Block, Paragraph, Widget},
};
use std::fmt::Write;
use std::mem::discriminant;




pub fn render_vigenere(code: &String, area: Rect, buf: &mut Buffer) {
    let vigenere_grid = format!("Vigenere Cipher\nCode: '{code}'\n");

    Paragraph::new(vigenere_grid).centered().render(area, buf);
}

pub fn render_rail_fence(text: &str, key: u8, area: Rect, buf: &mut Buffer) {
    let rails = key as usize; //2
    let fences = text.len(); //11
    let mut railfence = format!("RailFence Cipher\nKey: {key}\n");
    railfence.push_str(&"_".repeat(fences * 4 + 1));
    railfence.push('\n');
    let mut fenced_rails: Vec<char> = vec![' '; rails * fences];
    let letters: Vec<char> = text.chars().collect();
    let mut rows = 0;
    let mut down = false;
    for (index, &chr) in letters.iter().enumerate() {
        if rows == rails - 1 || rows == 0 {
            down = !down;
        }
        fenced_rails[(fences * (rows)) + index] = chr;
        if rails != 1 {
            if down {
                rows += 1;
            } else {
                rows -= 1;
            }
        }
    }
    //0,1,2,1,0
    for i in 0..rails {
        railfence.push('|');
        for chr in &fenced_rails[(fences * i)..fences * (i + 1)] {
            let _ = write!(railfence, " {chr} |");
        }
        railfence.push('\n');
    }

    railfence.push_str(&"‾".repeat(fences * 4 + 1));
    Paragraph::new(Text::from(railfence))
        .centered()
        .render(area, buf);
}
pub fn render_block(area: Rect, buf: &mut Buffer) {
    let block = Block::bordered()
        .title(Line::from(" Cipher Stacker ".bold()).centered())
        .border_set(border::THICK);
    block.render(area, buf);
}

pub fn render(app: &App, area: Rect, buf: &mut Buffer) {
    let areas = crate::ui::ui_area::UiArea::new(area);
    if let Some(index) = app.stack.selected {
        let cipher = &app.stack.ciphers[index];
        let text = &app.history[index];
        match cipher {
            CipherType::Caeser(shift) => {
                crate::ui::ciphers::caeser::render_caesar(*shift, areas.cipher, buf);
            }
            CipherType::Vigenere(code) => {
                render_vigenere(&code, areas.cipher, buf);
            }
            CipherType::RailFence(key) => {
                render_rail_fence(text, *key, areas.cipher, buf);
            }
            CipherType::Atbash => crate::ui::ciphers::atbash::render_atbash(areas.cipher, buf),
            CipherType::Affine(a, b) => {
                crate::ui::ciphers::affine::render_affine(text, *b, *a, areas.cipher, buf);
            }
        }
    } else {
        if let AppState::EditingText(Some(cipher)) = &app.state {
            let text = &app.history.last().unwrap_or(&app.history[0]);
            match cipher {
                CipherType::Caeser(shift) => {
                    crate::ui::ciphers::caeser::render_caesar(*shift, areas.cipher, buf);
                }
                CipherType::Vigenere(code) => {
                    render_vigenere(&code, areas.cipher, buf);
                }
                CipherType::RailFence(key) => {
                    render_rail_fence(text, *key, areas.cipher, buf);
                }
                CipherType::Atbash => crate::ui::ciphers::atbash::render_atbash(areas.cipher, buf),
                CipherType::Affine(a, b) => {
                    crate::ui::ciphers::affine::render_affine(text, *b, *a, areas.cipher, buf);
                }
            }
        } else {
            Paragraph::new(Text::from("CipherStacker"))
                .centered()
                .render(areas.cipher, buf);
        }
    }

    let state_text: Text<'_> = match &app.state {
        AppState::EditingText(None) => Text::from(vec![
            Line::from("Welcome to Cipher Stacker"),
            Line::from(""),
            Line::from("<Tab> Next Cipher"),
        ]),
        AppState::EditingText(Some(cipher)) if let Some(index) = app.stack.selected => {
            Text::from(vec![
                Line::from(format!("{cipher} ({index})")),
                Line::from(""),
                Line::from("<+> to Add Cipher"),
                Line::from("<Enter> to Edit selected Cipher"),
                Line::from("<'-'> to Delete Selected Cipher"),
                Line::from("<Tab> Next Cipher"),
                Line::from("<- Access Cipher in List ->"),
            ])
        }
        AppState::EditingText(Some(_cipher)) => Text::from(vec![
            Line::from("<+> to Add Cipher"),
            Line::from("<'-'> to Delete Cipher"),
            Line::from("<Tab> Next Cipher"),
        ]),
        AppState::CurrentlyEditingCiphers(indx) => {
            let cipher = app.stack.ciphers.get(*indx).unwrap();

            let mut text = Text::from(vec![
                Line::from(format!("{cipher} | (Editing)")),
                Line::from(""),
                Line::from("<Tab> Next Cipher"),
                Line::from("<Enter> to return"),
            ]);
            text.lines.extend(Text::from(cipher.instructions()).lines);
            text
        }
    };
    render_cipher_side_bar(areas.ciphers_side_bar, buf, app);
    render_footer(state_text, app, areas, buf);
    render_block(area, buf);
}

fn render_footer(state: Text<'_>, app: &App, area: UiArea, buf: &mut Buffer) {
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

fn render_cipher_side_bar(area: Rect, buf: &mut Buffer, app: &App) {
    let cipher = match &app.state {
        AppState::CurrentlyEditingCiphers(indx) => Some(&app.stack.ciphers[*indx]),
        AppState::EditingText(Some(cipher)) => Some(cipher),
        AppState::EditingText(None) => None,
    };
    let mut sidebar: Vec<Line> = Vec::new();
    sidebar.push(Line::from("-".repeat(20)));
    if let Some(cipher) = cipher {
        let mut cipher_name = CipherType::first();
        
        for _ in 0..5 {
            if discriminant(&cipher_name) == discriminant(&cipher) {
                sidebar.push(Line::from(vec![Span::raw("| "),Span::raw(cipher_name.name()).blue(),Span::raw(" |")]))
            } else {
                sidebar.push(Line::from(format!("| {} |",cipher_name.name())))
            }
            cipher_name = cipher_name.next()
        }

        
    } else {
        let mut cipher_name = CipherType::first();
        
        for _ in 0..5 {
            sidebar.push(Line::from(format!("| {} |",cipher_name.name())));
            cipher_name = cipher_name.next();
        }
        
    }
    let items = [
        "Caeser Cipher",
        "Vigenere Cipher",
        "RailFence Cipher",
        "Atbash Cipher",
        "Affine Cipher",
        ];
        
    
}

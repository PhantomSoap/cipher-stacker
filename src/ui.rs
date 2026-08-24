use crate::app::{App, AppState};
use crate::ciphermod::CipherType;
use crate::ui_area::UiArea;
use ciphers::{Caesar, Cipher};
use ratatui::text::{Span, Text};
use ratatui::widgets::Wrap;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{Block, Paragraph, Widget},
};
use std::fmt::Write;
fn render_atbash(area: Rect, buf: &mut Buffer) {
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
}
fn render_affine(text: &str, shift: u8, multiplyer: u8, area: Rect, buf: &mut Buffer) {
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
pub fn render_caesar(shift: i8, area: Rect, buf: &mut Buffer) {
    let ciphered_alphabet = Caesar::new(shift as u8)
        .encipher("ABCDEFGHIJKLMNOPQRSTUVWXYZ")
        .unwrap();
    let mut ciphered_boxed_alphabet = String::with_capacity(107);
    ciphered_boxed_alphabet.push('|');
    for c in ciphered_alphabet.chars() {
        ciphered_boxed_alphabet.push(' ');
        ciphered_boxed_alphabet.push(c);
        ciphered_boxed_alphabet.push(' ');
        ciphered_boxed_alphabet.push('|');
    }

    let caesar_shifter = format!(
        "Caesar Shifter
        {}
        | A | B | C | D | E | F | G | H | I | J | K | L | M | N | O | P | Q | R | S | T | U | V | W | X | Y | Z |
        | ↓   ↓   ↓   ↓   ↓   ↓   ↓   ↓   ↓   ↓   ↓   ↓   ↓   ↓   ↓   ↓   ↓   ↓   ↓   ↓   ↓   ↓   ↓   ↓   ↓   ↓ |
        {ciphered_boxed_alphabet}
        {}
        Shift: {shift}",

        "_".repeat(105),
        "‾".repeat(105),  
    );

    Paragraph::new(Text::from(caesar_shifter))
        .centered()
        .render(area, buf);
}

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
    let areas = crate::ui_area::UiArea::new(area);
    if let Some(index) = app.stack.selected {
        let cipher = &app.stack.ciphers[index];
        let text = &app.history[index + 1];
        match cipher {
            CipherType::Caeser(shift) => {
                render_caesar(*shift, areas.cipher, buf);
            }
            CipherType::Vigenere(code) => {
                render_vigenere(&code, areas.cipher, buf);
            }
            CipherType::RailFence(key) => {
                render_rail_fence(text, *key, areas.cipher, buf);
            }
            CipherType::Atbash => render_atbash(areas.cipher, buf),
            CipherType::Affine(a, b) => {
                render_affine(text, *b, *a, areas.cipher, buf);
            }
        }
    } else {
        if let AppState::EditingText(Some(cipher)) = &app.state {
            let text = &app.history.last().unwrap_or(&app.history[0]);
            match cipher {
                CipherType::Caeser(shift) => {
                    render_caesar(*shift, areas.cipher, buf);
                }
                CipherType::Vigenere(code) => {
                    render_vigenere(&code, areas.cipher, buf);
                }
                CipherType::RailFence(key) => {
                    render_rail_fence(text, *key, areas.cipher, buf);
                }
                CipherType::Atbash => render_atbash(areas.cipher, buf),
                CipherType::Affine(a, b) => {
                    render_affine(text, *b, *a, areas.cipher, buf);
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
        AppState::EditingText(Some(cipher)) => Text::from(vec![
            Line::from(cipher.name()),
            Line::from(""),
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

use crate::app::{App, AppState};
use crate::ciphermod::CipherType;
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
fn render_atbash(area: Rect, buf: &mut Buffer) {
    Paragraph::new("Atbash Cipher").centered().render(area, buf);
}
fn render_affine(shift: &u8, multiplyer: &u8, area: Rect, buf: &mut Buffer) {
    Paragraph::new(format!(
        "Affine Cipher\nShift: {} | multiplyer: {}",
        shift, multiplyer
    ))
    .centered()
    .render(area, buf);
}
pub fn render_caesar(shift: &i32, area: Rect, buf: &mut Buffer) {
    let ciphered_alphabet = Caesar::new(*shift as u8)   
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

        "-".repeat(105),
        "-".repeat(105),  
    );

    Paragraph::new(Text::from(caesar_shifter)).centered().render(area, buf);
}

pub fn render_vigenere(code: &String, area: Rect, buf: &mut Buffer) {
    let vigenere_grid = format!("Vigenere Cipher\nCode: '{}'\n", code);
    Paragraph::new(vigenere_grid).centered().render(area, buf);
}

pub fn render_rail_fence(key: &i32, area: Rect, buf: &mut Buffer) {
    Paragraph::new(format!("RailFence Cipher\nKey: {}", key))
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
    let layouts = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),  // Border
            Constraint::Length(1),  //Cipher Title
            Constraint::Length(28), // cipher visualization
            Constraint::Length(9),  // plaintext
        ])
        .split(area);
    let middles = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Length(115),
            Constraint::Percentage(20),
        ])
        .split(layouts[2]);

    if let AppState::CurrentlyEditingCiphers(indx) = app.state {
        match app.text.ciphers.get(indx).unwrap() {
            CipherType::Caeser(shift) => {
                render_caesar(shift, middles[1], buf);
            }
            CipherType::Vigenere(code) => {
                render_vigenere(code, middles[1], buf);
            }
            CipherType::RailFence(key) => {
                render_rail_fence(key, middles[1], buf);
            }
            CipherType::Atbash => {}
            CipherType::Affine(shift, multiplyer) => {
                render_affine(shift, multiplyer, middles[1], buf);
            }
        }
    }

    if let AppState::EditingText(Some(cipher)) = &app.state {
        match cipher {
            CipherType::Caeser(shift) => {
                render_caesar(shift, middles[1], buf);
            }
            CipherType::Vigenere(code) => {
                render_vigenere(code, middles[1], buf);
            }
            CipherType::RailFence(key) => {
                render_rail_fence(key, middles[1], buf);
            }
            CipherType::Atbash => render_atbash(middles[1], buf),
            CipherType::Affine(shift, multiplyer) => {
                render_affine(shift, multiplyer, middles[1], buf);
            }
        }
    }

    let state_text : Text<'_> = match &app.state {
        AppState::EditingText(None) => {
            Text::from(vec![Line::from("Welcome to Cipher Stacker"),Line::from(""),Line::from("<Tab> Next Cipher")])
        }
        AppState::EditingText(Some(cipher)) if let Some(index) = app.text.selected => {
            Text::from(vec![
                Line::from(format!("{cipher} ({index})")),
                Line::from(""),
                Line::from("<+> to Add Cipher"),
                Line::from("<'-'> to Delete Selected Cipher"),
                Line::from("<Tab> Next Cipher"),
                Line::from("<- Access Cipher in List ->"),
                ])
            
        }
        AppState::EditingText(Some(cipher)) => {
            Text::from(vec![
                Line::from(cipher.name()),
                Line::from(""),
                Line::from("<+> to Add Cipher"),
                Line::from("<'-'> to Delete Cipher"),
                Line::from("<Tab> Next Cipher"),
                Line::from("<Access Cipher in List>"),
            ])
        },
        AppState::CurrentlyEditingCiphers(indx) => {
            let cipher = app.text.ciphers.get(*indx).unwrap();
            
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
    render_footer(
        state_text,
        &app.history,
        &app.text.text,
        &app.text.ciphered,
        &app.text.ciphers,
        app.text.selected,
        layouts[3],
        buf,
    );
    render_block(area, buf);
}

fn render_footer(
    state: Text<'_>,
    history: &[String],
    plain_text: &String,
    cipher_text: &String,
    cipher_list: &Vec<CipherType>,
    selected_index: Option<usize>,
    area: Rect,
    buf: &mut Buffer,
) {
    let footer_area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(40),
            Constraint::Percentage(40),
        ])
        .split(area);

    let mut footer_text = Text::default();
    
    footer_text.push_line(Line::from(format!("PlainText: {}", plain_text)));

    if let Some(index) = selected_index {
        let mut cipher_line = Vec::new();
        cipher_line.push(Span::raw("["));
        
        for (indx, cipher) in cipher_list.iter().enumerate() {
            
            if index == indx {
                cipher_line.push(Span::raw(format!("{:?}", cipher)).blue());
            } else {
                cipher_line.push(Span::raw(format!("{:?}", cipher)));
            }
            
            if indx != cipher_list.len() - 1 {
                cipher_line.push(Span::raw(", "));
            }
        }
        cipher_line.push(Span::raw("]"));
        footer_text.push_line(Line::from(cipher_line));
    } else {
        footer_text.push_line(Line::from(format!("{:?}", cipher_list)));
    }

    footer_text.push_line(Line::from(format!("CipherText: {}", cipher_text)));
    footer_text.push_line(Line::from("")); 
    footer_text.lines.extend(state);

    let mut history_text = Text::default();
    history_text.push_line(Line::from("History:"));
    history_text.push_line(Line::from(format!("Plainttext -> {}", history.first().unwrap_or(&String::from("")))));

    for (index, cipher) in cipher_list.iter().enumerate() {
        if let Some(hist_item) = history.get(index + 1) {
            history_text.push_line(Line::from(format!("{:?} -> {}", cipher, hist_item)));
        }
    }

    Paragraph::new(footer_text)
        .centered()
        .wrap(Wrap { trim: true })
        .render(footer_area[1], buf);

    Paragraph::new(history_text)
        .centered()
        .wrap(Wrap { trim: true })
        .render(footer_area[2], buf);
}

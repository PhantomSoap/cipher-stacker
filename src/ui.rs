use crate::app::{App, AppState};
use crate::ciphermod::CipherType;
use ciphers::{Caesar, Cipher};

use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::Stylize,
    symbols::border,
    text::{Line},
    widgets::{Block, Paragraph, Widget},
};

pub fn render_affine(shift: &u8, multiplyer: &u8, area: Rect, buf: &mut Buffer) {
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

    Paragraph::new(caesar_shifter).centered().render(area, buf);
}

pub fn render_vigenere(code: &String, area: Rect, buf: &mut Buffer) {
    Paragraph::new(format!("Vigenere Cipher\nCode: {}", code))
        .centered()
        .render(area, buf);
}

pub fn render_cipher_list(cipher_list: &Vec<CipherType>, area: Rect, buf: &mut Buffer) {
    Paragraph::new(format!("{:?}", cipher_list))
        .centered()
        .render(area, buf);
}

pub fn render_cipher_text(text: &String, area: Rect, buf: &mut Buffer) {
    Paragraph::new(format!("Text: {}", text))
        .centered()
        .render(area, buf);
}

pub fn render_rail_fence(key: &i32, area: Rect, buf: &mut Buffer) {
    Paragraph::new(format!("RailFence Cipher\nKey: {}", key))
        .centered()
        .render(area, buf);
}
pub fn render_block(instuctions: Line<'_>, area: Rect, buf: &mut Buffer) {
    let block = Block::bordered()
        .title(Line::from(" Cipher Stacker ".bold()).centered())
        .title_bottom(instuctions.centered())
        .border_set(border::THICK);
    block.render(area, buf);
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layouts = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),  // Border
                Constraint::Length(1),  //Cipher Title
                Constraint::Length(28), // cipher visualization
                Constraint::Length(1),  // plaintext
                Constraint::Length(1),  // space
                Constraint::Length(1),  //ciphertext
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(2),
                Constraint::Length(1), // Border
            ])
            .split(area);
        let middles = Layout::default().direction(Direction::Horizontal).constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(60),
            Constraint::Percentage(20)
        ]).split(layouts[2]);
        if let AppState::CurrentlyEditingCiphers(indx) = self.state {
            match self.text.ciphers.get(indx).unwrap() {
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

        if let AppState::EditingText(Some(cipher)) = &self.state {
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
                CipherType::Atbash => {}
                CipherType::Affine(shift, multiplyer) => {
                    render_affine(shift, multiplyer, middles[1], buf);
                }
            }
        }
        render_block(
            Line::from(vec![
                " Caesar ".into(),
                "<Up>".blue().bold(),
                " Vinegere ".into(),
                "<Down>".blue().bold(),
                " Railfence ".into(),
                "<Right>".blue(),
                " Quit ".into(),
                "<Esc> ".blue().bold(),
            ]),
            area,
            buf,
        );

        render_cipher_text(&self.text.ciphered, layouts[5], buf);
        render_cipher_list(&self.text.ciphers, layouts[6], buf);
        let state_string = match &self.state {
            AppState::EditingText(None) => String::from("Welcome to Cipher Stacker"),
            AppState::EditingText(Some(cipher)) => {
                format!("{} | (Read Only)\r\n<Enter> to Add Cipher", cipher)
            }
            AppState::CurrentlyEditingCiphers(indx) => {
                format!("{} | (Editing)", self.text.ciphers.get(*indx).unwrap())
            }
        };
        Paragraph::new(state_string)
            .centered()
            .render(layouts[8], buf);
    }
}

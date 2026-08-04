use crate::app::{App, AppState};
use crate::ciphermod::CipherType;
use ciphers::{Caesar, Cipher};

use ratatui::widgets::Wrap;
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
    Paragraph::new(format!("Vigenere Cipher\nCode: '{}'", code))
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
                Constraint::Length(7),  // plaintext
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

        let state_string = match &self.state {
            AppState::EditingText(None) => String::from("Welcome to Cipher Stacker"),
            AppState::EditingText(Some(cipher)) => {
                format!("{} | (Read Only)\r\n<Enter> to Add Cipher", cipher)
            }
            AppState::CurrentlyEditingCiphers(indx) => {
                format!("{} | (Editing)", self.text.ciphers.get(*indx).unwrap())
            }
        };
        render_footer(state_string,&self.history,&self.text.text,&self.text.ciphered,&self.text.ciphers,layouts[3],buf);

    }
}

fn render_footer(state : String,history : &Vec<String>,plain_text : &String,cipher_text : &String,cipher_list : &Vec<CipherType>,area : Rect,buf : &mut Buffer) {
    let footer_area = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(33),
                Constraint::Percentage(34),
                Constraint::Percentage(33),
            ])
            .split(area);
    let mut footer = String::new();
    footer.push_str(&format!("{:?}\n", cipher_list));
    footer.push_str(&format!("{}\n",cipher_text));
    footer.push_str(&format!("{}\n",state));

    let mut history_string = String::from(format!("History:\nPlainttext -> {}\n",history.get(0).unwrap_or(&String::from(""))));
    for (index,cipher) in cipher_list.iter().enumerate() {
        history_string.push_str(&format!("{:?} -> {}\n",cipher,history.get(index+1).unwrap()));
    }
    Paragraph::new(footer).centered().wrap(Wrap {trim : true}).render(footer_area[1],buf);
    Paragraph::new(history_string).centered().wrap(Wrap {trim : true}).render(footer_area[2],buf);
    
}



pub fn render_history(ciphers : &Vec<CipherType>,history : &Vec<String>,area : Rect,buf : &mut Buffer) {
    let mut history_string = String::from(format!("History:\nPlainttext -> {}\n",history.get(0).unwrap_or(&String::from(""))));
    
    for (index,cipher) in ciphers.iter().enumerate() {
        history_string.push_str(&format!("{:?} -> {}\n",cipher,history.get(index+1).unwrap()));
    }
    Paragraph::new(history_string).centered().render(area,buf);
}
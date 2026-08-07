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
    Paragraph::new(Text::from(atbasher)).centered().render(area, buf);
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

        "_".repeat(105),
        "‾".repeat(105),  
    );

    Paragraph::new(Text::from(caesar_shifter)).centered().render(area, buf);
}

pub fn render_vigenere(code: &String, area: Rect, buf: &mut Buffer) {
    let vigenere_grid = format!("Vigenere Cipher\nCode: '{}'\n", code);
    Paragraph::new(vigenere_grid).centered().render(area, buf);
}

pub fn render_rail_fence(text : &String,key: &i32, area: Rect, buf: &mut Buffer) {
    let rails = *key as usize; //2
    let fences = text.len(); //11
    let mut railfence = format!("RailFence Cipher\nKey: {}\n", key);
    railfence.push_str(&"_".repeat(fences * 4 + 1));
    railfence.push('\n');
    let mut fenced_rails : Vec<char> = vec![' '; rails * fences ];
    let letters : Vec<char> = text.chars().collect();
    let mut rows = 0;
    let mut down = false;
    for (index,&chr) in letters.iter().enumerate() {
        if rows == rails-1 || rows == 0 {
            down = !down;
        }
        fenced_rails[(fences*(rows))+index] = chr;
        if !(rails == 1) {
        if down {
            rows+=1;
        } else {
            rows-=1;
        }
        }
    }
    //0,1,2,1,0
    for i in 0..rails {
        railfence.push('|');
        for chr in &fenced_rails[(fences*i)..fences*(i+1)] {
            railfence.push_str(&format!(" {} |",chr));
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
    let layouts = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),  // Border
            Constraint::Length(1),  //Cipher Title
            Constraint::Length(28), // cipher visualization
            Constraint::Length(12),  
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
        let text= app.history.get(indx).unwrap_or(app.history.first().unwrap());
        match app.text.ciphers.get(indx).unwrap() {
            CipherType::Caeser(shift) => {
                render_caesar(shift, middles[1], buf);
            }
            CipherType::Vigenere(code) => {
                render_vigenere(code, middles[1], buf);
            }
            CipherType::RailFence(key) => {
                render_rail_fence(text,key, middles[1], buf);
            }
            CipherType::Atbash => render_atbash(middles[1], buf),
            CipherType::Affine(a, b) => {
                render_affine(b, a, middles[1], buf);
            }
        }
    }

    if let AppState::EditingText(Some(cipher)) = &app.state {
        let text = if let Some(index) = app.text.selected {
            app.history.get(index+1).unwrap_or(app.history.first().unwrap()) 
        }  else {
            app.history.first().unwrap()
        };
        match cipher {
            CipherType::Caeser(shift) => {
                render_caesar(shift, middles[1], buf);
            }
            CipherType::Vigenere(code) => {
                render_vigenere(code, middles[1], buf);
            }
            CipherType::RailFence(key) => {
                render_rail_fence(text,key, middles[1], buf);
            }
            CipherType::Atbash => render_atbash(middles[1], buf),
            CipherType::Affine(a, b) => {
                render_affine(b, a, middles[1], buf);
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
                Line::from("<Enter> to Edit selected Cipher"),
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
        app,
        layouts[3],
        buf,
    );
    render_block(area, buf);
}

fn render_footer(
    state: Text<'_>,
    app : &App,
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
    
    footer_text.push_line(Line::from(format!("PlainText: {}", app.text.text)));

    if let Some(index) = app.text.selected {
        let mut cipher_line = Vec::new();
        cipher_line.push(Span::raw("["));
        
        for (indx, cipher) in app.text.ciphers.iter().enumerate() {
            
            if index == indx {
                cipher_line.push(Span::raw(format!("{:?}", cipher)).blue());
            } else {
                cipher_line.push(Span::raw(format!("{:?}", cipher)));
            }
            
            if indx != app.text.ciphers.len() - 1 {
                cipher_line.push(Span::raw(", "));
            }
        }
        cipher_line.push(Span::raw("]"));
        footer_text.push_line(Line::from(cipher_line));
    } else {
        footer_text.push_line(Line::from(format!("{:?}", app.text.ciphers)));
    }

    footer_text.push_line(Line::from(format!("CipherText: {}", app.text.ciphered)));
    footer_text.push_line(Line::from("")); 
    footer_text.lines.extend(state);

    let mut history_text = Text::default();
    history_text.push_line(Line::from("History:"));
    history_text.push_line(Line::from(format!("Plainttext -> {}", app.history.first().unwrap_or(&String::from("")))));

    for (index, cipher) in app.text.ciphers.iter().enumerate() {
        if let Some(hist_item) = app.history.get(index + 1) {
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

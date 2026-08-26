use crate::app::{App, AppState};
use crate::cipher_stack::CipherType;


use ratatui::text::Text;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{Block, Paragraph, Widget},
};

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
                crate::ui::ciphers_ui::caesar_ui::render_caesar(*shift, areas.cipher, buf);
            }
            CipherType::Vigenere(code) => {
                crate::ui::ciphers_ui::vigenere_ui::render_vigenere(&code, areas.cipher, buf);
            }
            CipherType::RailFence(key) => {
                crate::ui::ciphers_ui::rail_fence_ui::render_rail_fence(text, *key, areas.cipher, buf);
            }
            CipherType::Atbash => crate::ui::ciphers_ui::atbash_ui::render_atbash(areas.cipher, buf),
            CipherType::Affine(a, b) => {
                crate::ui::ciphers_ui::affine_ui::render_affine(text, *a, *b, areas.cipher, buf);
            }
        }
    } else {
        if let AppState::EditingText(Some(cipher)) = &app.state {
            let text = &app.history.last().unwrap_or(&app.history[0]);
            match cipher {
                CipherType::Caeser(shift) => {
                    crate::ui::ciphers_ui::caesar_ui::render_caesar(*shift, areas.cipher, buf);
                }
                CipherType::Vigenere(code) => {
                    crate::ui::ciphers_ui::vigenere_ui::render_vigenere(&code, areas.cipher, buf);
                }
                CipherType::RailFence(key) => {
                    crate::ui::ciphers_ui::rail_fence_ui::render_rail_fence(
                        text,
                        *key,
                        areas.cipher,
                        buf,
                    );
                }
                CipherType::Atbash => crate::ui::ciphers_ui::atbash_ui::render_atbash(areas.cipher, buf),
                CipherType::Affine(a, b) => {
                    crate::ui::ciphers_ui::affine_ui::render_affine(text, *a, *b, areas.cipher, buf);
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
        ]),
        AppState::EditingText(Some(cipher)) if let Some(index) = app.stack.selected => {
            Text::from(vec![
                Line::from(format!("{cipher} ({index})")),
                Line::from(""),
                Line::from("<+> to Add Cipher"),
                Line::from("<Enter> to Edit selected Cipher"),
                Line::from("<'-'> to Delete Selected Cipher"),
                Line::from("<- Access Cipher in List ->"),
            ])
        }
        AppState::EditingText(Some(_cipher)) => Text::from(vec![
            Line::from("<+> to Add Cipher"),
            Line::from("<'-'> to Delete Cipher"),
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
    crate::ui::side_bar::render_cipher_side_bar(areas.ciphers_side_bar, buf, app);
    crate::ui::footer::render_footer(state_text, app, areas, buf);
    render_block(area, buf);
}

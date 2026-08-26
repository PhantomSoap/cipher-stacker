use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, StatefulWidget};

use crate::app::{App, AppState};
use crate::cipher_stack::CipherType;

pub fn render_cipher_side_bar(area: Rect, buf: &mut Buffer, app: &App) {
    let cipher_opt = match &app.state {
        AppState::EditingCipher(indx) => Some(&app.stack.ciphers[*indx]),
        AppState::EditingText(Some(cipher)) => Some(cipher),
        AppState::EditingText(None) => None,
    };
    let index_opt = match cipher_opt {
        Some(cipher) => match cipher {
            CipherType::Caeser(_) => Some(0),
            CipherType::Vigenere(_) => Some(1),
            CipherType::RailFence(_) => Some(2),
            CipherType::Atbash => Some(3),
            CipherType::Affine(_, _) => Some(4),
        },
        None => None,
    };
    let mut state = ListState::default();
    state.select(index_opt);

    let list_widget = List::new(vec![
        ListItem::from("Caesar Cipher"),
        ListItem::from("Vigenere Cipher"),
        ListItem::from("Railfence Cipher"),
        ListItem::from("Atbash Cipher"),
        ListItem::from("Affine Cipher"),
        
    ])
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title_bottom(" <Tab> to move to next cipher ")
            .title_top(" Available Ciphers "),
    )
    .highlight_style(Style::new().blue())
    .highlight_spacing(ratatui::widgets::HighlightSpacing::Always);

    StatefulWidget::render(list_widget, area, buf, &mut state);
}

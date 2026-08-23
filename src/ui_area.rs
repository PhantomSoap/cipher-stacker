use ratatui::layout::{self, Constraint, Direction, Layout, Rect};

pub struct UiArea {
    title : Rect,
    cipher_visualization : Rect,
    history : Rect,
    plaintext : Rect,
    ciphertext : Rect,
    instructions : Rect,
    cipher_list : Rect,
    cipher : Rect,
    ciphers_side_bar : Rect,
}

impl UiArea {
    pub fn new(area : Rect) -> Self{
        let layouts = Layout::default()
            .margin(1)
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(1), //Title
                Constraint::Length(30), // cipher vis
                Constraint::Length(2), //Gap
                /* 
                Constraint::Length(1), //plaintext
                Constraint::Length(3), //cipher list
                Constraint::Length(1), //cipher text
                Constraint::Length(1), //Gap
                Constraint::Length(1), //cipher
                Constraint::Length(1), //Gap
                Constraint::Length(7), // Instructions
                */
                Constraint::Length(15),
            ]).split(area);
            let middle = Layout::default()
                .margin(1)
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Percentage(20),
                    Constraint::Percentage(40),
                    Constraint::Percentage(20),
                ]).split(layouts[1]);
            let footer_middles = Layout::default()
                .margin(1)
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Percentage(20),
                    Constraint::Percentage(60),
                    Constraint::Percentage(40),
                ]).split(layouts[4]);

            let footer_areas = Layout::default()
                .margin(1)
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Length(1), //plaintext
                    Constraint::Length(3), //cipher list
                    Constraint::Length(1), //cipher text
                    Constraint::Length(1), //Gap
                    Constraint::Length(1), //cipher
                    Constraint::Length(1), //Gap
                    Constraint::Length(7), // Instructions
                ]).split(footer_middles[1]);
        Self {
            title : layouts[0],
            cipher_visualization: middle[1],
            history: footer_middles[2],
            plaintext: footer_areas[0],
            ciphertext: footer_areas[2],
            instructions: footer_areas[6],
            cipher_list: footer_areas[1],
            cipher: footer_areas[4],
            ciphers_side_bar : middle[0],
        }
    }
}
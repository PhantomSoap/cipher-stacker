use ratatui::layout::{Constraint, Direction, Layout, Rect};

pub struct AppLayout {
    pub plaintext : Rect, 
    pub ciphertext : Rect, 
    pub cipherstack : Rect,
    pub history : Rect,
    pub cipherview : Rect,
    
}

impl AppLayout {
    pub fn build(area : Rect) -> Self {
        let layouts = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(1),
                Constraint::Length(30),
                Constraint::Length(20),         
            ]).split(area);

        let middles = Layout::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .constraints([
                    Constraint::Percentage(20),
                    Constraint::Percentage(60),
                    Constraint::Percentage(9),
                ]).split(layouts[1]);

        let bottoms = Layout::default()
                    .direction(Direction::Horizontal)
                    .margin(1)
                    .constraints([
                        Constraint::Percentage(20),
                        Constraint::Percentage(60),
                        Constraint::Percentage(20),
                    ]).split(layouts[2]);

        let footer_lines = Layout::default()
                        .direction(Direction::Vertical)
                        .margin(1)
                        .constraints([
                            Constraint::Length(4),
                            Constraint::Length(1),
                            Constraint::Length(4),
                        ]).split(bottoms[1]);

        Self {
            plaintext: footer_lines[0],
            ciphertext: footer_lines[2],
            cipherstack: bottoms[0],
            history: bottoms[2],
            cipherview : middles[1],
        }
    }
}
use ratatui::layout::{Constraint, Direction, Layout, Rect};

pub struct AppLayout {
    pub plaintext: Rect,
    pub ciphertext: Rect,
    pub cipherstack: Rect,
    pub instructions : Rect,
    pub cipherview: Rect,
}

impl AppLayout {
    pub fn build(area: Rect) -> Self {
        let vertical_split = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Ratio(3,5),
                Constraint::Ratio(2,5),
            ]).split(area);

        let right_panel = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Ratio(3, 5),
                Constraint::Ratio(2, 5),
            ]).split(vertical_split[1]);

        let pieces = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(4),
                Constraint::Length(30),
                Constraint::Length(4),
            ]).split(vertical_split[0]);

        Self {
            plaintext: pieces[0],
            ciphertext: pieces[2],
            cipherstack: right_panel[0],
            instructions : right_panel[1],
            cipherview: pieces[1],
        }
    }
}

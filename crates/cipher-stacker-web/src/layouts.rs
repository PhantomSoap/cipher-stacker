use ratatui::layout::{Constraint, Layout, Margin, Rect};

pub struct AppLayout {
    pub plaintext: Rect,
    pub output_text: Rect,
    pub cipherview: Rect,
    pub cipherstack: Rect,
    pub theme_editor: Rect,
}

impl AppLayout {
    pub fn build(area: Rect) -> Self {
        let area = area.inner(Margin::new(4, 2));
        let [left_side, right_side] =
            Layout::horizontal([Constraint::Percentage(60), Constraint::Percentage(40)])
                .areas(area);

        let [plaintext, cipherview, output_text] = Layout::vertical([
            Constraint::Length(10),
            Constraint::Max(20),
            Constraint::Length(10),
        ])
        .areas(left_side);

        let [cipherstack, theme_editor] =
            Layout::vertical([Constraint::Percentage(40), Constraint::Percentage(40)])
                .areas(right_side);

        Self {
            plaintext,
            cipherview,
            output_text,
            cipherstack,
            theme_editor,
        }
    }
}

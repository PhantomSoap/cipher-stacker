use ratatui::layout::{Constraint, Direction, Layout, Margin, Rect};

pub struct AppLayout {
    pub plaintext: Rect,
    pub ciphertext: Rect,
    pub cipherview: Rect,
    pub cipherstack: Rect,
    pub instructions: Rect,
    pub theme_editor: Rect,
}

impl AppLayout {
    pub fn build(area: Rect) -> Self {
        // 1. Clear outer margins
        //let area = area.inner(Margin::new(4, 2));

        // 2. Split horizontally into Main (Left) and Sidebar (Right)
        let horizontal_split = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(60), // Left Side
                Constraint::Percentage(40), // Right Side
            ])
            .split(area);

        let left_side = horizontal_split[0];
        let right_side = horizontal_split[1];

        // 3. Sub-split LEFT side vertically
        let left_panel = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Max(3), // Plaintext Box
                Constraint::Min(3),    // Cipherview (grows dynamically)
                Constraint::Max(3), // Ciphertext Box
            ])
            .split(left_side);

        // 4. Sub-split RIGHT side vertically into THREE parts
        // This stops the border from running infinitely down the page
        let right_panel = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(33),          // Top section (Cipherstack/History)
                Constraint::Percentage(33),       // Instructions box (Fixed rows match your text)
                Constraint::Percentage(33),      // Themes list box (Fixed rows match theme items)
            ])
            .split(right_side);

        Self {
            plaintext: left_panel[0],
            cipherview: left_panel[1],
            ciphertext: left_panel[2],
            cipherstack: right_panel[0],
            instructions: right_panel[1],
            theme_editor: right_panel[2],
        }
    }
}

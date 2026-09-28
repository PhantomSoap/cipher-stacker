use ratatui::layout::{Constraint, Direction, Layout, Margin, Rect};

pub struct AppLayout {
    pub plaintext: Rect,
    pub ciphertext: Rect,
    pub cipherstack: Rect,
    pub theme_editer: Rect,
    pub cipherview: Rect,
}

impl AppLayout {
    pub fn build(area: Rect) -> Self {
        // Apply your outer margin padding
        let area = area.inner(Margin::new(4, 2));
        
        // 1. Split horizontally into a Left Main side and a Right Sidebar
        let horizontal_split = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(60), // Left Side (3/5)
                Constraint::Percentage(40), // Right Side (2/5)
            ])
            .split(area);

        // 2. Split the RIGHT side vertically into Cipherstack and Theme Editor
        let right_panel = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(50), // Cipherstack (Upper right box)
                Constraint::Percentage(50), // Theme Editor / Instructions (Lower right)
            ])
            .split(horizontal_split[1]);

        // 3. Split the LEFT side vertically into Plaintext, Cipherview, and Ciphertext
        // Fixed lengths for input/output boxes prevent terminal resize clipping
        let left_panel = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),       // Plaintext Box (Fixed height for top box)
                Constraint::Min(3),          // Cipherview takes up all remaining middle space
                Constraint::Length(3),       // Ciphertext Box (Fixed height for input box)
            ])
            .split(horizontal_split[0]);

        Self {
            plaintext: left_panel[0],
            cipherview: left_panel[1],
            ciphertext: left_panel[2],
            cipherstack: right_panel[0],
            theme_editer: right_panel[1],
        }
    }
}

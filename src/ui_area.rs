use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Widget},
};

pub struct UiArea {
    pub title: Rect,
    pub cipher: Rect,
    pub history: Rect,
    pub plaintext: Rect,
    pub ciphertext: Rect,
    pub instructions: Rect,
    pub cipher_list: Rect,
    pub cipher_detail: Rect,
    pub ciphers_side_bar: Rect,
}

impl UiArea {
    pub fn new(area: Rect) -> Self {
        let layouts = Layout::default()
            .margin(1)
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),  //Title
                Constraint::Length(30), // cipher vis
                Constraint::Length(2),  //Gap
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
            ])
            .split(area);
        let middle = Layout::default()
            .margin(1)
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(20),
                Constraint::Length(115),
                Constraint::Percentage(10),
            ])
            .split(layouts[1]);
        let footer_middles = Layout::default()
            .margin(1)
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(20),
                Constraint::Percentage(60),
                Constraint::Percentage(40),
            ])
            .split(layouts[3]);

        let footer_areas = Layout::default()
            .margin(1)
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1), //plaintext
                Constraint::Length(3), //cipher list
                Constraint::Length(1), //cipher text
                Constraint::Length(1), //Gap
                Constraint::Length(1), //cipher
                Constraint::Length(1), //Gap
                Constraint::Length(7), // Instructions
            ])
            .split(footer_middles[1]);
        Self {
            title: layouts[0],
            cipher: middle[1],
            history: footer_middles[2],
            plaintext: footer_areas[0],
            ciphertext: footer_areas[2],
            instructions: footer_areas[6],
            cipher_list: footer_areas[1],
            cipher_detail: footer_areas[4],
            ciphers_side_bar: middle[0],
        }
    }

    pub fn render_borders(&self, buf: &mut Buffer) {
        Block::bordered().title("Title").render(self.title, buf);
        Block::bordered().title("history").render(self.history, buf);
        Block::bordered()
            .title("plaintext")
            .render(self.plaintext, buf);
        Block::bordered()
            .title("ciphertext")
            .render(self.ciphertext, buf);
        Block::bordered()
            .title("instructions")
            .render(self.instructions, buf);
        Block::bordered()
            .title("cipher_list")
            .render(self.cipher_list, buf);
        Block::bordered()
            .title("ciphers_side_bar")
            .render(self.ciphers_side_bar, buf);
        Block::bordered()
            .title("cipher_detail")
            .render(self.cipher_detail, buf);
        Block::bordered().title("cipher").render(self.cipher, buf);
    }
}

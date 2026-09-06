use ratatui::style::{Color, Style};

use crate::components::Component;

pub struct Instructions {}

impl Component for Instructions {
    fn handle_key_events(&mut self, _key: crossterm::event::KeyEvent) -> Option<crate::Message> {
        None
    }

    fn draw(&self, frame: &mut ratatui::prelude::Frame, area: ratatui::prelude::Rect, focus: bool) {
        let style = if focus {Style::default().fg(Color::Blue)} else {Style::default()};
        
    }

    fn update(&mut self, _msg: crate::Message) -> Option<crate::Message> {
        None
    }
}
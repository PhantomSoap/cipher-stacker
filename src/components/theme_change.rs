use ratatui::{style::Modifier, widgets::{Block, List, ListItem, ListState}};
use ratzilla::event::{KeyCode, KeyEvent};

use crate::{Message, components::Component,theme::{Theme,PRESETS}};

pub struct ThemeChanger {
    list_state : Option<usize>
}

impl ThemeChanger {
    pub fn new() -> Self {
        Self {
            list_state : Some(3)
        }
    }

    pub fn get_theme(&self) -> Theme {
        PRESETS[self.list_state.unwrap_or(3)]

        
    }
}

impl Component for ThemeChanger {
    fn handle_key_events(&mut self, key: KeyEvent) -> Option<crate::Message> {
        

        match key.code {
            KeyCode::Esc => Some(Message::Exit),
            KeyCode::Up if let Some(i) = &mut self.list_state && *i !=0 => {
                *i -= 1;
                None
                
            }
            KeyCode::Down if let Some(i) = &mut self.list_state && *i !=PRESETS.len()-1 => {
                *i += 1;
                None
                
            }
            KeyCode::Tab => Some(Message::NextFocus),

            _ => None,
        }
    }

    fn draw(&self, frame: &mut ratatui::prelude::Frame, area: ratatui::prelude::Rect, focus: bool, t: Theme) {
       let block = Block::bordered().title("Themes").border_style(if focus {t.ring} else {t.border});
       frame.render_stateful_widget(
             List::new(
                PRESETS.iter().map(|t| ListItem::new(t.name))
            )
            .highlight_style(Modifier::REVERSED)
            .highlight_symbol("> ")
            .block(block), 
            area,
            &mut ListState::default().with_selected(self.list_state),
        );
    }

    fn update(&mut self, _msg: crate::Message) -> Option<crate::Message> {
        None
    }
}
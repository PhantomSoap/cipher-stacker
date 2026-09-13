use crossterm::event::{KeyCode, KeyEventKind};
use ratatui::{style::Modifier, widgets::{List, ListItem, ListState}};
use ratatui_themekit::{BUILTIN_THEMES, ThemeData, ThemeExt, available_theme_ids};

use crate::{Message, components::Component};

pub struct ThemeChanger {
    list_state : Option<usize>
}

impl ThemeChanger {
    pub fn new() -> Self {
        Self {
            list_state : Some(3)
        }
    }

    pub fn get_theme(&self) -> ThemeData {
        BUILTIN_THEMES[self.list_state.unwrap_or(3)]

        
    }
}

impl Component for ThemeChanger {
    fn handle_key_events(&mut self, key: crossterm::event::KeyEvent) -> Option<crate::Message> {
        if key.kind == KeyEventKind::Release {
            return None;
        }

        match key.code {
            KeyCode::Esc => Some(Message::Exit),
            KeyCode::Up if let Some(i) = &mut self.list_state && *i !=0 => {
                *i -= 1;
                None
                
            }
            KeyCode::Down if let Some(i) = &mut self.list_state && *i !=BUILTIN_THEMES.len()-1 => {
                *i += 1;
                None
                
            }
            KeyCode::Tab => Some(Message::NextFocus),

            _ => None,
        }
    }

    fn draw(&self, frame: &mut ratatui::prelude::Frame, area: ratatui::prelude::Rect, focus: bool, t: ThemeData) {
       let block = t.block("Themes").focused(focus).build();
       frame.render_stateful_widget(
             List::new(
                BUILTIN_THEMES.iter().map(|t| ListItem::new(t.name))
            )
            .highlight_style(Modifier::REVERSED)
            .highlight_symbol("> ")
            .block(block), 
            area,
            &mut ListState::default().with_selected(self.list_state),
        );
    }

    fn update(&mut self, msg: crate::Message) -> Option<crate::Message> {
        None
    }
}
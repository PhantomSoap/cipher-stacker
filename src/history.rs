use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, MouseEvent};
use ratatui::{Frame, layout::Rect, style::Color, text::{Line, Text}, widgets::{Block, ListState, Paragraph, Wrap}};

use crate::{CipherStack, Message, cipher_stack::CipherType};

#[derive(Default)]
pub struct History {
    pub list : Vec<String>,
    pub scroll : usize,
}

impl History {
    pub fn draw(&self,frame : &mut Frame,area : Rect,cipherstack : &CipherStack,focus : bool) {
    let mut history_text = Text::default();
    history_text.push_line(Line::from("History:"));
    history_text.push_line(Line::from(format!(
        "Plainttext -> {}",
        self.list.first().unwrap_or(&String::new())
    )));

    for (index, cipher) in cipherstack.ciphers.iter().enumerate() {
        if let Some(hist_item) = self.list.get(index + 1) {
            history_text.push_line(Line::from(format!("{cipher:?} -> {hist_item}")));
        }
    }

    frame.render_widget(if focus {
        Paragraph::new(history_text)
        .wrap(Wrap {trim : true})
        .block(Block::bordered().border_style(Color::Blue))
        .scroll((self.scroll as u16,0))
        } else {
            Paragraph::new(history_text)
            .wrap(Wrap {trim : true})
            .block(Block::bordered())
            .scroll((self.scroll as u16,0))
        },
        area,
    )


        
    }
    pub fn handle_key_events(&mut self,key : KeyEvent)  -> Option<Message>{
        if let KeyEventKind::Release =  key.kind {
            return None
        }

        match key.code {
            KeyCode::Esc => {Some(Message::Exit)},
            KeyCode::Tab => Some(Message::NextFocus),

            _ => None
        }
    }

    pub fn handle_mouse_events(&mut self, m : MouseEvent) {
        match m.kind {
            crossterm::event::MouseEventKind::ScrollDown => {self.scroll+=1},
            crossterm::event::MouseEventKind::ScrollUp if self.scroll !=0=> {self.scroll-=1},
            crossterm::event::MouseEventKind::ScrollLeft => {},
            crossterm::event::MouseEventKind::ScrollRight => {},
            _ => {}
        }
    }
}

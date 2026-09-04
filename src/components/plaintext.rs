use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::{Frame, layout::Rect, style::{Color}, widgets::{Block, Paragraph, Wrap}};

use crate::Message;
use super::Component;

pub struct Plaintext {
    pub text : String,
    pub scroll : usize,
}
impl Plaintext {
    pub fn new(text : String) -> Self {
        Self {
            text,
            scroll : 0
        }
    }
}
impl Component for Plaintext {
    fn draw(&self,frame : &mut Frame,area : Rect,focus : bool) {
    
            let widget = if focus {
                Paragraph::new(format!("{}",self.text))
                    .wrap(Wrap { trim : false})
                    .block(Block::bordered().title_top("Plaintext").border_style(Color::Blue))
                    .scroll((if self.scroll == (self.text.len() / 137) || self.scroll ==0 {self.scroll} else {self.scroll-1} as u16,0))
            } else {
                Paragraph::new(format!("{}",self.text))
                    .wrap(Wrap { trim : false})
                    .block(Block::bordered().title_top("Plaintext"))
                    .scroll((if self.scroll == self.text.len() / 137  {self.scroll} else {self.scroll-1} as u16 as u16,0))
            };
        frame.render_widget(widget, area);
        
    }

    fn handle_key_events(&mut self,key : KeyEvent)  -> Option<Message>{
        if let KeyEventKind::Release =  key.kind {
            return None
        }

        match key.code {
            KeyCode::Esc => {Some(Message::Exit)},
            KeyCode::Up if self.scroll !=0 => {self.scroll -=1; None},
            KeyCode::Down  => {self.scroll +=1; None},
            KeyCode::Backspace => {
                self.text.pop();
                Some(Message::CipherPlaintext)
            },
            KeyCode::Char(c) => {
                self.text.push(c);
                Some(Message::CipherPlaintext)
            },
            KeyCode::Tab => Some(Message::NextFocus),

            _ => None
        }
    }
    
    fn update(&mut self, _msg : Message) -> Option<Message> {
        None
    }



    
}
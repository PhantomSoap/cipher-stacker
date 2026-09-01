

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, MouseEvent};
use ratatui::{Frame, layout::Rect, style::{Color, Style}, text::{Line, Span, Text}, widgets::{Block, Paragraph}};
use crate::CipherEdit;
use crate::{Message, CipherName};






pub struct EditingPanel {
    pub cipher : Option<CipherName>,
    pub scroll : (u16,u16),
}

impl EditingPanel {
    pub fn new() -> Self {
        Self {
            cipher : None,
            scroll : (0,0)
        }
    }

    pub fn draw(&self,frame : &mut Frame,area : Rect,focus : bool) {
        let cipher = match self.cipher {
            Some(cipher) => format!("{:?}",cipher),
            None => String::from("None Selected")

        };
        let panel = if focus {
            Paragraph::new(
                Text::from(
                    Line::from(
                        vec![
                            Span::raw("| "),
                            Span::styled(cipher, Style::default().fg(Color::Black).bg(Color::White)),
                            Span::raw(" |"),
                            Span::styled("Edit",Style::default().fg(Color::Blue)),
                            Span::raw(" Cipher |")
                        ]
                )
            )
            ).block(Block::bordered().border_style(Color::Blue).title_top("Edit Cipher"))
        } else {
            Paragraph::new(
                Text::from(
                    Line::from(
                        vec![
                            Span::raw("| "),
                            Span::styled(cipher, Style::default().fg(Color::Black).bg(Color::White)),
                            Span::raw(" |"),
                            Span::styled("Edit",Style::default().fg(Color::Blue)),
                            Span::raw(" Cipher |")
                        ]
                )
            )
            ).block(Block::bordered().title_top("Edit Cipher"))
        };
        frame.render_widget(panel, area);
    }

    pub fn handle_key_events(&mut self,key : KeyEvent)  -> Option<Message>{
        if let KeyEventKind::Release =  key.kind {
            return None
        }

        match key.code {
            KeyCode::Esc => {Some(Message::Exit)},
            KeyCode::Char(chr) => Some(Message::EditCipher(CipherEdit::PushChar(chr))),
            KeyCode::Backspace => Some(Message::EditCipher(CipherEdit::Popchar)),
            KeyCode::Up => Some(Message::EditCipher(CipherEdit::Up)),
            KeyCode::Down => Some(Message::EditCipher(CipherEdit::Down)),
            KeyCode::Left => Some(Message::EditCipher(CipherEdit::Left)),
            KeyCode::Right => Some(Message::EditCipher(CipherEdit::Right)),
            KeyCode::Tab => Some(Message::NextFocus),

            _ => None
        }
    }

    pub fn handle_mouse_events(&mut self, m : MouseEvent) {
        match m.kind {
            crossterm::event::MouseEventKind::ScrollDown => {self.scroll.0+=1},
            crossterm::event::MouseEventKind::ScrollUp if self.scroll.0 !=0=> {self.scroll.0-=1},
            crossterm::event::MouseEventKind::ScrollLeft => {},
            crossterm::event::MouseEventKind::ScrollRight => {},
            _ => {}
        }
    }

    pub fn update(&mut self,_msg : Message) {
        
    }
}
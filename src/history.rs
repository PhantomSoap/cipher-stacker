use ratatui::{Frame, layout::Rect, text::{Line, Text}, widgets::{ListState, Paragraph, Wrap}};

use crate::{CipherStack, cipher_stack::CipherType};

#[derive(Default)]
pub struct History {
    pub list : Vec<String>,
    pub scroll : usize,
}

impl History {
    pub fn draw(&self,frame : &mut Frame,area : Rect,cipherstack : &CipherStack) {
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

    frame.render_widget(
        Paragraph::new(history_text)
        .wrap(Wrap {trim : true})
        .scroll((self.scroll as u16,0)),
        area,
    )


        
    }
}

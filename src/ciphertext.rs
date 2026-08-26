use ratatui::{Frame, layout::Rect, widgets::{Block, Paragraph, Wrap}};

pub struct Ciphertext {
    pub text : String,
    pub scroll : usize,
}

impl Ciphertext {
    pub fn new(text : String) -> Self {
        Self {
            text,
            scroll : 0
        }
    }

    pub fn draw(&self,frame : &mut Frame,area : Rect) {
        let widget = Paragraph::new(format!("Ciphertext: {}",self.text))
            .wrap(Wrap { trim : true})
            .block(Block::bordered())//.border_style(Color::Blue))
            .scroll((self.scroll as u16,0));
        frame.render_widget(widget, area);
    }
}
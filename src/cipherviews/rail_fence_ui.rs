use std::fmt::Write;

use ratatui::{
    Frame, buffer::Buffer, layout::Rect, text::Text, widgets::{Block, Paragraph, Widget},
};

use crate::CipherView;
pub struct RailfenceView<'a> {
    pub key : u8,
    pub text : &'a str
}

impl<'a> RailfenceView<'a> {
    pub fn new(text : &'a str, key : u8) -> Self{
        Self {
            text,
            key,
        }
    }
} 
impl<'a> CipherView for RailfenceView<'a> {
    fn draw(&self,frame : &mut Frame, area : Rect) {
        let rails = self.key as usize; //2
        let fences = self.text.len(); //11
        let mut railfence = format!("RailFence Cipher\nKey: {rails}\n");
        railfence.push_str(&"_".repeat(fences * 4 + 1));
        railfence.push('\n');
        let mut fenced_rails: Vec<char> = vec![' '; rails * fences];
        let letters: Vec<char> = self.text.chars().collect();
        let mut rows = 0;
        let mut down = false;
        for (index, &chr) in letters.iter().enumerate() {
            if rows == rails - 1 || rows == 0 {
                down = !down;
            }
            fenced_rails[(fences * (rows)) + index] = chr;
            if rails != 1 {
                if down {
                    rows += 1;
                } else {
                    rows -= 1;
                }
            }
        }
        //0,1,2,1,0
        for i in 0..rails {
            railfence.push('|');
            for chr in &fenced_rails[(fences * i)..fences * (i + 1)] {
                let _ = write!(railfence, " {chr} |");
            }
            railfence.push('\n');
        }

        railfence.push_str(&"‾".repeat(fences * 4 + 1));
        frame.render_widget(Paragraph::new(Text::from(railfence))
            .centered()
            .block(Block::bordered())
            ,area)
        
    }
} 
pub fn render_rail_fence(text: &str, key: u8, area: Rect, buf: &mut Buffer) {
    let rails = key as usize; //2
    let fences = text.len(); //11
    let mut railfence = format!("RailFence Cipher\nKey: {key}\n");
    railfence.push_str(&"_".repeat(fences * 4 + 1));
    railfence.push('\n');
    let mut fenced_rails: Vec<char> = vec![' '; rails * fences];
    let letters: Vec<char> = text.chars().collect();
    let mut rows = 0;
    let mut down = false;
    for (index, &chr) in letters.iter().enumerate() {
        if rows == rails - 1 || rows == 0 {
            down = !down;
        }
        fenced_rails[(fences * (rows)) + index] = chr;
        if rails != 1 {
            if down {
                rows += 1;
            } else {
                rows -= 1;
            }
        }
    }
    //0,1,2,1,0
    for i in 0..rails {
        railfence.push('|');
        for chr in &fenced_rails[(fences * i)..fences * (i + 1)] {
            let _ = write!(railfence, " {chr} |");
        }
        railfence.push('\n');
    }

    railfence.push_str(&"‾".repeat(fences * 4 + 1));
    Paragraph::new(Text::from(railfence))
        .centered()
        .render(area, buf);
    Block::bordered().render(area, buf);
}

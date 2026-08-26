
use ratatui::{
    Frame, buffer::Buffer, layout::Rect, text::Text, widgets::{Block, Paragraph, Widget},
};
use cifers::{cipher::Cipher,Caeser};

pub struct CaesarView {
    pub shift : i8,
    
}

impl CaesarView {
    pub fn new(shift : i8) -> Self {
        Self {
            shift,
        }
    }
    pub fn draw(&self,frame : &mut Frame, area : Rect) {
        let shift = self.shift;
        let ciphered_alphabet = Caeser::new(shift as i32)
        .encipher("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
        
        let mut ciphered_boxed_alphabet = String::with_capacity(107);
        ciphered_boxed_alphabet.push('|');
        for c in ciphered_alphabet.chars() {
            ciphered_boxed_alphabet.push(' ');
            ciphered_boxed_alphabet.push(c);
            ciphered_boxed_alphabet.push(' ');
            ciphered_boxed_alphabet.push('|');
        }

        let caesar_shifter = format!(
            "Caesar Shifter
            {}
            | A | B | C | D | E | F | G | H | I | J | K | L | M | N | O | P | Q | R | S | T | U | V | W | X | Y | Z |
            | ↓   ↓   ↓   ↓   ↓   ↓   ↓   ↓   ↓   ↓   ↓   ↓   ↓   ↓   ↓   ↓   ↓   ↓   ↓   ↓   ↓   ↓   ↓   ↓   ↓   ↓ |
            {ciphered_boxed_alphabet}
            {}
            Shift: {shift}",

            "_".repeat(105),
            "‾".repeat(105),  
        );

        frame.render_widget(Paragraph::new(Text::from(caesar_shifter))
            .centered()
            .block(Block::bordered()),area)
        
    }
}


pub fn render_caesar(shift: i8, area: Rect, buf: &mut Buffer) {
    let ciphered_alphabet = Caeser::new(shift as i32)
        .encipher("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
        
    let mut ciphered_boxed_alphabet = String::with_capacity(107);
    ciphered_boxed_alphabet.push('|');
    for c in ciphered_alphabet.chars() {
        ciphered_boxed_alphabet.push(' ');
        ciphered_boxed_alphabet.push(c);
        ciphered_boxed_alphabet.push(' ');
        ciphered_boxed_alphabet.push('|');
    }

    let caesar_shifter = format!(
        "Caesar Shifter
        {}
        | A | B | C | D | E | F | G | H | I | J | K | L | M | N | O | P | Q | R | S | T | U | V | W | X | Y | Z |
        | ↓   ↓   ↓   ↓   ↓   ↓   ↓   ↓   ↓   ↓   ↓   ↓   ↓   ↓   ↓   ↓   ↓   ↓   ↓   ↓   ↓   ↓   ↓   ↓   ↓   ↓ |
        {ciphered_boxed_alphabet}
        {}
        Shift: {shift}",

        "_".repeat(105),
        "‾".repeat(105),  
    );

    Paragraph::new(Text::from(caesar_shifter))
        .centered()
        .render(area, buf);
    Block::bordered().render(area, buf);
}

use ratatui::{Frame, layout::Rect};

pub trait CipherView {
    fn draw(&self,frame : &mut Frame,area : Rect);
}

pub struct AppCipher {
    pub cipherview : Box<dyn CipherView>,
    pub scroll : (u16,u16),
}

impl AppCipher {
    pub fn draw(&self,frame : &mut Frame,area : Rect) {
        self.cipherview.draw(frame,area)
    }
}
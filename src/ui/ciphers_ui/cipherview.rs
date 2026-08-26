use ratatui::{Frame, layout::Rect};

pub trait CipherView {
    fn draw(&self,frame : &mut Frame,area : Rect);
}
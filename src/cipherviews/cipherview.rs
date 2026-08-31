use ratatui::{Frame, layout::Rect};

use crate::{CipherName, cipher_stack::CipherType, cipherviews::{affine_ui::AffineView, atbash_ui::AtbashView, caesar_ui::CaesarView, rail_fence_ui::RailfenceView, vigenere_ui::VigenereView}};

pub trait CipherView {
    fn draw(&self,frame : &mut Frame,area : Rect);
}
pub struct AppCipher {
    pub index : usize,
    pub cipher : Box<dyn CipherView>,
    pub scroll : (u16,u16),
}



impl AppCipher {
    pub fn new(index : usize,cipher : &CipherType,text : &str) -> Self {
        let cipher : Box<dyn CipherView> = match cipher {
            CipherType::Caeser(shift) => Box::new(CaesarView::new(*shift)),
            CipherType::Vigenere(code) => Box::new(VigenereView::new(code.clone())),
            CipherType::RailFence(key) => Box::new(RailfenceView::new(text,*key)),
            CipherType::Atbash => Box::new(AtbashView::default()),
            CipherType::Affine(a, b) => Box::new(AffineView::new(*a,*b,text)),
        };

        Self {
            index,
            cipher,
            scroll : (0,0)
        }
    }
    pub fn assign(&mut self,index : usize,cipher : &CipherType,text : &str) {
        match cipher {
            CipherType::Caeser(shift) => self.cipher = Box::new(CaesarView::new(*shift)),
            CipherType::Vigenere(code) => self.cipher  = Box::new(VigenereView::new(code.clone())),
            CipherType::RailFence(key) => self.cipher  = Box::new(RailfenceView::new(text,*key)),
            CipherType::Atbash => self.cipher  = Box::new(AtbashView::default()),
            CipherType::Affine(a, b) => self.cipher  = Box::new(AffineView::new(*a,*b,text)),
        }
        self.index = index;
    }
    pub fn draw(&self,frame : &mut Frame,area : Rect) {
        self.cipher.draw(frame,area)
    }

    
}
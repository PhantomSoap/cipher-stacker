use ratatui::{Frame, layout::Rect};

use crate::{CipherName, cipher_stack::CipherType, cipherviews::{affine_ui::AffineView, atbash_ui::AtbashView, caesar_ui::CaesarView, rail_fence_ui::RailfenceView, vigenere_ui::VigenereView}};

pub trait CipherView {
    fn draw(&self,frame : &mut Frame,area : Rect);
}

pub struct AppCipher {
    pub cipher : CipherType,
    pub scroll : (u16,u16),
}

impl AppCipher {
    
}

impl AppCipher {
    pub fn draw(&self,text : &str,frame : &mut Frame,area : Rect) {
        match &self.cipher {
            CipherType::Caeser(shift) => CaesarView::new(*shift).draw(frame,area),
            CipherType::Vigenere(code) => VigenereView::new(&code).draw(frame,area),
            CipherType::RailFence(key) => RailfenceView::new(text,*key).draw(frame,area),
            CipherType::Atbash => AtbashView::default().draw(frame,area),
            CipherType::Affine(a, b) => AffineView::new(*a,*b,text).draw(frame,area),
        }
    }
}
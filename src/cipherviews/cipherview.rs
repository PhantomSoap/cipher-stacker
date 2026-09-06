use crossterm::event::{KeyCode, KeyEventKind};
use ratatui::{Frame, layout::Rect, widgets::Block};

use crate::{
    CipherType, cipherviews::{
        affine_ui::AffineView, atbash_ui::AtbashView, caesar_ui::CaesarView,
        rail_fence_ui::RailfenceView, vigenere_ui::VigenereView,
    }, components::Component,
};

pub trait CipherView {
    fn draw(&self, frame: &mut Frame, area: Rect,focus : bool,scroll : (u16,u16));
}
pub struct AppCipher {
    pub index: usize,
    pub cipher: Box<dyn CipherView>,
    pub scroll: (u16, u16),
}

impl AppCipher {
    pub fn new(index: usize, cipher: &CipherType, text: &str) -> Self {
        let cipher: Box<dyn CipherView> = match cipher {
            CipherType::Caeser(shift) => Box::new(CaesarView::new(*shift)),
            CipherType::Vigenere(code) => Box::new(VigenereView::new(code.clone())),
            CipherType::RailFence(key) => Box::new(RailfenceView::new(text, *key)),
            CipherType::Atbash => Box::new(AtbashView::default()),
            CipherType::Affine(a, b) => Box::new(AffineView::new(*a, *b, text)),
        };

        Self {
            index,
            cipher,
            scroll: (0, 0),
        }
    }
    pub fn assign(&mut self, index: usize, cipher: &CipherType, text: &str) {
        match cipher {
            CipherType::Caeser(shift) => self.cipher = Box::new(CaesarView::new(*shift)),
            CipherType::Vigenere(code) => self.cipher = Box::new(VigenereView::new(code.clone())),
            CipherType::RailFence(key) => self.cipher = Box::new(RailfenceView::new(text, *key)),
            CipherType::Atbash => self.cipher = Box::new(AtbashView::default()),
            CipherType::Affine(a, b) => self.cipher = Box::new(AffineView::new(*a, *b, text)),
        }
        self.index = index;
    }
    
}

impl Component for AppCipher {
    fn draw(&self, frame: &mut Frame, area: Rect,focus : bool) {
        self.cipher.draw(frame, area,focus,self.scroll);
        
    }
    
    fn handle_key_events(&mut self, key: crossterm::event::KeyEvent) -> Option<crate::Message> {
        if key.kind == KeyEventKind::Release {
            return None
        }
        match key.code {
            KeyCode::Up => {
                self.scroll.0 +=1;
                None
            },
            KeyCode::Down if self.scroll.0 !=0 => {
                self.scroll.0 -=1;
                None
            },
            KeyCode::Left if self.scroll.1 !=0 => {
                self.scroll.1 -=1;
                None
                
            },
            KeyCode::Right => {
                self.scroll.1 +=1;
                None
            },
            KeyCode::Esc => Some(crate::Message::Exit),
            KeyCode::Tab => Some(crate::Message::NextFocus),
            _ => None
        }
    }
    
    fn update(&mut self, _msg: crate::Message) -> Option<crate::Message> {
        None
    }
}

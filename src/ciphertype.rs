use core::fmt;
use std::fmt::Formatter;

use crate::CipherName;

#[derive(Debug, Clone, PartialEq)]
pub enum CipherType {
    Caeser(i8),
    Vigenere(String),
    RailFence(u8),
    Atbash,
    Affine(u8, u8),
}

impl CipherType {
    pub fn instructions(&self) -> String {
        match self {
            CipherType::Caeser(_) => String::from("<- Shift ->"),
            CipherType::Vigenere(_) => String::from("Type a Keyword"),
            CipherType::RailFence(_) => String::from("<Up> Increment Key <Down> Decrement Key"),
            CipherType::Atbash => String::from(""),
            CipherType::Affine(_, _) => {
                String::from("<- Shift -> | <Up> Increment Multiplyer <Down Decrement Multiplyer>")
            }
        }
    }

    pub fn name(&self) -> String {
        match self {
            CipherType::Caeser(_) => String::from("Caesar Cipher"),
            CipherType::Vigenere(_) => String::from("Vigenere Cipher"),
            CipherType::RailFence(_) => String::from("RailFence Cipher"),
            CipherType::Atbash => String::from("Atbash Cipher"),
            CipherType::Affine(_, _) => String::from("Affine Cipher"),
        }
    }

    pub fn into_ciphername(&self) -> CipherName {
        match self {
            CipherType::Caeser(_) => CipherName::Caesar,
            CipherType::Vigenere(_) => CipherName::Vigenere,
            CipherType::RailFence(_) => CipherName::Railfence,
            CipherType::Atbash => CipherName::Atbash,
            CipherType::Affine(_, _) => CipherName::Affine,
        }
    }
}

impl fmt::Display for CipherType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            CipherType::Caeser(shift) => {
                write!(f, "Caeser Cipher; Shift: {}", shift)
            }
            CipherType::Vigenere(code) if !code.is_empty() => {
                write!(f, "Vigenere Cipher; Keyword: {}", code)
            }
            CipherType::Vigenere(_code) => write!(f, "Vigenere Cipher;"),
            CipherType::RailFence(key) => {
                write!(f, "RailFence Cipher; Key: {}", key)
            }
            CipherType::Atbash => write!(f, "Atbash Cipher"),
            CipherType::Affine(a, b) => write!(f, "Affine Cipher; Multiplyer: {}, Shift: {}", a, b),
        }
    }
}
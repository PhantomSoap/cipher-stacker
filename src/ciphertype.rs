use crate::{CIPHER_INSTRUCTIONS, CipherName};

#[derive(Debug, Clone, PartialEq)]
pub enum CipherType {
    Caeser(i8),
    Vigenere(String),
    RailFence(u8),
    Atbash,
    Affine(u8, u8),
}

impl CipherType {
    pub fn instructions(&self) -> &'static str {
        match self {
            CipherType::Caeser(_) => CIPHER_INSTRUCTIONS[0],
            CipherType::Vigenere(_) => "Type a keyword",
            CipherType::RailFence(_) => CIPHER_INSTRUCTIONS[2],
            CipherType::Atbash => "",
            CipherType::Affine(_, _) => CIPHER_INSTRUCTIONS[1],
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

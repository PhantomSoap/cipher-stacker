use crate::CipherType;

#[derive(Debug,Clone, Copy)]
pub enum CipherName {
    Caesar,
    Vigenere,
    Railfence,
    Atbash,
    Affine,
}

impl CipherName {
    pub fn next(&mut self) {
        *self = match self {
            CipherName::Caesar => CipherName::Vigenere,
            CipherName::Vigenere => CipherName::Railfence,
            CipherName::Railfence => CipherName::Atbash,
            CipherName::Atbash => CipherName::Affine,
            CipherName::Affine => CipherName::Caesar,
        }
    }

    pub fn previous(&mut self) {
        *self = match self {
            CipherName::Caesar => CipherName::Affine,
            CipherName::Vigenere => CipherName::Caesar,
            CipherName::Railfence => CipherName::Vigenere,
            CipherName::Atbash => CipherName::Railfence,
            CipherName::Affine => CipherName::Atbash,
        }
    }

    pub fn into_ciphertype(&self) -> CipherType {
        match self {
            CipherName::Caesar => CipherType::Caeser(0),
            CipherName::Vigenere => CipherType::Vigenere("".to_string()),
            CipherName::Railfence => CipherType::RailFence(1),
            CipherName::Atbash => CipherType::Atbash,
            CipherName::Affine => CipherType::Affine(1, 0),
        }
    }
}
use std::{
    fmt::{self, Formatter},
    mem::discriminant,
};

use ciphers::{Affine, Atbash, Caesar, Cipher, RailFence, Vigenere};

#[derive(Debug, Clone)]
pub enum CipherType {
    Caeser(i32),
    Vigenere(String),
    RailFence(i32),
    Atbash,
    Affine(u8, u8),
}

#[derive(Debug)]
pub struct CipherText {
    pub text: String,
    pub ciphers: Vec<CipherType>,
    pub ciphered: String,
}

impl CipherText {
    pub fn new() -> CipherText {
        CipherText {
            text: "EXAMPLETEXT".to_string(),
            ciphers: Vec::new(),
            ciphered: String::from("EXAMPLETEXT"),
        }
    }

    pub fn cipher(&mut self) -> Vec<String> {
        let mut history: Vec<String> = Vec::new();
        history.push(self.text.clone());
        let mut working_cipher = self.text.clone();
        if self.ciphers.is_empty() {
            self.ciphered = working_cipher;
            return history;
        };

        for cipher in &self.ciphers {
            match cipher {
                CipherType::Caeser(shift) => {
                    working_cipher = Caesar::new(*shift as u8)
                        .encipher(&working_cipher)
                        .unwrap()
                        .to_string();

                    history.push(working_cipher.clone());
                }
                CipherType::Vigenere(code) => {
                    if !code.is_empty() {
                        working_cipher = Vigenere::new(code)
                            .encipher(&working_cipher)
                            .unwrap()
                            .to_string();
                    }
                    history.push(working_cipher.clone());
                }
                CipherType::RailFence(key) => {
                    if !(*key < 2 || *key >= working_cipher.len() as i32) {
                        working_cipher = RailFence::new(*key as usize)
                            .encipher(&working_cipher)
                            .unwrap()
                            .to_string();
                        history.push(working_cipher.clone());
                    };
                }
                CipherType::Atbash => {
                    working_cipher = Atbash::new().encipher(&working_cipher).unwrap().to_string();
                    history.push(working_cipher.clone());
                }
                CipherType::Affine(a, b) => {
                    working_cipher = Affine::new(*a as i32, *b as i32)
                        .encipher(&working_cipher)
                        .unwrap()
                        .to_string();
                    history.push(working_cipher.clone());
                }
            };
        }
        self.ciphered = working_cipher;
        history
    }
}

impl CipherText {
    pub fn next(&self, cipher: &CipherType) -> CipherType {
        let disc = discriminant(&cipher.next());
        if let Some(index) = self.ciphers.iter().rposition(|c| discriminant(c) == disc) {
            self.ciphers.get(index).unwrap().clone()
        } else {
            cipher.next()
        }
    }
}
impl CipherType {
    pub fn next(&self) -> CipherType {
        match self {
            CipherType::Caeser(_) => CipherType::Vigenere("".to_string()),
            CipherType::Vigenere(_) => CipherType::RailFence(2),
            CipherType::RailFence(_) => CipherType::Atbash,
            CipherType::Atbash => CipherType::Affine(1, 0),
            CipherType::Affine(_, _) => CipherType::Caeser(0),
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

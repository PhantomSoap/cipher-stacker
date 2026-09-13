use crate::{CipherName, CipherType, INSTRUCTIONS, Message, components::Component};
use cifers::{Affine, Caeser, Cipher, Railfence, Vigenere};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{List, ListItem, ListState, Paragraph},
};
use ratatui_themekit::{ThemeData, ThemeExt};
#[derive(Debug, Clone, Copy)]
pub enum CipherEdit {
    PushChar(char),
    Popchar,
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]

pub enum StackState {
    Main,
    ShowHistory,
}
#[derive(Debug)]
pub enum Task {
    Editing,
    Adding,
}

#[derive(Debug)]
pub struct CipherStack {
    pub ciphers: Vec<CipherType>,
    pub selected: Option<usize>,
    pub cipher_to_add: CipherName,
    pub state: StackState,
    pub history: Vec<String>,
    pub task: Task,
}

impl CipherStack {
    pub fn new() -> CipherStack {
        CipherStack {
            ciphers: Vec::new(),
            selected: None,
            cipher_to_add: CipherName::Caesar,
            state: StackState::Main,
            history: Vec::new(),
            task: Task::Adding,
        }
    }

    pub fn stack_cipher(&mut self, text: &str, ciphertext: &mut String) {
        let mut history: Vec<String> = Vec::new();
        let mut working_cipher = text.to_string();
        if self.ciphers.is_empty() {
            *ciphertext = working_cipher;
            self.history = history;
            return;
        };

        for cipher in &self.ciphers {
            match cipher {
                CipherType::Caeser(shift) => {
                    working_cipher = Caeser::new(*shift as i32).encipher(&working_cipher);

                    history.push(working_cipher.to_string());
                }
                CipherType::Vigenere(code) => {
                    if !code.is_empty() {
                        working_cipher = Vigenere::new(code.clone()).encipher(&working_cipher)
                    }
                    history.push(working_cipher.clone());
                }
                CipherType::RailFence(key) => {
                    working_cipher = Railfence::new(*key as u8 % working_cipher.len() as u8)
                        .encipher(&working_cipher);

                    history.push(working_cipher.clone());
                }
                CipherType::Atbash => {
                    working_cipher = Affine::atbash().encipher(&working_cipher);
                    history.push(working_cipher.clone());
                }
                CipherType::Affine(a, b) => {
                    working_cipher = Affine::new(*a as i32, *b as i32).encipher(&working_cipher);
                    history.push(working_cipher.clone());
                }
            };
        }
        *ciphertext = working_cipher;
        self.history = history;
    }
}

impl Component for CipherStack {
    fn draw(&self, frame: &mut Frame, area: Rect, focus: bool, t: ThemeData) {
        let split = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(8),
                Constraint::Length(8),
            ])
            .split(area);

        let panel = match self.task {
            Task::Editing => Paragraph::new(Text::from(Line::from(vec![
                Span::raw("| "),
                Span::styled(
                    format!("{:?}", self.ciphers[self.selected.unwrap()]),
                    Style::default().fg(Color::Black).bg(Color::White),
                ),
                Span::raw(" |"),
            ])))
            .block(t.block("Edit Cipher").focused(focus).build()),
            Task::Adding => Paragraph::new(Text::from(Line::from(vec![
                Span::raw("| "),
                Span::styled(
                    format!("{:?}", &self.cipher_to_add),
                    Style::default().fg(Color::Black).bg(Color::White),
                ),
                Span::raw(" |"),
                Span::styled("<+>", Color::Blue),
                Span::raw(" to add |"),
            ])))
            .block(t.block("Add Cipher").focused(focus).build()),
        };
        frame.render_widget(panel, split[0]);

        let list = match self.state {
            StackState::Main => List::new(
                self.ciphers
                    .iter()
                    .map(|cipher| ListItem::from(format!("{:?}", cipher))),
            )
            .highlight_style(Color::LightRed)
            .block(t.block("Ciphers").focused(focus).build()),
            StackState::ShowHistory => {
                let mut history_text: Vec<ListItem> = Vec::new();

                for (index, cipher) in self.ciphers.iter().enumerate() {
                    if let Some(hist_item) = self.history.get(index) {
                        history_text.push(ListItem::from(Text::from(format!(
                            "{cipher:?} -> {hist_item}"
                        ))));
                    }
                }
                List::new(history_text)
                    .highlight_style(Color::LightRed)
                    .block(t.block("History").focused(focus).build())
            }
        };
        frame.render_stateful_widget(
            list,
            split[1],
            &mut ListState::default().with_selected(self.selected),
        );

        if let Task::Editing = self.task {
            frame.render_widget(
                Paragraph::new(self.ciphers[self.selected.unwrap()].instructions())
                    .block(t.block("Instructions").focused(focus).build()),
                split[2],
            )
        } else {
            frame.render_widget(
                Paragraph::new(INSTRUCTIONS[0])
                    .block(t.block("Instructions").focused(focus).build()),
                split[2],
            )
        }
    }

    fn handle_key_events(&mut self, key: KeyEvent) -> Option<Message> {
        if let KeyEventKind::Release = key.kind {
            return None;
        }
        if let Task::Editing = self.task {
            match key.code {
                KeyCode::Esc => Some(Message::Exit),
                KeyCode::Char(chr) => Some(Message::EditCipher(CipherEdit::PushChar(chr))),
                KeyCode::Backspace => Some(Message::EditCipher(CipherEdit::Popchar)),
                KeyCode::Up => Some(Message::EditCipher(CipherEdit::Up)),
                KeyCode::Down => Some(Message::EditCipher(CipherEdit::Down)),
                KeyCode::Left => Some(Message::EditCipher(CipherEdit::Left)),
                KeyCode::Right => Some(Message::EditCipher(CipherEdit::Right)),
                KeyCode::Tab => Some(Message::NextFocus),
                KeyCode::Enter => {
                    self.task = Task::Adding;
                    None
                }
                _ => None,
            }
        } else if let StackState::Main = self.state {
            match key.code {
                KeyCode::Esc => Some(Message::Exit),
                KeyCode::Tab => Some(Message::NextFocus),
                KeyCode::Char('-') if let Some(index) = self.selected => {
                    let _removed = self.ciphers.remove(index);
                    self.selected = if self.ciphers.len() != 0 {
                        Some(self.ciphers.len() - 1)
                    } else {
                        None
                    };
                    None
                }
                KeyCode::Char('-') => {
                    if let Some(_removed) = self.ciphers.pop() {
                        self.selected = if self.ciphers.len() != 0 {
                            Some(self.ciphers.len() - 1)
                        } else {
                            None
                        };
                    }
                    None
                }
                KeyCode::Char('+') if let Some(index) = self.selected => {
                    self.ciphers
                        .insert(index, self.cipher_to_add.into_ciphertype());
                    self.selected = Some(index);
                    None
                }
                KeyCode::Char('+') => {
                    self.ciphers.push(self.cipher_to_add.into_ciphertype());
                    self.selected = Some(self.ciphers.len() - 1);
                    None
                }
                KeyCode::Up if let Some(index) = &mut self.selected => {
                    if *index != 0 {
                        *index -= 1;
                    }
                    None
                }
                KeyCode::Down if let Some(index) = &mut self.selected => {
                    if *index != self.ciphers.len() - 1 {
                        *index += 1;
                    }
                    None
                }
                KeyCode::Right => {
                    self.cipher_to_add.next();
                    None
                }
                KeyCode::Left => {
                    self.cipher_to_add.previous();
                    None
                }
                KeyCode::Enter if let Some(_) = self.selected => {
                    self.task = Task::Editing;
                    None
                }
                KeyCode::Char(' ') => {
                    self.state = StackState::ShowHistory;
                    None
                }

                _ => None,
            }
        } else if let StackState::ShowHistory = self.state {
            match key.code {
                KeyCode::Esc => Some(Message::Exit),
                KeyCode::Tab => Some(Message::NextFocus),
                KeyCode::Char('-') if let Some(index) = self.selected => {
                    let _removed = self.ciphers.remove(index);
                    self.selected = if self.ciphers.len() != 0 {
                        Some(self.ciphers.len() - 1)
                    } else {
                        None
                    };
                    None
                }
                KeyCode::Char('-') => {
                    if let Some(_removed) = self.ciphers.pop() {
                        self.selected = if self.ciphers.len() != 0 {
                            Some(self.ciphers.len() - 1)
                        } else {
                            None
                        };
                    }
                    None
                }
                KeyCode::Char('+') if let Some(index) = self.selected => {
                    self.ciphers
                        .insert(index, self.cipher_to_add.into_ciphertype());
                    self.selected = Some(index);
                    None
                }
                KeyCode::Char('+') => {
                    self.ciphers.push(self.cipher_to_add.into_ciphertype());
                    self.selected = Some(self.ciphers.len() - 1);
                    None
                }
                KeyCode::Up if let Some(index) = &mut self.selected => {
                    if *index != 0 {
                        *index -= 1;
                    }
                    None
                }
                KeyCode::Down if let Some(index) = &mut self.selected => {
                    if *index != self.ciphers.len() - 1 {
                        *index += 1;
                    }
                    None
                }
                KeyCode::Right => {
                    self.cipher_to_add.next();
                    None
                }
                KeyCode::Left => {
                    self.cipher_to_add.previous();
                    None
                }
                KeyCode::Enter if let Some(_) = self.selected => {
                    self.task = Task::Editing;
                    None
                }
                KeyCode::Char(' ') => {
                    self.state = StackState::Main;
                    None
                }
                _ => None,
            }
        } else {
            None
        }
    }

    fn update(&mut self, msg: Message) -> Option<Message> {
        match msg {
            Message::EditCipher(edit) => {
                if let Some(index) = self.selected {
                    match edit {
                        CipherEdit::PushChar(chr)
                            if let CipherType::Vigenere(code) = &mut self.ciphers[index] =>
                        {
                            code.push(chr.to_ascii_uppercase());
                        }
                        CipherEdit::Popchar
                            if let CipherType::Vigenere(code) = &mut self.ciphers[index] =>
                        {
                            code.pop();
                        }
                        CipherEdit::Up => match &mut self.ciphers[index] {
                            CipherType::Affine(a, _b) => {
                                let mut shift = *a;
                                while !(*a == 26) && !(shift == 26) {
                                    if !((shift + 1) % 2 == 0) && !((shift + 1) % 13 == 0) {
                                        *a = shift + 1;
                                        break;
                                    } else {
                                        shift += 1;
                                    }
                                }
                            }
                            CipherType::RailFence(key) => *key += 1,
                            _ => {}
                        },
                        CipherEdit::Down => match &mut self.ciphers[index] {
                            CipherType::Affine(a, _b) => {
                                let mut shift = *a;
                                while !(*a == 0) && !(shift == 0) {
                                    if !((shift - 1) % 2 == 0) && !((shift - 1) % 13 == 0) {
                                        *a = shift - 1;
                                        break;
                                    } else {
                                        shift -= 1;
                                    }
                                }
                            }
                            CipherType::RailFence(key) if *key != 1 => *key -= 1,
                            _ => {}
                        },
                        CipherEdit::Left => match &mut self.ciphers[index] {
                            CipherType::Caeser(shift) => {
                                *shift = ((*shift - 1) % 26 + 26) % 26;
                            }
                            CipherType::Affine(_a, b) if !(*b == 0) => {
                                *b -= 1;
                            }
                            _ => {}
                        },
                        CipherEdit::Right => match &mut self.ciphers[index] {
                            CipherType::Caeser(shift) => {
                                *shift = ((*shift + 1) % 26 + 26) % 26;
                            }
                            CipherType::Affine(_a, b) if !(*b == 25) => {
                                *b += 1;
                            }

                            _ => {}
                        },
                        _ => {}
                    }
                }
                None
            }
            _ => None,
        }
    }
}

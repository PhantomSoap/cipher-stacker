use ratatui::layout::Rect;

pub struct UiArea {
    title : Rect,
    cipher_visualization : Rect,
    history : Rect,
    plaintext : Rect,
    ciphertext : Rect,
    instructions : Rect,
    cipher_list : Rect,
}

impl UiArea {
    pub fn new(area : Rect) -> Self{
        todo!()
    }
}
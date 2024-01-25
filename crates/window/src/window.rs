use crate::cursor::Cursor;

#[derive(Debug)]
pub struct Window {
    pub title: String,
    pub cursor: Cursor,
    pub visible: bool,
    pub transparent: bool,
    pub focused: bool,
    pub resizeable: bool,
}

impl Default for Window {
    fn default() -> Self {
        Self {
            title: "App".to_owned(),
            cursor: Default::default(),
            visible: true,
            transparent: false,
            focused: true,
            resizeable: true,
        }
    }
}
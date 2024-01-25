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
            cursor: Cursor::default(),
            visible: true,
            transparent: false,
            focused: true,
            resizeable: true,
        }
    }
}

#[derive(Debug)]
pub struct WindowBuilder {
    title: String,
    cursor: Cursor,
    visible: bool,
    transparent: bool,
    focused: bool,
    resizeable: bool,
}

impl WindowBuilder {
    pub fn with_title(self, title: String) -> Self {
        Self {
            title,
            ..self
        }
    }

    pub fn with_cursor(self, cursor: Cursor) -> Self {
        Self {
            cursor,
            ..self
        }
    }

    pub fn set_visible(self, visible: bool) -> Self {
        Self {
            visible,
            ..self
        }
    }

    pub fn set_transparent(self, transparent: bool) -> Self {
        Self {
            transparent,
            ..self
        }
    }

    pub fn set_focused(self, focused: bool) -> Self {
        Self {
            focused,
            ..self
        }
    }

    pub fn set_resizeable(self, resizeable: bool) -> Self {
        Self {
            resizeable,
            ..self
        }
    }

    pub fn build(self) -> Window {
        Window {
            title: self.title,
            cursor: self.cursor,
            visible: self.visible,
            transparent: self.transparent,
            focused: self.focused,
            resizeable: self.resizeable,
        }
    }
}
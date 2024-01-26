use winit::window::CursorIcon;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Cursor {
    pub cursor_icon: CursorIcon,
    pub visible: bool,
    pub grab_mode: CursorGrabMode,
}

impl Default for Cursor {
    fn default() -> Self {
        Self {
            cursor_icon: CursorIcon::Default,
            visible: true,
            grab_mode: CursorGrabMode::None,
        }
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CursorGrabMode {
    #[default]
    None,
    Confined,
    Locked,
}
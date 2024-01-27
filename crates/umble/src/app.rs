use std::cell::RefCell;
use std::collections::HashMap;
use crate::window;
use crate::window::Window;

pub struct App {
    pub(crate) windows: RefCell<HashMap<window::Id, Window>>,
}
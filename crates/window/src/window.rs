use std::collections::HashMap;
// use winit::error::EventLoopError;
// use winit::event::{ElementState, Event, KeyEvent, WindowEvent};
// use winit::event_loop::{ControlFlow, EventLoop, EventLoopBuilder, EventLoopWindowTarget};
// use winit::keyboard::{KeyCode, PhysicalKey};
// use winit::window::WindowId;
// use crate::cursor::Cursor;

// #[derive(Debug)]
// pub struct Window {
//     pub id: WindowId,
//     pub title: String,
//     pub cursor: Cursor,
//     pub visible: bool,
//     pub transparent: bool,
//     pub focused: bool,
//     pub resizeable: bool,
//     pub(crate) internal: winit::window::Window,
// }
//
// impl Default for Window {
//     fn default() -> Self {
//         Self {
//             title: "App".to_owned(),
//             cursor: Cursor::default(),
//             visible: true,
//             transparent: false,
//             focused: true,
//             resizeable: true,
//             ..Default::default()
//         }
//     }
// }

// impl Window {
//     pub fn new(handler: fn(&Window, Event<()>, &EventLoopWindowTarget<()>), title: String) -> Self {
//         let event_loop = EventLoopBuilder::new().build().unwrap();
//
//         let internal = winit::window::WindowBuilder::new()
//             .with_title(title)
//             .with_transparent(false)
//             .with_resizable(true)
//             .build(&event_loop)
//             .expect("Failed to build window.");
//
//         let window = Window {
//             id: internal.id(),
//             title: internal.title().to_owned(),
//             cursor: Cursor::default(),
//             visible: internal.is_visible().unwrap(),
//             transparent: false,
//             focused: internal.has_focus(),
//             resizeable: internal.is_resizable(),
//             internal,
//         };
//
//         let event_handler = |event: Event<()>, event_loop: &EventLoopWindowTarget<()>| {
//             handler(&window, event, event_loop);
//         };
//
//         event_loop.set_control_flow(ControlFlow::Poll);
//         event_loop.run(&event_handler).expect("Failed to run event loop.");
//
//         window
//     }
// }

// #[derive(Debug)]
// pub struct WindowBuilder {
//     title: String,
//     cursor: Cursor,
//     visible: bool,
//     transparent: bool,
//     focused: bool,
//     resizeable: bool,
// }
//
// impl WindowBuilder {
//     pub fn with_title(self, title: String) -> Self {
//         Self {
//             title,
//             ..self
//         }
//     }
//
//     pub fn with_cursor(self, cursor: Cursor) -> Self {
//         Self {
//             cursor,
//             ..self
//         }
//     }
//
//     pub fn with_visible(self, visible: bool) -> Self {
//         Self {
//             visible,
//             ..self
//         }
//     }
//
//     pub fn with_transparent(self, transparent: bool) -> Self {
//         Self {
//             transparent,
//             ..self
//         }
//     }
//
//     pub fn with_focused(self, focused: bool) -> Self {
//         Self {
//             focused,
//             ..self
//         }
//     }
//
//     pub fn with_resizeable(self, resizeable: bool) -> Self {
//         Self {
//             resizeable,
//             ..self
//         }
//     }
//
//     pub fn build(self) -> Window {
//         Window {
//             title: self.title,
//             cursor: self.cursor,
//             visible: self.visible,
//             transparent: self.transparent,
//             focused: self.focused,
//             resizeable: self.resizeable,
//             ..Default::default()
//         }
//     }
// }

// #[derive(Debug, Default)]
// pub struct WindowManager {
//     focused_window: Option<WindowId>,
//     windows: HashMap<WindowId, Window>,
//     // event_handler: fn(&Window, Event<()>, &EventLoopWindowTarget<()>),
// }
//
// impl WindowManager {
//     pub fn add_window(&mut self, window: Window) {
//         self.windows.insert(window.id, window);
//     }
//
//     pub fn remove_window(&mut self, window_id: WindowId) {
//         self.windows.remove(&window_id);
//     }
//
//     pub fn focus_window(&mut self, window_id: WindowId) {
//         self.focused_window = Some(window_id);
//     }
//
//     pub fn un_focus_window(&mut self) {
//         self.focused_window = None;
//     }
//
//     pub fn get_focused_window(&self) -> Option<&Window> {
//         self.focused_window.map(|window_id| self.windows.get(&window_id).unwrap())
//     }
//
//     pub fn get_window(&self, window_id: &WindowId) -> Option<&Window> {
//         self.windows.get(window_id)
//     }
// }
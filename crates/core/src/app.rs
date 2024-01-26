use window::window::WindowManager;
use window::winit::{ElementState, Event, EventLoopWindowTarget, KeyCode, KeyEvent, PhysicalKey, WindowEvent, WindowId};
use crate::window::Window;

pub const APP_NAME: &str = "R&nder";

#[derive(Debug)]
pub struct App {
    pub window_id: WindowId,
    pub windows: WindowManager,
}

impl App {
    pub fn init() -> Self {
        let event_handler = move |window: &Window, event: Event<()>, event_loop: &EventLoopWindowTarget<()>| {
            match event {
                Event::Resumed => {
                    println!("Window resumed");
                }
                Event::WindowEvent {
                    ref event,
                    window_id,
                } => {
                    if window_id == window.id {
                        match event {
                            WindowEvent::CloseRequested => {
                                on_close(event_loop);
                            }
                            WindowEvent::KeyboardInput {
                                event: KeyEvent {
                                    state: ElementState::Pressed,
                                    physical_key: PhysicalKey::Code(KeyCode::KeyQ),
                                    ..
                                },
                                ..
                            } => {
                                println!("Space was pressed");
                            }
                            _ => (),
                        }
                    }
                }
                _ => (),
            }
        };

        let window_name = format!("{} - {}", APP_NAME, "Engine");
        let window = Window::new(event_handler, window_name);
        let window_id = window.id;

        let mut windows = WindowManager::default();
        windows.add_window(window);

        Self {
            window_id,
            windows,
        }
    }

    pub fn get_current_window(&self) -> Option<&Window> {
        self.windows.get_window(&self.window_id)
    }
}

fn on_close(event_loop: &EventLoopWindowTarget<()>) {
    println!("Window close requested");
    event_loop.exit();
}
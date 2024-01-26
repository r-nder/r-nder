use nannou::prelude::*;

pub const APP_NAME: &str = "R&nder";
pub const APP_WIDTH: u32 = 1366;
pub const APP_HEIGHT: u32 = 768;

pub struct MainApp {
    pub window_id: WindowId,
}

impl MainApp {
    pub fn init() {
        nannou::app(Self::model).run();
    }

    fn model(app: &App) -> MainApp {
        let window_id = app
            .new_window()
            .size(APP_WIDTH, APP_HEIGHT)
            .title(APP_NAME)
            .view(Self::view)
            .event(Self::event)
            .build()
            .expect("Failed to create main window.");

        MainApp {
            window_id,
        }
    }

    fn event(app: &App, model: &mut MainApp, event: WindowEvent) {
        println!("Event: {:?}", event);
    }

    fn view(app: &App, model: &MainApp, frame: Frame) {
        frame.clear(CORNFLOWERBLUE);
    }
}
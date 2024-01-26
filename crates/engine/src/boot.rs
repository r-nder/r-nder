use nannou::prelude::*;
use nannou::winit;
use nannou_egui::Egui;
use crate::{Engine, Window};
use crate::constant::{ENGINE_APP_HEIGHT, ENGINE_APP_NAME, ENGINE_APP_WIDTH};

pub fn init() {
    println!("Hello, World!");

    nannou::app(configure_app)
        .update(update)
        .loop_mode(LoopMode::RefreshSync)
        .run();
}

fn configure_app(app: &App) -> Engine {
    let window_id = app
        .new_window()
        .title(ENGINE_APP_NAME)
        .size(ENGINE_APP_WIDTH, ENGINE_APP_HEIGHT)
        .event(|app, model: &mut Engine, event| {
            model.on_window_event(app, event);
        })
        .view(|app, model: &Engine, frame| {
            model.view(app, frame);
        })
        .raw_event(|app, model: &mut Engine, event: &winit::event::WindowEvent| {
            model.raw_window_event(app, event);
        })
        .build()
        .unwrap();

    let window = app.window(window_id).unwrap();
    let egui = Egui::from_window(&window);

    Engine::init(window_id, egui)
}

fn update(app: &App, model: &mut Engine, update: Update) {
    model.update(app, update);
}
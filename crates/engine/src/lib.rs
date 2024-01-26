use nannou::{App, Frame, winit};
use nannou::color::BLACK;
use nannou::event::{Update, WindowEvent};
use nannou::prelude::{WindowId};
use nannou_egui::{Egui, egui};
use crate::models::engine_settings::EngineSettings;

pub mod boot;
mod constant;
mod models;

pub struct Engine {
    pub window_id: WindowId,
    pub egui: Egui,
    pub engine_settings: EngineSettings,
}

impl Engine {
    pub fn init(window_id: WindowId, egui: Egui) -> Self {
        Self {
            window_id,
            egui,
            engine_settings: EngineSettings::default(),
        }
    }
}

impl Window for Engine {
    fn view(&self, app: &App, frame: Frame) {
        Self::from_egui_view(app, &self.egui, frame);
    }

    fn update(&mut self, _app: &App, update: Update) {
        let Engine {
            ref mut egui,
            ref mut engine_settings,
            ..
        } = self;

        egui.set_elapsed_time(update.since_start);
        let ctx = egui.begin_frame();

        egui::Window::new("Engine Settings").show(&ctx, |ui| {
            let mut changed = false;

            changed |= ui.add(egui::Slider::new(&mut engine_settings.age, 0.0..=20.0).text("Age"))
                .changed();

            if changed {
                println!("Age changed to {}", engine_settings.age);
            }
        });
    }

    fn raw_window_event(&mut self, _app: &App, event: &winit::event::WindowEvent) {
        Self::handle_raw_event_from_egui(_app, self, event);
    }
}

pub trait EguiRawWindowEvent {
    fn handle_raw_event(_app: &App, event: &winit::event::WindowEvent);
}

pub trait Window {
    fn on_window_event(&mut self, _app: &App, _event: WindowEvent) {}
    fn view(&self, _app: &App, _frame: Frame) {}

    fn from_egui_view(app: &App, egui: &Egui, frame: Frame) {
        let draw = app.draw();
        draw.background().color(BLACK);
        draw.to_frame(app, &frame).unwrap();
        egui.draw_to_frame(&frame).unwrap();
    }

    fn update(&mut self, _app: &App, _update: Update) {}
    fn raw_window_event(&mut self, _app: &App, _event: &winit::event::WindowEvent) {}

    fn handle_raw_event_from_egui(_app: &App, model: &mut Engine, event: &winit::event::WindowEvent) {
        model.egui.handle_raw_event(event);
    }
}
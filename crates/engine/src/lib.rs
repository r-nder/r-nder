use egui_dock::egui::{Ui, WidgetText};
use nannou::prelude::*;
use nannou::winit;
use nannou_egui::{Egui, egui};
use nannou_egui::egui::Color32;
use crate::editor::Editor;
use crate::models::engine_settings::EngineSettings;

pub mod boot;
mod constant;
mod models;
mod editor;

pub struct Engine {
    pub window_id: WindowId,
    pub egui: Egui,
    pub engine_settings: EngineSettings,
    pub editor: Editor,
}

impl Engine {
    pub fn init(window_id: WindowId, egui: Egui) -> Self {
        Self {
            window_id,
            egui,
            engine_settings: EngineSettings::default(),
            editor: Editor::default(),
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

        self.editor.update(&ctx);
    }

    fn raw_window_event(&mut self, _app: &App, event: &winit::event::WindowEvent) {
        Self::handle_raw_event_from_egui(_app, self, event);
    }
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
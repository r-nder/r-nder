use egui_dock::{DockArea, DockState, egui, NodeIndex, Style};
use egui_dock::egui::{Ui, WidgetText};

pub struct Editor {
    pub tree: DockState<String>,
}

impl Default for Editor {
    fn default() -> Self {
        let layout = Self::default_layout(&egui::Context::default());
        Self { tree: layout }
    }
}

impl Editor {
    pub fn update(&mut self, ctx: &egui::Context) {
        let tree = &mut self.tree;

        DockArea::new(tree)
            .style(Style::from_egui(ctx.style().as_ref()))
            .show(ctx, &mut TabViewer {});
    }

    pub fn default_layout(ctx: &egui::Context) -> DockState<String> {
        let mut tree = DockState::new(vec!["Game".to_owned()]);

        // You can modify the tree before constructing the dock
        let [a, b] =
            tree.main_surface_mut()
                .split_left(NodeIndex::root(), 0.3, vec!["Hierarchy".to_owned()]);
        let [_, _] = tree
            .main_surface_mut()
            .split_below(a, 0.7, vec!["Console".to_owned()]);
        let [_, _] = tree
            .main_surface_mut()
            .split_below(b, 0.5, vec!["Inspector".to_owned()]);

        tree
    }
}

struct TabViewer;

impl egui_dock::TabViewer for TabViewer {
    type Tab = String;

    fn title(&mut self, tab: &mut Self::Tab) -> WidgetText {
        (&*tab).into()
    }

    fn ui(&mut self, ui: &mut Ui, tab: &mut Self::Tab) {
        ui.label(format!("Content of {tab}"));
    }
}

// #[derive(Debug)]
// pub struct SceneEditor;
//
// impl Draw for SceneEditor {
//     fn draw(&mut self, _app: &App, _frame: Frame) {
//         println!("SceneEditor::draw");
//
//
//     }
// }
//
// pub trait Draw {
//     fn draw(&mut self, _app: &App, _frame: Frame);
// }
use std::env;
use std::fmt::format;
use winit::dpi::LogicalSize;
use winit::window::Fullscreen;

pub const DEFAULT_DIMENSIONS: LogicalSize<geom::scalar::Default> = LogicalSize {
    width: 1024.0,
    height: 768.0,
};

pub struct Builder {
    window: winit::window::WindowBuilder,
    title_was_set: bool,
}

#[derive(Debug)]
pub struct Window {
    pub(crate) window: winit::window::Window,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            window: winit::window::WindowBuilder::new(),
        }
    }

    pub fn window(mut self, window: winit::window::WindowBuilder) -> Self {
        self.window = window;
        self
    }

    pub fn build(self) {
        let Builder {
            mut window,
            title_was_set,
        } = self;

        if !title_was_set {
            if let Ok(exe_path) = env::current_exe() {
                if let Some(os_str) = exe_path.file_stem() {
                    if let Some(exe_name) = os_str.to_str() {
                        let title = format!("r&nder - {}", exe_name);
                        window = window.with_title(title);
                    }
                }
            }
        }

        let initial_window_size = window
            .window_attributes()
            .inner_size.or_else(|| {
            window.window_attributes()
                .fullscreen().as_ref()
                .and_then(|fullscreen| match fullscreen {
                    Fullscreen::Exclusive(video_mode) => {
                        let monitor = video_mode.monitor();
                        Some(
                            video_mode
                                .size()
                                .to_logical::<f32>(monitor.scale_factor())
                                .into()
                        )
                    }
                    Fullscreen::Borderless(monitor) => monitor.as_ref().map(|monitor| {
                        monitor
                            .size()
                            .to_logical::<f32>(monitor.scale_factor())
                            .into()
                    })
                })
        })
            .unwrap_or_else(|| {
                let mut dimension = DEFAULT_DIMENSIONS;

                if let Some(min) = window.window_attributes().min_inner_size {
                    match min {
                        winit::dpi::Size::Logical(min) => {
                            dimension
                        }
                    }
                }
            });
    }
}
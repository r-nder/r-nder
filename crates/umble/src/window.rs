pub use winit::window::WindowId as Id;

/// A nannou window.
///
/// The **Window** acts as a wrapper around the `winit::window::Window` and the `wgpu::Surface`
/// types.
#[derive(Debug)]
pub struct Window {
    // pub(crate) window: winit::window::Window,
    // pub(crate) surface: wgpu::Surface,
    // pub(crate) surface_conf: wgpu::SurfaceConfiguration,
    // pub(crate) device_queue_pair: Arc<wgpu::DeviceQueuePair>,
    // msaa_samples: u32,
    // pub(crate) frame_data: Option<FrameData>,
    // pub(crate) frame_count: u64,
    // pub(crate) user_functions: UserFunctions,
    pub(crate) tracked_state: TrackedState,
    // pub(crate) is_invalidated: bool, // Whether framebuffer must be cleared
    // pub(crate) clear_color: wgpu::Color,
}

// Track and store some information about the window in order to avoid making repeated internal
// queries to the platform-specific API. This is beneficial in some cases where queries to the
// platform-specific API can be very slow (e.g. macOS cocoa).
#[derive(Debug)]
pub(crate) struct TrackedState {
    // Updated on `ScaleFactorChanged`.
    pub(crate) scale_factor: f64,
    // Updated on `Resized`.
    pub(crate) physical_size: winit::dpi::PhysicalSize<u32>,
}
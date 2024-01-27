use std::path::PathBuf;
use std::time::Duration;
use winit::event::{MouseButton, MouseScrollDelta, TouchPhase};
use winit::keyboard::Key;
use umble_core::math;
use umble_core::math::point::Point2;
use umble_core::math::vector::Vector2;
use crate::window;

/// Event types that are compatible with the nannou app loop.
pub trait LoopEvent: 'static + From<Update> {
    /// Produce a loop event from the given winit event.
    fn from_winit_event<'a, T>(_: &winit::event::Event<'a, T>, _: &App) -> Option<Self>;
}

/// Update event, emitted on each pass of an application loop.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Update {
    /// The duration since the last update was emitted.
    ///
    /// The first update's delta will be the time since the given `model` function returned.
    pub since_last: Duration,
    /// The duration since the start of the app loop.
    ///
    /// Specifically, this is the duration of time since the given `model` function returned.
    pub since_start: Duration,
}

/// The default application **Event** type.
#[derive(Debug)]
pub enum Event {
    /// A window-specific event has occurred for the window with the given Id.
    ///
    /// The event is available as a **WindowEvent**, a more user-friendly form of
    /// **winit::event::WindowEvent**. Once
    /// [winit#1387](https://github.com/rust-windowing/winit/issues/1387) is fixed, its "raw" form
    /// will also be available.
    WindowEvent {
        id: window::Id,
        simple: Option<WindowEvent>,
        // TODO: Re-add this when winit#1387 is resolved.
        // raw: winit::event::WindowEvent,
    },

    /// A device-specific event has occurred for the device with the given Id.
    DeviceEvent(winit::event::DeviceId, winit::event::DeviceEvent),

    /// A timed update alongside the duration since the last update was emitted.
    ///
    /// The first update's delta will be the time since the `model` function returned.
    Update(Update),

    /// The application has been suspended or resumed.
    Suspended,
    /// The application has been awakened.
    Resumed,
}

/// The event associated with a touch at a single point.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TouchEvent {
    /// The unique ID associated with this touch, e.g. useful for distinguishing between fingers.
    pub id: u64,
    /// The state of the touch.
    pub phase: TouchPhase,
    /// The position of the touch.
    pub position: Point2,
}

/// Pressure on a touch pad.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TouchpadPressure {
    /// The unique ID associated with the device that emitted this event.
    pub device_id: winit::event::DeviceId,
    /// The amount of pressure applied.
    pub pressure: f32,
    /// Integer representing the click level.
    pub stage: i64,
}

/// Motion along some axis of a device e.g. joystick or gamepad.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AxisMotion {
    /// The unique ID of the device that emitted this event.
    pub device_id: winit::event::DeviceId,
    /// The axis on which motion occurred.
    pub axis: winit::event::AxisId,
    /// The motion value.
    pub value: math::scalar::Default,
}

/// A simplified version of winit's `WindowEvent` type to make it easier to get started.
///
/// All co-ordinates and dimensions are DPI-agnostic scalar values.
///
/// Co-ordinates for each window are as follows:
///
/// - `(0.0, 0.0)` is the centre of the window.
/// - positive `x` points to the right, negative `x` points to the left.
/// - positive `y` points upwards, negative `y` points downwards.
/// - positive `z` points into the screen, negative `z` points out of the screen.
#[derive(Clone, Debug, PartialEq)]
pub enum WindowEvent {
    /// The window has been moved to a new position.
    Moved(Point2),

    /// The given keyboard key was pressed.
    KeyPressed(Key),

    /// The given keyboard key was released.
    KeyReleased(Key),

    /// Character input received event.
    ReceivedCharacter(char),

    /// The mouse moved to the given x, y position.
    MouseMoved(Point2),

    /// The given mouse button was pressed.
    MousePressed(MouseButton),

    /// The given mouse button was released.
    MouseReleased(MouseButton),

    /// The mouse entered the window.
    MouseEntered,

    /// The mouse exited the window.
    MouseExited,

    /// A mouse wheel movement or touchpad scroll occurred.
    MouseWheel(MouseScrollDelta, TouchPhase),

    /// The window was resized to the given dimensions (in DPI-agnostic points, not pixels).
    Resized(Vector2),

    /// A file at the given path was hovered over the window.
    HoveredFile(PathBuf),

    /// A file at the given path was dropped onto the window.
    DroppedFile(PathBuf),

    /// A file at the given path that was hovered over the window was cancelled.
    HoveredFileCancelled,

    /// Received a touch event.
    Touch(TouchEvent),

    /// Touchpad pressure event.
    ///
    /// At the moment, only supported on Apple forcetouch-capable macbooks.
    /// The parameters are: pressure level (value between 0 and 1 representing how hard the touchpad
    /// is being pressed) and stage (integer representing the click level).
    TouchPressure(TouchpadPressure),

    /// The window gained focus.
    Focused,

    /// The window lost focus.
    Unfocused,

    /// The window was closed and is no longer stored in the `App`.
    Closed,
}
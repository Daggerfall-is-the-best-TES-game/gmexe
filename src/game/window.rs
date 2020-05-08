//! Windowing magic.

mod win32;

use crate::input::{Key, MouseButton};
use std::slice;

#[cfg(target_os = "windows")]
use win32 as platform;

#[derive(Copy, Clone)]
pub enum Backend {
    OpenGL,
    Vulkan, // unimplemented so far
}

#[derive(Copy, Clone)]
pub enum Cursor {
    Arrow,     // ⇖
    AppStart,  // ⇖⌛
    Beam,      // I
    Cross,     // +
    Hand,      // 👆
    Hourglass, // ⌛
    Invisible, //
    SizeNESW,  // ⤢
    SizeNS,    // ↕
    SizeNWSE,  // ⤡
    SizeWE,    // ↔
    SizeAll,   // ✥
    Up,        // ↑
}

impl Default for Cursor {
    fn default() -> Self {
        Self::Arrow
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Event {
    Resize(u32, u32),
    KeyboardDown(Key),
    KeyboardUp(Key),
    MouseButtonDown(MouseButton),
    MouseButtonUp(MouseButton),
    MouseWheelUp,
    MouseWheelDown,
    MouseMove(i32, i32),
}

#[derive(Copy, Clone)]
pub enum Style {
    /// Regular non-resizable decorated window (minimize, close buttons).
    Regular,

    /// Same as Regular but with additional resizability and a maximize button.
    Resizable,

    /// Same as Regular except no buttons in the title bar.
    Undecorated,

    /// Borderless window.
    Borderless,

    /// Borderless fullscreen mode.
    BorderlessFullscreen,
}

pub struct Window(Box<dyn WindowTrait>);

pub trait WindowTrait {
    fn close_requested(&self) -> bool;
    fn set_close_requested(&mut self, value: bool);
    fn get_inner_size(&self) -> (u32, u32);
    fn process_events<'a>(&'a mut self) -> slice::Iter<'a, Event>;
    fn resize(&mut self, width: u32, height: u32);
    fn set_style(&mut self, style: Style);
    fn set_visible(&mut self, visible: bool);
    fn window_handle(&self) -> usize;
}

impl Window {
    /// Creates a new Window, invisible by default.
    pub fn new(builder: &WindowBuilder) -> Result<Self, String> {
        Ok(Self(Box::new(platform::WindowImpl::new(builder)?)))
    }

    /// Returns whether the window requested to be closed.
    pub fn close_requested(&self) -> bool {
        self.0.close_requested()
    }

    /// Sets the value of close_requested.
    pub fn set_close_requested(&mut self, value: bool) {
        self.0.set_close_requested(value)
    }

    /// Gets the inner size of the window, that is, the area we draw to.
    pub fn get_inner_size(&self) -> (u32, u32) {
        self.0.get_inner_size()
    }

    /// Processes window events and returns a one-time Iterator to them.
    pub fn process_events<'a>(&'a mut self) -> slice::Iter<'a, Event> {
        self.0.process_events()
    }

    /// Sets the inner size of the window, as in the area we draw to.
    pub fn resize(&mut self, width: u32, height: u32) {
        self.0.resize(width, height)
    }

    /// Sets the window style.
    pub fn set_style(&mut self, style: Style) {
        self.0.set_style(style)
    }

    /// Sets whether the window is visible at all.
    pub fn set_visible(&mut self, visible: bool) {
        self.0.set_visible(visible)
    }

    /// Returns the platform-specific window handle.
    /// This is not only here for Renderer, but there's a GML getter for this too...
    pub fn window_handle(&self) -> usize {
        self.0.window_handle()
    }
}

pub struct WindowBuilder {
    cursor: Cursor,
    size: (u32, u32),
    style: Style,
    title: String,
}

impl Default for WindowBuilder {
    fn default() -> Self {
        Self { cursor: Cursor::default(), size: (640, 480), style: Style::Regular, title: String::new() }
    }
}

impl WindowBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_cursor(self, cursor: Cursor) -> Self {
        Self { cursor, ..self }
    }

    pub fn with_size(self, width: u32, height: u32) -> Self {
        Self { size: (width, height), ..self }
    }

    pub fn with_style(self, style: Style) -> Self {
        Self { style, ..self }
    }

    pub fn with_title(self, title: impl Into<String>) -> Self {
        Self { title: title.into(), ..self }
    }

    pub fn build(&self) -> Result<Window, String> {
        Window::new(self)
    }
}

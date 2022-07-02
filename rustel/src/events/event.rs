use bitflags::bitflags;

#[derive(Debug)]
pub enum EventType {
    None,

    // Window Events
    WindowCloseEvent,
    WindowResizeEvent,
    WindowFocusEvent,
    WindowLostFocusEvent,
    WindowMovedEvent,

    // Application Events
    AppTickEvent,
    AppUpdateEvent,
    AppRenderEvent,

    // Key Events
    KeyPressedEvent,
    KeyReleasedEvent,

    // Mouse Events
    MouseButtonPressedEvent,
    MouseButtonReleasedEvent,
    MouseMovedEvent,
    MouseScrolledEvent,
}

bitflags! {
    pub struct EventCategory: u32 {
        const NONE         = 0b00000000;
        const APPLICATION  = 0b00000001;
        const INPUT        = 0b00000010;
        const KEYBOARD     = 0b00000100;
        const MOUSE        = 0b00001000;
        const MOUSE_BUTTON = 0b00010000;
    }
}

pub trait Event: ToString {
    fn event_type(&self) -> EventType;
    fn name(&self) -> &str;
    fn in_category(&self, category: EventCategory) -> bool;
}

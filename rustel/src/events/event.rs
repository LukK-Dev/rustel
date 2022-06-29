use bitflags::bitflags;

pub enum EventType {
    None,

    // Window Events
    WindowClose,
    WindowResize,
    WindowFocus,
    WindowLostFocus,
    WindowMoved,

    // Application Events
    AppTick,
    AppUpdate,
    AppRender,

    // Key Events
    KeyPressed,
    KeyReleased,

    // Mouse Events
    MouseButtonPressed,
    MouseButtonReleased,
    MouseMoved,
    MouseScrolled,
}

pub enum EventCategory {
    None,
    EventCategoryApplication,
    EventCategoryInput,
    EventCategoryKeyboard,
    EventCategoryMouse,
    EventCategoryMouseButton,
}

impl EventCategory {
    pub fn bitflag(&self) -> EventCategoryFlag {
        match *self {
            Self::None => EventCategoryFlag::NONE,
            Self::EventCategoryApplication => EventCategoryFlag::EVENT_CATEGORY_APPLICATION,
            Self::EventCategoryInput => EventCategoryFlag::EVENT_CATEGORY_INPUT,
            Self::EventCategoryKeyboard => EventCategoryFlag::EVENT_CATEGORY_KEYBOARD,
            Self::EventCategoryMouse => EventCategoryFlag::EVENT_CATEGORY_MOUSE,
            Self::EventCategoryMouseButton => EventCategoryFlag::EVENT_CATEGORY_MOUSE_BUTTON,
        }
    }
}

bitflags! {
    pub struct EventCategoryFlag: u32 {
        const NONE = 0b00000000;
        const EVENT_CATEGORY_APPLICATION = 0b00000001;
        const EVENT_CATEGORY_INPUT       = 0b00000010;
        const EVENT_CATEGORY_KEYBOARD    = 0b00000100;
        const EVENT_CATEGORY_MOUSE       = 0b00001000;
        const EVENT_CATEGORY_MOUSE_BUTTON = 0b00010000;
    }
}

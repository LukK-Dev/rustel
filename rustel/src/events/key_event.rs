use rustel_derive::Event;
use super::event::{Event, EventType};
use crate::events::event::EventCategory;

#[derive(Event)]
pub struct KeyPressedEvent {
    pub keycode: u32,
    pub repeat_count: u32,
    pub category: EventCategory,
    handled: bool,
}

impl KeyPressedEvent {
    pub fn new(keycode: u32, repeat_count: u32) -> Self {
        let category = EventCategory::INPUT | EventCategory::KEYBOARD;
        let handled = false;

        Self {
            keycode,
            repeat_count,
            category,
            handled,
        }
    }
}

impl ToString for KeyPressedEvent {
    fn to_string(&self) -> String {
        format!("KeyReleasedEvent: {} ({} repeats)", self.keycode, self.repeat_count)
    }
}

#[derive(Event)]
pub struct KeyReleasedEvent {
    pub keycode: u32,
    pub category: EventCategory,
    handled: bool,
}

impl KeyReleasedEvent {
    pub fn new(keycode: u32) -> Self {
        let category = EventCategory::INPUT | EventCategory::KEYBOARD;
        let handled = false;

        Self {
            keycode,
            category,
            handled,
        }
    }
}

impl ToString for KeyReleasedEvent {
    fn to_string(&self) -> String {
        format!("KeyReleasedEvent: {}", self.keycode)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn in_category() {
        let e1 = KeyPressedEvent::new(10, 1);
        let e2 = KeyReleasedEvent::new(10);

        assert!(e1.in_category(EventCategory::INPUT));
        assert!(e1.in_category(EventCategory::KEYBOARD));
        assert!(!e1.in_category(EventCategory::MOUSE));
        assert!(e2.in_category(EventCategory::INPUT));
        assert!(e2.in_category(EventCategory::KEYBOARD));
        assert!(!e2.in_category(EventCategory::MOUSE));
    }
}

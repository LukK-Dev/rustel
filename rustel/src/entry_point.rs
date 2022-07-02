use log::info;

use crate::{application::Application, log::init_logger, events::{key_event::KeyPressedEvent, event::{Event, EventCategory}}};

pub fn run(mut application: impl Application) {
    init_logger().expect("Failed to initialize Logger");

    application.run().expect("Failed to run Application");
}

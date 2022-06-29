use crate::{application::Application, log::init_logger};

pub fn run(mut application: impl Application) {
    init_logger().expect("Failed to initialize Logger");

    application.run().expect("Failed to run Application");
}

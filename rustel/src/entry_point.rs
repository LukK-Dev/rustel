use crate::{application::Application, log::init_logger};

pub fn run(mut application: impl Application) {
    init_logger().unwrap();

    application.run().unwrap();
}

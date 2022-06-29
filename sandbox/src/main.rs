#![allow(dead_code)]
use rustel::application::Application;
use rustel::entry_point::run;
use rustel::errors::ApplicationError;

struct Sandbox;

impl Application for Sandbox {
    fn run(&mut self) -> Result<(), ApplicationError> {
        loop {}
    }
}

fn main() {
    let sandbox = Sandbox;
    run(sandbox);
}

#![allow(dead_code)]
use rustel::prelude::*;

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

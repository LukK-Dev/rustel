use crate::errors::ApplicationError;

pub trait Application {
    fn run(&mut self) -> Result<(), ApplicationError> {
        // i dont know
        Ok(())
    }
}
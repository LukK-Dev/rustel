use std::{error::Error, fmt};

#[derive(Debug)]
pub enum ApplicationError<'a> {
    GenericError(&'a str),
}

impl Error for ApplicationError<'_> {
    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }

    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            Self::GenericError(..) => None,
        }
    }
}

impl fmt::Display for ApplicationError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::GenericError(msg) => write!(f, "{}", msg),
        }
    }
}
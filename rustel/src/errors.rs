use std::{error::Error, fmt};

// inspired by fern errors :)

#[derive(Debug)]
pub enum ApplicationError {
    GenericError(String),
    LogInitError(log::SetLoggerError),
}

impl Error for ApplicationError {
    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }

    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::GenericError(..) => None,
            Self::LogInitError(ref e) => Some(e),
        }
    }
}

impl fmt::Display for ApplicationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::GenericError(msg) => write!(f, "{}", msg),
            Self::LogInitError(ref e) => write!(f, "Failed to initialize Logger: {}", e),
        }
    }
}

impl From<log::SetLoggerError> for ApplicationError {
    fn from(error: log::SetLoggerError) -> Self {
        Self::LogInitError(error)
    }
}
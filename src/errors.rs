use std::{fmt, io::Error as IoError};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io(IoError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(e) => write!(f, "IO failed: {}", e),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Io(e) => Some(e),
        }
    }
}

macro_rules! impl_from_error {
    ($source:ty, $variant:ident) => {
        impl From<$source> for Error {
            fn from(error: $source) -> Self {
                Error::$variant(error)
            }
        }
    };
}

impl_from_error!(IoError, Io);

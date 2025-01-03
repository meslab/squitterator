use std::{fmt, io::Error as IoError};

pub type AppResult<T> = std::result::Result<T, AppError>;

#[derive(Debug)]
pub enum AppError {
    Io(IoError),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Io(e) => write!(f, "IO failed: {}", e),
        }
    }
}

impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AppError::Io(e) => Some(e),
        }
    }
}

macro_rules! impl_from_error {
    ($source:ty, $variant:ident) => {
        impl From<$source> for AppError {
            fn from(error: $source) -> Self {
                AppError::$variant(error)
            }
        }
    };
}

impl_from_error!(IoError, Io);

use std::io;
use std::fmt;
use std::error::Error;

#[derive(Error)]
pub enum AppError {
    MissingQuery,
    MissingFilename,
    ConfigLoad {
        error: io::Error,
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingQuery => f.write_str("No query string"),
            Self::MissingFilename => f.write_str("No file name"),
            Self::ConfigLoad { error } => write!(f, "Error state: {}", source),
        }
    }
}
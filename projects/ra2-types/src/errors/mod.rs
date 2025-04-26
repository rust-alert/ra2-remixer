//! Error types for RA2 MIX file operations

mod convert;
use std::{
    fmt::{Display, Formatter},
};

/// Result type for RA2 MIX file operations
pub type Result<T> = std::result::Result<T, MixError>;

/// Error type for RA2 MIX file operations
#[derive(Debug)]
pub enum MixError {
    /// IO error
    IoError(std::io::Error),

    /// Crypto error
    CryptoError(String),

    /// Invalid file format
    InvalidFormat(String),

    /// Missing file
    FileNotFound(String),
}

impl Display for MixError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MixError::IoError(e) => {
                write!(f, "IO error: {}", e)
            }
            MixError::CryptoError(e) => {
                write!(f, "Crypto error:: {}", e)
            }
            MixError::InvalidFormat(e) => {
                write!(f, "Invalid file format: {}", e)
            }
            MixError::FileNotFound(e) => {
                write!(f, "File not found: {}", e)
            }
        }
    }
}

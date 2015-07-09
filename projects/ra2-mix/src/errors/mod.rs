//! Error types for RA2 MIX file operations

use std::io;
use thiserror::Error;

/// Error type for RA2 MIX file operations
#[derive(Error, Debug)]
pub enum MixError {
    /// IO error
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),

    /// Crypto error
    #[error("Crypto error: {0}")]
    CryptoError(String),

    /// Invalid file format
    #[error("Invalid file format: {0}")]
    InvalidFormat(String),

    /// Missing file
    #[error("File not found: {0}")]
    FileNotFound(String),

    /// Invalid argument
    #[error("Invalid argument: {0}")]
    InvalidArgument(String),
}

/// Result type for RA2 MIX file operations
pub type Result<T> = std::result::Result<T, MixError>;
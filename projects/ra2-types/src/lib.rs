#![deny(missing_debug_implementations, missing_copy_implementations)]
#![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://avatars.githubusercontent.com/u/208321371")]
#![doc(html_favicon_url = "https://avatars.githubusercontent.com/u/208321371")]

//! RA2 MIX file format library
//!
//! This library provides functionality for reading and writing Red Alert 2 MIX files.
//! It supports both encrypted and unencrypted MIX files, and can extract files from MIX archives.

mod errors;

use std::error::Error;
use std::fmt::{Display, Formatter};
#[cfg(feature = "image")]
pub use image::{Rgba, RgbaImage};

/// Result type for RA2 MIX file operations
pub type Result<T> = std::result::Result<T, MixError>;

/// Error type for RA2 MIX file operations
#[derive(Debug)]
pub enum MixError {
    /// IO error
    IoError(std::io::Error),

    /// Crypto error
    CryptoError {
        /// The error message
        message: String,
    },

    /// Invalid file format
    InvalidFormat(String),

    /// Missing file
    FileNotFound(String),
}

impl Error for MixError { }

impl Display for MixError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MixError::IoError(e) => {
                write!(f, "IO error: {}", e)
            }
            MixError::CryptoError { message: e } => {
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

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
mod games;

pub use crate::games::CncGame;
#[cfg(feature = "image")]
pub use image::{DynamicImage, Rgba, RgbaImage};
#[cfg(feature = "apng")]
pub use apng;
#[cfg(feature = "walkdir")]
pub use walkdir::WalkDir;


use std::{
    error::Error,
    fmt::{Display, Formatter},
};

/// Result type for RA2 MIX file operations
pub type Result<T> = std::result::Result<T, Ra2Error>;

/// Error type for RA2 MIX file operations
#[derive(Debug)]
pub enum Ra2Error {
    /// IO error
    IoError(std::io::Error),

    /// Crypto error
    CryptoError {
        /// The error message
        message: String,
    },
    /// Invalid file format
    InvalidFormat {
        /// The error message
        message: String,
    },
    /// Encode error
    EncodeError {
        /// The error format
        format: String,
        /// The error message
        message: String,
    },
    /// Decode error
    DecodeError {
        /// The error format
        format: String,
        /// The error message
        message: String,
    },
    /// Missing file
    FileNotFound(String),
    /// Out of boundary
    OutOfBoundary {
        /// Max limit
        limit: usize,
        /// The error message
        message: String,
    },
}

impl Error for Ra2Error {}

impl Display for Ra2Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Ra2Error::IoError(e) => {
                write!(f, "IO error: {}", e)
            }
            Ra2Error::CryptoError { message: e } => {
                write!(f, "Crypto error:: {}", e)
            }
            Ra2Error::InvalidFormat { message: e } => {
                write!(f, "Invalid file format: {}", e)
            }
            Ra2Error::FileNotFound(e) => {
                write!(f, "File not found: {}", e)
            }
            Ra2Error::DecodeError { format, message } => {
                write!(f, "Decode error: {}: {}", format, message)
            }
            Ra2Error::EncodeError { format, message } => {
                write!(f, "Encode error: {}: {}", format, message)
            }
            Ra2Error::OutOfBoundary { limit, message } => {
                write!(f, "Out of boundary {}: {}", limit, message)
            }
        }
    }
}

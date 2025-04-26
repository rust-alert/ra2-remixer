//! Error types for RA2 MIX file operations

#[cfg(feature = "image")]
mod from_image;

use crate::MixError;

impl From<std::io::Error> for MixError {
    fn from(error: std::io::Error) -> Self {
        Self::IoError(error)
    }
}
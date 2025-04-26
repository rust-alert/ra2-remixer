//! Error types for RA2 MIX file operations

#[cfg(feature = "image")]
mod from_image;

use crate::Ra2Error;

impl From<std::io::Error> for Ra2Error {
    fn from(error: std::io::Error) -> Self {
        Self::IoError(error)
    }
}
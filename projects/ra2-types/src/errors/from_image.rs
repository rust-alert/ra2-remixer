use crate::Ra2Error;
use image::{Frames, ImageError};

impl From<ImageError> for Ra2Error {
    fn from(error: ImageError) -> Self {
        Self::InvalidFormat { message: error.to_string() }
    }
}

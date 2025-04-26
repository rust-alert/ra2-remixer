use crate::Ra2Error;
use image::ImageError;

impl From<ImageError> for Ra2Error {
    fn from(error: ImageError) -> Self {
        Self::InvalidFormat { message: error.to_string() }
    }
}

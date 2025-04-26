use crate::MixError;
use image::ImageError;

impl From<ImageError> for MixError {
    fn from(error: ImageError) -> Self {
        Self::InvalidFormat(error.to_string())
    }
}

use crate::Ra2Error;
use apng::errors::{APNGError, AppError};

impl From<APNGError> for Ra2Error {
    fn from(error: APNGError) -> Self {
        Self::EncodeError { format: "apng".to_string(), message: error.to_string() }
    }
}

impl From<AppError> for Ra2Error {
    fn from(error: AppError) -> Self {
        Self::EncodeError { format: "png".to_string(), message: error.to_string() }
    }
}
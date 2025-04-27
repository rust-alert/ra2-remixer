use crate::Ra2Error;
use apng::errors::APNGError;

impl From<APNGError> for Ra2Error {
    fn from(error: APNGError) -> Self {
        Self::EncodeError { format: "apng".to_string(), message: error.to_string() }
    }
}

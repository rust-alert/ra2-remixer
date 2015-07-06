use super::*;

impl From<std::io::Error> for MixError {
    fn from(_error: std::io::Error) -> Self {
        Self::UnknownError
    }
}
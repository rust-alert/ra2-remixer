use super::*;

impl From<std::io::Error> for MixError {
    fn from(error: std::io::Error) -> Self {
        Self::IoError(error)
    }
}
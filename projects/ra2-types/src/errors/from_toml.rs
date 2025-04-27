use crate::Ra2Error;

impl From<toml::ser::Error> for Ra2Error {
    fn from(error: toml::ser::Error) -> Self {
        Self::EncodeError { format: "toml".to_string(), message: error.to_string() }
    }
}

impl From<toml::de::Error> for Ra2Error {
    fn from(error: toml::de::Error) -> Self {
        Self::DecodeError { format: "toml".to_string(), message: error.to_string() }
    }
}

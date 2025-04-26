use super::*;

impl Error for MixError { }

impl Display for MixError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self { MixError::UnknownError => { write!(f, "UnknownError") } }
    }
}

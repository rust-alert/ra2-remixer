use std::fmt::{Debug, Formatter};
use std::error::Error;
use std::fmt::Display;

mod display;
mod convert;

/// The kind of [ExampleError].
#[derive(Debug, Copy, Clone)]
pub enum MixError {
    /// An unknown error.
    UnknownError
}



#![deny(missing_debug_implementations, missing_copy_implementations)]
#![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]

//! RA2 MIX file format library
//! 
//! This library provides functionality for reading and writing Red Alert 2 MIX files.
//! It supports both encrypted and unencrypted MIX files, and can extract files from MIX archives.

mod checksum;
mod constants;
mod crypto;
mod errors;
mod reader;
mod writer;

pub use crate::constants::XCCGame;
pub use crate::errors::MixError;
pub use crate::reader::{extract, read, read_file_info, FileEntry, Header};
pub use crate::writer::write;

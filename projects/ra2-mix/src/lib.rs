#![deny(missing_debug_implementations, missing_copy_implementations)]
#![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]

//! RA2 MIX file format library
//! 
//! This library provides functionality for reading and writing Red Alert 2 MIX files.
//! It supports both encrypted and unencrypted MIX files, and can extract files from MIX archives.

pub mod checksum;
pub mod constants;
pub mod crypto;
pub mod errors;
pub mod reader;
pub mod writer;
pub mod xcc_package;

pub use crate::constants::XccGame;
pub use crate::errors::MixError;
pub use crate::reader::{extract, decrypt, read_file_info, FileEntry, Header};
pub use crate::writer::encrypt;
pub use xcc_package::XccPackage;
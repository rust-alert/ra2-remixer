#![deny(missing_debug_implementations, missing_copy_implementations)]
#![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://avatars.githubusercontent.com/u/208321371")]
#![doc(html_favicon_url = "https://avatars.githubusercontent.com/u/208321371")]

//! RA2 MIX file format library
//!
//! This library provides functionality for reading and writing Red Alert 2 MIX files.
//! It supports both encrypted and unencrypted MIX files, and can extract files from MIX archives.

mod checksum;
mod constants;
mod crypto;
mod xcc_package;

pub use ra2_types::{Ra2Error, Result};

pub use crate::{
    constants::XccGame,
    xcc_package::{MixPackage, extract, patch},
};

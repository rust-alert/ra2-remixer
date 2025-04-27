#![deny(missing_debug_implementations, missing_copy_implementations)]
#![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://avatars.githubusercontent.com/u/208321371")]
#![doc(html_favicon_url = "https://avatars.githubusercontent.com/u/208321371")]

//! RA2 MIX file format library
//!
//! This library provides functionality for reading and writing Red Alert 2 MIX files.
//! It supports both encrypted and unencrypted MIX files, and can extract files from MIX archives.

mod frames;
mod reader;

pub use crate::{
    frames::ShpFrame,
    reader::{ShpReader, shp_with_pal, shp2apng, shp2png},
};

/// 文件头结构体
#[derive(Copy, Clone, Debug)]
pub struct ShpHeader {
    /// Unused reserved header
    pub reserved: u16,
    /// The animation width
    pub width: u16,
    /// The animation height
    pub height: u16,
    /// The animation frames
    pub number_of_frames: u16,
}

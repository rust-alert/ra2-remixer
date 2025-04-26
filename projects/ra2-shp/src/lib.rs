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

pub use crate::{frames::ShpFrame, reader::ShpReader};

// 文件头结构体
#[derive(Copy, Clone, Debug)]
pub struct ShpHeader {
    pub reserved: u16,         // 保留字 (必须为 0)
    pub width: u16,            // 宽度
    pub height: u16,           // 高度
    pub number_of_frames: u16, // 帧数
}

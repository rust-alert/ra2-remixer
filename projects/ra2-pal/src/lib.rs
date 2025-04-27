#![deny(missing_debug_implementations, missing_copy_implementations)]
#![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://avatars.githubusercontent.com/u/208321371")]
#![doc(html_favicon_url = "https://avatars.githubusercontent.com/u/208321371")]

//! RA2 MIX file format library
//!
//! This library provides functionality for reading and writing Red Alert 2 MIX files.
//! It supports both encrypted and unencrypted MIX files, and can extract files from MIX archives.

mod colors;
mod reader;

pub use crate::colors::Ra2Color;
use ra2_types::Ra2Error;
use serde::Serialize;

/// `PAL` files contain color palettes for various objects in the game.
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize)]
pub struct Palette {
    /// The 256 colors in palette
    #[serde(serialize_with = "<[_]>::serialize")]
    pub colors: [Ra2Color; 256],
}

impl Palette {
    /// 从字节数组创建 PalFile 实例
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Ra2Error> {
        if bytes.len() != 256 * 3 {
            return Err(Ra2Error::InvalidFormat {
                message: "字节数组长度不正确，PAL 文件应为 256 * 3 字节".to_string()
            });
        }

        let mut colors: [Ra2Color; 256] = [Ra2Color { red: 0, green: 0, blue: 0 }; 256];

        for i in 0..256 {
            colors[i].red = bytes[i * 3];
            colors[i].green = bytes[i * 3 + 1];
            colors[i].blue = bytes[i * 3 + 2];
        }

        Ok(Palette { colors })
    }

    /// 获取指定索引的颜色
    pub fn get_color(&self, index: u8) -> Result<Ra2Color, Ra2Error> {
        match self.colors.get(index as usize) {
            Some(s) => Ok(*s),
            None => Err(Ra2Error::InvalidFormat { message: "超出范围".to_string() }),
        }
    }
}

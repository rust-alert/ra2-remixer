#![deny(missing_debug_implementations, missing_copy_implementations)]
#![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://avatars.githubusercontent.com/u/208321371")]
#![doc(html_favicon_url = "https://avatars.githubusercontent.com/u/208321371")]

//! RA2 MIX file format library
//!
//! This library provides functionality for reading and writing Red Alert 2 MIX files.
//! It supports both encrypted and unencrypted MIX files, and can extract files from MIX archives.

use serde::{Deserialize, Serialize};

/// `PAL` files contain color palettes for various objects in the game.
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize)]
pub struct Palette {
    /// The 256 colors in palette
    #[serde(serialize_with = "<[_]>::serialize")]
    pub colors: [Ra2Color; 256],
}

/// The palette color used in game
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Ra2Color {
    /// red
    pub red: u8,
    /// green
    pub green: u8,
    /// blue
    pub blue: u8,
}

impl Palette {
    /// 从字节数组创建 PalFile 实例
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
        if bytes.len() != 256 * 3 {
            return Err("字节数组长度不正确，PAL 文件应为 256 * 3 字节".to_string());
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
    pub fn get_color(&self, index: usize) -> Option<Ra2Color> {
        if index < 256 { Some(self.colors[index]) } else { None }
    }
}

/// 将 6 位颜色值转换为 8 位颜色值 (将值乘以 255/63)
pub fn convert_6bit_to_8bit(color: u8) -> u8 {
    ((color as u32 * 255) / 63) as u8
}

/// 将 8 位颜色值转换为 5 位或 6 位颜色值 (分别用于红/蓝和绿)
pub fn convert_8bit_to_5or6bit(color: u8, is_green: bool) -> u8 {
    let divider = if is_green { 4 } else { 8 };
    (color as u32 / divider) as u8
}

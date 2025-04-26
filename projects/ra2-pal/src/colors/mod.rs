use image::{Rgba, RgbaImage};
use serde::{Deserialize, Serialize};

/// The palette color used in game
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize)]
pub struct Ra2Color {
    /// red 5 bits
    pub red: u8,
    /// green 6 bits
    pub green: u8,
    /// blue 5 bits
    pub blue: u8,
}

impl From<Ra2Color> for Rgba<u8> {
    fn from(value: Ra2Color) -> Self {
        Rgba([
            ((value.red as u32 * 255) / 63) as u8,
            ((value.green as u32 * 255) / 63) as u8,
            ((value.blue as u32 * 255) / 63) as u8,
            255,
        ])
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

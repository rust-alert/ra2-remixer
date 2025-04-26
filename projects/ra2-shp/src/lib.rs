#![deny(missing_debug_implementations, missing_copy_implementations)]
#![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://avatars.githubusercontent.com/u/208321371")]
#![doc(html_favicon_url = "https://avatars.githubusercontent.com/u/208321371")]

//! RA2 MIX file format library
//!
//! This library provides functionality for reading and writing Red Alert 2 MIX files.
//! It supports both encrypted and unencrypted MIX files, and can extract files from MIX archives.

use byteorder::{LittleEndian, ReadBytesExt};
use std::{
    error::Error,
    io::{Read, Seek, SeekFrom},
};

// 文件头结构体
#[derive(Copy, Clone, Debug)]
pub struct FileHeader {
    pub reserved: u16,         // 保留字 (必须为 0)
    pub width: u16,            // 宽度
    pub height: u16,           // 高度
    pub number_of_frames: u16, // 帧数
}

// 帧头结构体
#[derive(Copy, Clone, Debug)]
pub struct FrameHeader {
    pub x: u16,             // 水平位置 (0,0)
    pub y: u16,             // 垂直位置 (0,0)
    pub width: u16,         // 帧宽度
    pub height: u16,        // 帧高度
    pub flags: u8,          // 特殊标志
    pub reserved1: [u8; 3], // 对齐 (3 字节)
    pub color: u32,         // 颜色 (可以是透明色)
    pub reserved2: u32,     // 保留字2 (未使用)
    pub offset: u32,        // 帧数据在文件中的偏移量
}

// 读取文件头
pub fn read_file_header<R: Read>(reader: &mut R) -> Result<FileHeader, Box<dyn Error>> {
    let reserved = reader.read_u16::<LittleEndian>()?;
    let width = reader.read_u16::<LittleEndian>()?;
    let height = reader.read_u16::<LittleEndian>()?;
    let number_of_frames = reader.read_u16::<LittleEndian>()?;

    Ok(FileHeader { reserved, width, height, number_of_frames })
}

// 读取帧头
pub fn read_frame_header<R: Read>(reader: &mut R) -> Result<FrameHeader, Box<dyn Error>> {
    let x = reader.read_u16::<LittleEndian>()?;
    let y = reader.read_u16::<LittleEndian>()?;
    let width = reader.read_u16::<LittleEndian>()?;
    let height = reader.read_u16::<LittleEndian>()?;
    let flags = reader.read_u8()?;
    let mut align = [0u8; 3];
    reader.read_exact(&mut align)?;
    let color = reader.read_u32::<LittleEndian>()?;
    let reserved2 = reader.read_u32::<LittleEndian>()?;
    let offset = reader.read_u32::<LittleEndian>()?;

    Ok(FrameHeader { x, y, width, height, flags, reserved1: align, color, reserved2, offset })
}

// 解压缩 RLE 数据
pub fn decompress_rle_data<R: Read>(reader: &mut R, frame_width: u16, frame_height: u16) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut decompressed_data = Vec::new();
    let mut row_length_buffer = [0u8; 2];

    for _ in 0..frame_height {
        // 读取行长度
        reader.read_exact(&mut row_length_buffer)?;
        let row_length = u16::from_le_bytes(row_length_buffer);

        let mut current_byte_index = 2; // 已经读取了两个字节的行长度
        while current_byte_index < row_length {
            let control_byte = reader.read_u8()?;
            current_byte_index += 1;

            if control_byte == 0x00 {
                // 透明像素
                let transparent_count = reader.read_u8()?;
                current_byte_index += 1;
                decompressed_data.extend(vec![0x00; transparent_count as usize]); // 0x00 代表透明
            }
            else {
                // 普通像素
                decompressed_data.push(control_byte);
            }
        }
    }

    Ok(decompressed_data)
}

// 读取帧数据
pub fn read_frame_data<R: Read + Seek>(reader: &mut R, frame_header: &FrameHeader) -> Result<Vec<u8>, Box<dyn Error>> {
    // 如果偏移量为 0，则表示空帧
    if frame_header.offset == 0 {
        return Ok(Vec::new());
    }

    // 跳转到帧数据的偏移位置
    reader.seek(SeekFrom::Start(frame_header.offset as u64))?;

    // 检查是否使用压缩
    if frame_header.flags & 0x02 != 0 {
        // 使用 RLE 压缩
        let decompressed_data = decompress_rle_data(reader, frame_header.width, frame_header.height)?;
        Ok(decompressed_data)
    }
    else {
        // 未压缩
        let frame_size = frame_header.width as u32 * frame_header.height as u32;
        let mut frame_data = vec![0u8; frame_size as usize];
        reader.read_exact(&mut frame_data)?;
        Ok(frame_data)
    }
}

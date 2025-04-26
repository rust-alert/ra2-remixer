use std::io::{BufReader, Read, Seek, SeekFrom};
use byteorder::{LittleEndian, ReadBytesExt};
use ra2_types::MixError;
use crate::{ShpFrame, ShpHeader};

#[derive(Debug)]
pub struct ShpReader<R> {
    header: ShpHeader,
    reader: BufReader<R>,
}

impl<R: Read> ShpReader<R> {
    pub fn new(buffer: R) -> Result<Self, MixError> {
        let mut reader = BufReader::new(buffer);
        let file_header = read_file_header(&mut reader)?;
        Ok(Self { header: file_header, reader })
    }
    pub fn header(&self) -> &ShpHeader {
        &self.header
    }
    pub fn read_frame(&mut self) -> Result<ShpFrame, MixError>
    where
        R: Seek,
    {
        let mut buffer = ShpFrame::default();
        buffer.read_frame_header(&mut self.reader)?;
        buffer.read_frame_data(&mut self.reader)?;
        Ok(buffer)
    }
}



// 读取文件头
pub fn read_file_header<R: Read>(reader: &mut R) -> Result<ShpHeader, MixError> {
    let reserved = reader.read_u16::<LittleEndian>()?;
    let width = reader.read_u16::<LittleEndian>()?;
    let height = reader.read_u16::<LittleEndian>()?;
    let number_of_frames = reader.read_u16::<LittleEndian>()?;

    Ok(ShpHeader { reserved, width, height, number_of_frames })
}

impl ShpFrame {
    // 读取帧头
    fn read_frame_header<R: Read>(&mut self, reader: &mut R) -> Result<(), MixError> {
        self.x = reader.read_u16::<LittleEndian>()?;
        self.y = reader.read_u16::<LittleEndian>()?;
        self.width = reader.read_u16::<LittleEndian>()?;
        self.height = reader.read_u16::<LittleEndian>()?;
        self.flags = reader.read_u8()?;
        reader.read_exact(&mut self.reserved1)?;
        self.color = reader.read_u32::<LittleEndian>()?;
        reader.read_exact(&mut self.reserved2)?;
        self.offset = reader.read_u32::<LittleEndian>()?;
        Ok(())
    }
    // 读取帧数据
    fn read_frame_data<R: Read + Seek>(&mut self, reader: &mut R) -> Result<(), MixError> {
        // 如果偏移量为 0，则表示空帧
        if self.offset == 0 {
            return Ok(());
        }

        // 跳转到帧数据的偏移位置
        reader.seek(SeekFrom::Start(self.offset as u64))?;

        // 检查是否使用压缩
        if self.flags & 0x02 != 0 {
            // 使用 RLE 压缩
            self.buffer = decompress_rle_data(reader, self.width, self.height)?;
        }
        else {
            // 未压缩
            let frame_size = self.width as u32 * self.height as u32;
            self.buffer = vec![0u8; frame_size as usize];
            reader.read_exact(&mut self.buffer)?;
        }
        Ok(())
    }
}

// 解压缩 RLE 数据
pub fn decompress_rle_data<R: Read>(reader: &mut R, frame_width: u16, frame_height: u16) -> Result<Vec<u8>, MixError> {
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

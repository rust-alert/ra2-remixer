use crate::{ShpFrame, ShpHeader};

use byteorder::{LittleEndian, ReadBytesExt};
use ra2_pal::Palette;
use ra2_types::{DynamicImage, Ra2Error, WalkDir, apng};
use std::{
    fs::File,
    io::{BufReader, BufWriter, Read, Seek, SeekFrom},
    path::Path,
};

/// The lazy SHP reader
#[derive(Debug)]
pub struct ShpReader {
    header: ShpHeader,
    reader: BufReader<File>,
}

impl ShpReader {
    /// Create a new shp reader from file or buffer
    ///
    /// # Arguments
    ///
    /// * `buffer`:
    ///
    /// returns: Result<ShpReader<R>, Ra2Error>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use std::path::Path;
    /// # use ra2_shp::ShpReader;
    /// let file = Path::new("ra2/conquer/engineer.shp");
    /// let shp = ShpReader::new(file)?;
    /// ```
    pub fn new(file: &Path) -> Result<Self, Ra2Error> {
        let file = File::open(file)?;
        let mut reader = BufReader::new(file);
        let file_header = read_file_header(&mut reader)?;
        Ok(Self { header: file_header, reader })
    }
    /// Count the frames in `sha` animation
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use std::path::Path;
    /// # use ra2_shp::ShpReader;
    /// let file = Path::new("ra2/conquer/engineer.shp");
    /// let mut shp = ShpReader::new(file)?;
    /// let frames = shp.animation_frames();
    /// ```
    pub fn animation_frames(&self) -> u32 {
        self.header.number_of_frames as u32
    }
    /// Get the max animation width in `sha` frames
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use std::path::Path;
    /// # use ra2_shp::ShpReader;
    /// let file = Path::new("ra2/conquer/engineer.shp");
    /// let mut shp = ShpReader::new(file)?;
    /// let width = shp.animation_width();
    /// ```
    pub fn animation_width(&self) -> u32 {
        self.header.width as u32
    }
    /// Get the max animation height in `sha` frames
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use std::path::Path;
    /// # use ra2_shp::ShpReader;
    /// let file = Path::new("ra2/conquer/engineer.shp");
    /// let mut shp = ShpReader::new(file)?;
    /// let width = shp.animation_height();
    /// ```
    pub fn animation_height(&self) -> u32 {
        self.header.height as u32
    }
    /// Get the raw `sha` frame buffer in `O(1)` time
    ///
    /// # Arguments
    ///
    /// * `index`:
    ///
    /// returns: Result<ShpFrame, Ra2Error>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use std::path::Path;
    /// # use ra2_shp::ShpReader;
    /// let file = Path::new("ra2/conquer/engineer.shp");
    /// let mut shp = ShpReader::new(file)?;
    /// let idle = shp.get_frame(0);
    /// ```
    pub fn get_frame(&mut self, index: u64) -> Result<ShpFrame, Ra2Error> {
        self.reader.seek(SeekFrom::Start(8 + index * 24))?;
        let mut buffer = ShpFrame::default();
        buffer.read_frame_header(&mut self.reader)?;
        buffer.read_frame_data(&mut self.reader)?;
        Ok(buffer)
    }
}

// 读取文件头
pub fn read_file_header<R: Read>(reader: &mut R) -> Result<ShpHeader, Ra2Error> {
    let reserved = reader.read_u16::<LittleEndian>()?;
    let width = reader.read_u16::<LittleEndian>()?;
    let height = reader.read_u16::<LittleEndian>()?;
    let number_of_frames = reader.read_u16::<LittleEndian>()?;
    Ok(ShpHeader { reserved, width, height, number_of_frames })
}

impl ShpFrame {
    fn read_frame_header<R: Read>(&mut self, reader: &mut R) -> Result<(), Ra2Error> {
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
    fn read_frame_data<R: Read + Seek>(&mut self, reader: &mut R) -> Result<(), Ra2Error> {
        // 如果偏移量为 0，则表示空帧
        if self.offset == 0 {
            return Ok(());
        }
        // 跳转到帧数据的偏移位置
        reader.seek(SeekFrom::Start(self.offset as u64))?;
        // 检查是否使用压缩
        if self.flags & 0x02 == 0 {
            // 未压缩
            let frame_size = self.width as u32 * self.height as u32;
            self.buffer = vec![0u8; frame_size as usize];
            reader.read_exact(&mut self.buffer)?;
        }
        else {
            // 使用 RLE 压缩
            self.buffer = decompress_rle_data(reader, self.width, self.height)?;
        }
        debug_assert_eq!(self.buffer.len(), self.width as usize * self.height as usize);
        Ok(())
    }
}

// 解压缩 RLE 数据
pub fn decompress_rle_data<R: Read>(reader: &mut R, frame_width: u16, frame_height: u16) -> Result<Vec<u8>, Ra2Error> {
    let mut decompressed_data = Vec::with_capacity(frame_width as usize * frame_height as usize);
    for _ in 0..frame_height {
        let mut line_buffer = Vec::with_capacity(frame_width as usize);
        // 获取该行长度
        let row_length = reader.read_u16::<LittleEndian>()?;
        // 已经读取了两个字节的行长度
        let mut current_byte_index = 2;
        while current_byte_index < row_length {
            let control_byte = reader.read_u8()?;
            current_byte_index += 1;
            // 0x00 代表透明
            if control_byte == 0x00 {
                // 透明像素个数
                let transparent_count = reader.read_u8()?;
                current_byte_index += 1;
                line_buffer.extend(vec![0x00; transparent_count as usize]);
            }
            else {
                line_buffer.push(control_byte);
            }
        }
        // 不明原因导致 line_buffer 有可能比 frame_width 长, 此时截掉多余部分即可
        for index in 0..frame_width {
            let byte = line_buffer.get(index as usize).unwrap_or(&0);
            decompressed_data.push(*byte);
        }
    }
    Ok(decompressed_data)
}

/// Convert shp file to png format
///
/// # Arguments
///
/// * `file`:
/// * `palette`:
///
/// returns: Result<(), Ra2Error>
///
/// # Examples
///
/// ```
/// ```
pub fn shp2png(file: &Path, palette: &Palette) -> Result<(), Ra2Error> {
    match file.extension() {
        Some(s) if s.eq("shp") => {
            let mut shp = ShpReader::new(file)?;
            let frame = shp.get_frame(0)?;
            let image = frame.render(palette, shp.animation_width(), shp.animation_height())?;
            image.save(&file.with_extension("png"))?;
        }
        _ => {}
    }
    Ok(())
}
/// Convert shp file to apng format
///
/// # Arguments
///
/// * `file`:
/// * `palette`:
///
/// returns: Result<(), Ra2Error>
///
/// # Examples
///
/// ```
/// ```
pub fn shp2apng(file: &Path, palette: &Palette) -> Result<(), Ra2Error> {
    let shp_path = Path::new(file);
    let mut shp = ShpReader::new(shp_path)?;
    let mut png_images: Vec<apng::PNGImage> = Vec::new();
    for index in 0..shp.animation_frames() {
        match shp.get_frame(index as u64) {
            Ok(frame) => {
                let dy = DynamicImage::ImageRgba8(frame.render(&palette, shp.animation_width(), shp.animation_height())?);
                let png = apng::load_dynamic_image(dy).unwrap();
                png_images.push(png)
            }
            Err(e) => {
                tracing::error!("{}", e);
            }
        }
    }
    let path = shp_path.with_extension("apng");
    let mut out = BufWriter::new(File::create(path)?);
    let config = apng::create_config(&png_images, None)?;
    let mut encoder = apng::Encoder::new(&mut out, config).unwrap();
    let frame = apng::Frame { delay_num: Some(1), delay_den: Some(24), ..Default::default() };
    encoder.encode_all(png_images, Some(&frame))?;
    Ok(())
}

/// Convert shp with same name pal to png
/// 
/// # Arguments 
/// 
/// * `root`: 
/// 
/// returns: Result<(), Ra2Error> 
/// 
/// # Examples 
/// 
/// ```
/// 
/// ```
pub fn shp_with_pal(root: &Path) -> ra2_types::Result<()> {
    for entry in WalkDir::new(root) {
        let entry = entry.unwrap();
        let name = entry.file_name().to_string_lossy();
        if name.ends_with("shp") {
            let pal = entry.path().with_extension("pal");
            if pal.exists() {
                let pal = Palette::load(&pal)?;
                shp2png(&entry.path(), &pal)?;
            }
        }
    }
    Ok(())
}

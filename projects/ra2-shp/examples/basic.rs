//! Basic example of using RA2 MIX library

use ra2_shp::ShpReader;
use ra2_types::{MixError, Rgba, RgbaImage};
use std::fs::File;

fn main() -> Result<(), MixError> {
    // 打开文件
    let file = File::open(r#"E:\Games\Red Alert 2 - Yuris Revenge\提取\glsl.shp"#)?;
    let mut reader = ShpReader::new(file)?;

    // 读取文件头
    println!("File Header: {:?}", reader.header());

    while let Ok(frame) = reader.read_frame() {
        // 打印帧数据大小
        println!("Frame Data Size: {} bytes", frame.buffer.len());

        let mut image = RgbaImage::new(frame.width as u32, frame.height as u32);

        for (x, y, pixel) in image.enumerate_pixels_mut() {
            let index = x + y * frame.width as u32;
            let color = frame.buffer[index as usize];
            // 假设是灰度图像
            *pixel = Rgba([color, color, color, 255]);
        }
        // 保存为 PNG
        image.save(r#"E:\Games\Red Alert 2 - Yuris Revenge\提取\glsl.png"#)?;
    }

    Ok(())
}

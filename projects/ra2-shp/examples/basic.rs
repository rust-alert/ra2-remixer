//! Basic example of using RA2 MIX library

use image::RgbaImage;
use ra2_shp::{read_file_header, read_frame_data, read_frame_header};
use std::{error::Error, fs::File, io::BufReader, path::Path};

#[test]
fn main() -> Result<(), Box<dyn Error>> {
    // 打开文件
    let file = File::open(r#"E:\Games\Red Alert 2 - Yuris Revenge\提取\glsl.shp"#)?;
    let mut reader = BufReader::new(file);

    // 读取文件头
    let file_header = read_file_header(&mut reader)?;
    println!("File Header: {:?}", file_header);

    // 循环读取每一帧
    for i in 0..file_header.number_of_frames {
        println!("--- Frame {} ---", i + 1);

        // 读取帧头
        let frame_header = read_frame_header(&mut reader)?;
        println!("Frame Header: {:?}", frame_header);

        // 读取帧数据
        let frame_data = read_frame_data(&mut reader, &frame_header)?;

        // 打印帧数据大小
        println!("Frame Data Size: {} bytes", frame_data.len());

        let mut image = RgbaImage::new(frame_header.width as u32, frame_header.height as u32);

        for (x, y, pixel) in image.enumerate_pixels_mut() {
            let index = x + y * frame_header.width as u32;
            let color = frame_data[index as usize];
            // 假设是灰度图像
            *pixel = image::Rgba([color, color, color, 255]);
        }
        // 保存为 PNG
        image.save(r#"E:\Games\Red Alert 2 - Yuris Revenge\提取\glsl.png"#)?;
    }

    Ok(())
}

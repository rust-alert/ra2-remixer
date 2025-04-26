//! Basic example of using RA2 MIX library

use ra2_shp::ShpReader;
use ra2_types::{Ra2Error};
use std::fs::File;
use ra2_pal::Palette;

fn main() -> Result<(), Ra2Error> {
    let pal = include_bytes!(r#"E:\Games\Red Alert 2 - Yuris Revenge\提取\gls.pal"#);
    // 打开文件
    let file = File::open(r#"E:\Games\Red Alert 2 - Yuris Revenge\提取\glsl.shp"#)?;
    let mut reader = ShpReader::new(file)?;

    // 读取文件头
    println!("File Header: {:?}", reader.header());

    while let Ok(frame) = reader.read_frame() {
        // 打印帧数据大小
        println!("Frame Data Size: {} bytes", frame.buffer.len());
        let image = frame.render(Palette::from_bytes(pal)?)?;
        image.save(r#"E:\Games\Red Alert 2 - Yuris Revenge\提取\glsl.png"#)?;
    }

    Ok(())
}

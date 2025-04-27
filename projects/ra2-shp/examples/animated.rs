//! Example of working with encrypted RA2 MIX files

use apng::{Encoder, Frame, PNGImage, load_dynamic_image};
use ra2_pal::Palette;
use ra2_shp::ShpReader;
use ra2_types::{DynamicImage, Ra2Error};
use std::{fs::File, io::BufWriter, path::Path};

fn main() -> Result<(), Ra2Error> {
    let file = std::fs::read("E:\\Games\\Red Alert 2 - Yuri's Revenge\\ra2\\cache\\unittem.pal")?;
    let pal = Palette::decode(&file)?;
    let shp_path = Path::new("E:\\Games\\Red Alert 2 - Yuri's Revenge\\ra2\\conquer\\engineer.shp");
    let mut shp = ShpReader::new(File::open(shp_path)?)?;
    let mut png_images: Vec<PNGImage> = Vec::new();
    for index in 0..(shp.animation_frames() / 2) {
        match shp.get_frame(index as u64) {
            Ok(frame) => {
                let dy = DynamicImage::ImageRgba8(frame.render(&pal, shp.animation_width(), shp.animation_height())?);
                let png = load_dynamic_image(dy).unwrap();
                png_images.push(png)
            }
            Err(e) => {
                tracing::error!("{}", e);
            }
        }
    }
    let path = shp_path.with_extension("apng");
    let mut out = BufWriter::new(File::create(path)?);
    let config = apng::create_config(&png_images, None).unwrap();
    let mut encoder = Encoder::new(&mut out, config).unwrap();
    let frame = Frame { delay_num: Some(1), delay_den: Some(24), ..Default::default() };
    match encoder.encode_all(png_images, Some(&frame)) {
        Ok(_n) => println!("success"),
        Err(err) => eprintln!("{}", err),
    }
    Ok(())
}

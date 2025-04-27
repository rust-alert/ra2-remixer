//! Example of working with encrypted RA2 MIX files

use apng::{Encoder, Frame, PNGImage, load_dynamic_image};
use ra2_pal::Palette;
use ra2_shp::{ShpFrame, ShpReader};
use ra2_types::{DynamicImage, Ra2Error};
use std::{fs::File, io::BufWriter, path::Path};

fn main() -> Result<(), Ra2Error> {
    let file = std::fs::read("E:\\Games\\Red Alert 2 - Yuri's Revenge\\ra2\\cache\\unittem.pal")?;
    let pal = Palette::decode(&file)?;
    let mut shp = ShpReader::new(File::open("E:\\Games\\Red Alert 2 - Yuri's Revenge\\ra2\\conquer\\engineer.shp")?)?;

    let mut png_images: Vec<PNGImage> = Vec::new();

    loop {
        match shp.read_frame() {
            Ok(frame) => {
                let dy = DynamicImage::ImageRgba8(frame.render(&pal)?);
                let png = load_dynamic_image(dy).unwrap();
                png_images.push(png)
            }
            Err(e) => {
                println!("{:?}", e);
                break;
            }
        }
    }

    println!("PNG images: {:?}", png_images.len());

    let path = Path::new("E:\\Games\\Red Alert 2 - Yuri's Revenge\\ra2\\conquer\\engineer.png");
    let mut out = BufWriter::new(File::create(path)?);

    let config = apng::create_config(&png_images, None).unwrap();
    let mut encoder = Encoder::new(&mut out, config).unwrap();
    let frame = Frame { delay_num: Some(1), delay_den: Some(1), ..Default::default() };

    match encoder.encode_all(png_images, Some(&frame)) {
        Ok(_n) => println!("success"),
        Err(err) => eprintln!("{}", err),
    }
    Ok(())
}

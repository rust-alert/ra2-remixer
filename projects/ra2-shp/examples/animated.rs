//! Example of working with encrypted RA2 MIX files

use ra2_pal::Palette;
use ra2_shp::shp2apng;
use ra2_types::Ra2Error;
use std::path::Path;

fn main() -> Result<(), Ra2Error> {
    let root = Path::new("E:\\Games\\Red Alert 2 - Yuri's Revenge");
    let pal = Palette::load(&root.join("ra2/cache/unittem.pal"))?;
    let shp_path = root.join("ra2/conquer/engineer.shp");
    shp2apng(&shp_path, &pal)?;
    Ok(())
}

//! Basic example of using RA2 MIX library

use ra2_pal::Palette;
use ra2_shp::shp2png;
use ra2_types::Ra2Error;
use std::path::Path;

fn main() -> Result<(), Ra2Error> {
    let root = Path::new("E:\\Games\\Red Alert 2 - Yuri's Revenge");
    let pal = Palette::load(&root.join("ra2/local/gls.pal"))?;
    shp2png(&root.join("ra2/local/glsl.shp"), &pal)?;
    let pal = Palette::load(&root.join("langmd/glsmd.pal"))?;
    shp2png(&root.join("langmd/glslmd.shp"), &pal)?;
    let pal = Palette::load(&root.join("ra2md/ntrlmd/glsmd.pal"))?;
    shp2png(&root.join("ra2md/ntrlmd/glslmd.shp"), &pal)?;
    Ok(())
}

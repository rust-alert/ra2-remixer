//! Basic example of using RA2 MIX library

use ra2_pal::Palette;
use ra2_types::Ra2Error;
use std::path::Path;

fn main() -> Result<(), Ra2Error> {
    let root = Path::new("E:\\Games\\Red Alert 2 - Yuri's Revenge");
    let _ = Palette::load(&root.join("ra2/cache/unittem.pal"))?;
    Ok(())
}

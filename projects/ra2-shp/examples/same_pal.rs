//! Example of extracting files from RA2 MIX archives

use ra2_shp::shp_with_pal;
use ra2_types::Ra2Error;
use std::path::Path;

fn main() -> Result<(), Ra2Error> {
    let root = Path::new("E:\\Games\\Red Alert 2 - Yuri's Revenge");
    shp_with_pal(root)?;
    Ok(())
}

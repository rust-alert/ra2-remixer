use ra2_mix::{MixDatabase, decompress};
use ra2_types::Ra2Error;
use std::path::Path;

fn main() -> Result<(), Ra2Error> {
    let db = MixDatabase::load(Path::new("C:\\Program Files (x86)\\XCC\\Utilities\\global mix database.dat"))?;
    decompress(Path::new("E:\\Games\\Red Alert 2 - Yuri's Revenge"), &db)?;
    Ok(())
}

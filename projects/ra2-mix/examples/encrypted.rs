//! Example of working with encrypted RA2 MIX files

use ra2_mix::{MixDatabase, MixPackage, Ra2Error};
use std::path::Path;

fn main() -> Result<(), Ra2Error> {
    let dat = include_bytes!(r#"C:\Program Files (x86)\XCC\Utilities\global mix database.dat"#);
    let db = MixDatabase::decode(dat)?;
    let _ = db.save(Path::new(r#"C:\Program Files (x86)\XCC\Utilities\global mix database.toml"#))?;

    // Load an encrypted MIX file
    let mix = MixPackage::load(Path::new("E:\\Games\\Red Alert 2 - Yuris Revenge\\ra2.mix"), &db)?;

    // Check if the file is encrypted
    if mix.files.is_empty() {
        println!("Failed to decrypt MIX file");
    }
    else {
        println!("Successfully decrypted MIX file with {} files", mix.files.len());
    }
    mix.dump(Path::new("E:\\Games\\Red Alert 2 - Yuris Revenge\\ra2"))?;
    Ok(())
}

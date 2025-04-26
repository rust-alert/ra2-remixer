//! Example of working with encrypted RA2 MIX files

use ra2_mix::{MixError, MixPackage};
use std::path::Path;
use ra2_types::MixError;

fn main() -> Result<(), MixError> {
    // Load an encrypted MIX file
    let mix = MixPackage::load(Path::new("E:\\RTS\\Mental Omega\\expandmo99.mix"))?;
    
    // Check if the file is encrypted
    if mix.files.is_empty() {
        println!("Failed to decrypt MIX file");
    } else {
        println!("Successfully decrypted MIX file with {} files", mix.files.len());
    }
    
    Ok(())
}
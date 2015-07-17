//! Example of working with encrypted RA2 MIX files

use ra2_mix::{MixError, MixPackage};
use std::path::Path;

fn main() -> Result<(), MixError> {
    // Load an encrypted MIX file
    let mix = MixPackage::load(Path::new("encrypted.mix"))?;
    
    // Check if the file is encrypted
    if mix.files.is_empty() {
        println!("Failed to decrypt MIX file");
    } else {
        println!("Successfully decrypted MIX file with {} files", mix.files.len());
    }
    
    Ok(())
}
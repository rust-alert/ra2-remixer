//! Basic example of using RA2 MIX library

use ra2_mix::{MixError, MixPackage};
use std::path::Path;

fn main() -> Result<(), MixError> {
    // Load a MIX file
    let mix = MixPackage::load(Path::new("example.mix"))?;
    
    // Print all files in the archive
    println!("Files in MIX archive:");
    for (filename, _) in mix.files.iter() {
        println!("- {}", filename);
    }
    
    Ok(())
}
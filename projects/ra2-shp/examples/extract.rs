//! Example of extracting files from RA2 MIX archives

use ra2_mix::{MixError, MixPackage};
use std::path::Path;
use std::fs;
use ra2_types::Ra2Error;

fn main() -> Result<(), Ra2Error> {
    // Load a MIX file
    let mix = MixPackage::load(Path::new("example.mix"))?;
    
    // Create output directory
    fs::create_dir_all("extracted")?;
    
    // Extract all files
    for (filename, data) in mix.files.iter() {
        let output_path = Path::new("extracted").join(filename);
        fs::write(output_path, data)?;
        println!("Extracted: {}", filename);
    }
    
    Ok(())
}
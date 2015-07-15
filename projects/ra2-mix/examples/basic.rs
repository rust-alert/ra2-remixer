//! Basic example of using RA2 MIX library

use ra2_mix::XccPackage;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load a MIX file
    let mix = XccPackage::load(Path::new("example.mix"))?;
    
    // Print all files in the archive
    println!("Files in MIX archive:");
    for (filename, _) in mix.files.iter() {
        println!("- {}", filename);
    }
    
    Ok(())
}
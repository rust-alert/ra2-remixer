//! Checksum and filename obfuscation utilities for RA2 MIX files

use std::convert::TryInto;

/// Obfuscates a filename according to RA2 MIX format rules
/// 
/// # Arguments
/// * `filename` - The filename to obfuscate
/// 
/// # Returns
/// The obfuscated filename
pub fn obfuscate_filename(filename: &str) -> String {
    let filename_length = filename.len();
    let salt = filename_length & 0xFFFFFFFC;
    
    let mut obfuscated_name = filename.to_uppercase();
    
    if filename_length & 3 != 0 {
        let fill_char = (filename_length - salt) as u8 as char;
        obfuscated_name.push(fill_char);
        
        let fill_count = 3 - (filename_length & 3);
        for _ in 0..fill_count {
            let salt_char = obfuscated_name.chars().nth(salt).unwrap_or('A');
            obfuscated_name.push(salt_char);
        }
    }
    
    obfuscated_name
}

/// Calculates the CRC32 checksum for a filename in RA2 MIX format
/// 
/// # Arguments
/// * `filename` - The filename to calculate CRC for
/// 
/// # Returns
/// The CRC32 checksum as a signed 32-bit integer
pub fn ra2_crc(filename: &str) -> i32 {
    let obfuscated_name = obfuscate_filename(filename);
    let binary_data = obfuscated_name.as_bytes();
    
    // Calculate CRC32
    let crc = crc32fast::hash(binary_data);
    
    // Convert to signed 32-bit integer for proper sorting in mix file
    crc as i32
}
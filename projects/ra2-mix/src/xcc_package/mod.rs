use crate::{
    MixError, XccGame,
    checksum::ra2_crc,
    constants::*,
    crypto::{decrypt_blowfish_key, decrypt_mix_header, get_decryption_block_sizing},
};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::{
    collections::HashMap,
    fs::File,
    io::{Seek, SeekFrom, Write},
    path::{Path, PathBuf},
};

pub mod reader;
pub mod writer;

#[derive(Debug)]
pub struct XccPackage {
    pub game: XccGame,
    pub files: HashMap<String, Vec<u8>>,
}

/// MIX file header
#[derive(Copy, Debug, Clone)]
pub struct Header {
    /// Flags (None for old format)
    pub flags: Option<u32>,
    /// Number of files in the MIX
    pub file_count: u16,
    /// Total size of file data
    pub data_size: u32,
}

/// MIX file entry
#[derive(Debug, Clone, Copy)]
pub struct FileEntry {
    /// File ID (CRC of filename)
    pub id: i32,
    /// Offset in the body data
    pub offset: u32,
    /// Size of the file
    pub size: u32,
}
/// File information for MIX file creation
#[derive(Debug, Clone)]
struct FileInfo {
    /// File ID (CRC of filename)
    file_id: i32,
    /// File data
    data: Vec<u8>,
}

impl Default for XccPackage {
    fn default() -> Self {
        Self { game: XccGame::RA2, files: Default::default() }
    }
}

impl XccPackage {
    /// Add any file to the MIX package, no matter if it is valid or not.
    ///
    /// # Arguments
    ///
    /// * `name`: the file name with extension
    /// * `data`: the file bytes
    ///
    /// # Examples
    ///
    /// ```
    /// let mut mix = ra2_mix::XccPackage::default();
    /// mix.add_any("hello.txt".to_string(), b"Hello, World!".to_vec());
    /// ```
    pub fn add_any(&mut self, name: String, data: Vec<u8>) {
        self.files.insert(name, data);
    }
}

/// Extract single file from the MIX file to a folder
///
/// # Arguments
///
/// * `input`:
/// * `output`:
///
/// returns: Result<(), MixError>
///
/// # Examples
///
/// ```
/// ```
pub fn extract(input: &Path, output: &Path) -> Result<(), MixError> {
    let xcc = XccPackage::load(input)?;
    let file_map = xcc.files;
    std::fs::create_dir_all(output)?;
    for (filename, file_data) in file_map {
        let file_path = output.join(filename);
        let mut file = File::create(file_path)?;
        file.write_all(&file_data)?;
    }

    Ok(())
}
/// Patch a folder into the MIX file
///
/// # Arguments
///
/// * `input`:
/// * `output`:
///
/// returns: Result<(), MixError>
///
/// # Examples
///
/// ```
/// ```
pub fn patch(input: &Path, output: &Path) -> Result<(), MixError> {
    let data = std::fs::read(input)?;

    // let file_map = decrypt(&data)?;
    //
    // let folder = folder_path;
    // std::fs::create_dir_all(folder)?;
    //
    // for (filename, file_data) in file_map {
    //     let file_path = folder.join(filename);
    //     let mut file = File::create(file_path)?;
    //     file.write_all(&file_data)?;
    // }

    Ok(())
}

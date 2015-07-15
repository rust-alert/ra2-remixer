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
    path::{Path},
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

    /// Add a file from filesystem to the package
///
/// # Arguments
/// * `data` - Path to the file to add
///
/// # Returns
/// Size of the added file in bytes on success, or error if file not found
///
/// # Examples
/// ```no_run
/// use ra2_mix::XccPackage;
/// use std::path::Path;
///
/// let mut package = XccPackage::default();
/// package.add_file(Path::new("test.txt")).unwrap();
/// ```
pub fn add_file(&mut self, data: &Path) -> Result<usize, MixError> {
        if !data.is_file() {
            return Err(MixError::FileNotFound("".to_string()));
        }
        let name = data.file_name().and_then(|s| s.to_str()).ok_or(MixError::FileNotFound("".to_string()))?;
        let data = std::fs::read(data)?;
        let size = data.len();
        self.files.insert(name.to_string(), data);
        Ok(size)
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
    let mut xcc = XccPackage::load(input)?;
    for entry in std::fs::read_dir(input)? {
        let entry = entry?;
        xcc.add_file(&entry.path())?;
    }
    xcc.save(output)?;
    Ok(())
}

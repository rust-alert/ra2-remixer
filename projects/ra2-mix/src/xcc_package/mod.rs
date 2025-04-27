use crate::{Ra2Error, CncGame, checksum::ra2_crc, constants::*, crypto::{decrypt_blowfish_key, decrypt_mix_header, get_decryption_block_sizing}, MixDatabase};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::{
    collections::HashMap,
    fs::File,
    io::{Seek, SeekFrom, Write},
    path::Path,
};

pub mod reader;
pub mod writer;

/// MIX package
#[derive(Default, Debug)]
pub struct MixPackage {
    /// The game version of the MIX package
    pub game: CncGame,
    /// A map of file names to file data
    pub files: HashMap<String, Vec<u8>>,
}



/// MIX file header
#[derive(Copy, Debug, Clone)]
struct MixHeader {
    /// Flags (None for old format)
    pub flags: Option<u32>,
    /// Number of files in the MIX
    pub file_count: u16,
    /// Total size of file data
    pub data_size: u32,
}

/// MIX file entry
#[derive(Debug, Clone, Copy)]
struct FileEntry {
    /// File ID (CRC of filename)
    pub id: u32,
    /// Offset in the body data
    pub offset: i32,
    /// Size of the file
    pub size: i32,
}
/// File information for MIX file creation
#[derive(Debug, Clone)]
struct FileInfo {
    /// File ID (CRC of filename)
    file_id: u32,
    /// File data
    data: Vec<u8>,
}

impl MixPackage {
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
    /// let mut mix = ra2_mix::MixPackage::default();
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
    /// use ra2_mix::MixPackage;
    /// use std::path::Path;
    ///
    /// let mut package = MixPackage::default();
    /// package.add_file(Path::new("test.txt")).unwrap();
    /// ```
    pub fn add_file(&mut self, data: &Path) -> Result<usize, Ra2Error> {
        if !data.is_file() {
            return Err(Ra2Error::FileNotFound("must file".to_string()));
        }
        let name = data.file_name().and_then(|s| s.to_str()).ok_or(Ra2Error::FileNotFound("".to_string()))?;
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
pub fn extract(input: &Path, output: &Path) -> Result<(), Ra2Error> {
    let xcc = MixPackage::load(input, &MixDatabase::default())?;
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
pub fn patch(input: &Path, output: &Path) -> Result<(), Ra2Error> {
    let mut xcc = MixPackage::load(input, &MixDatabase::default())?;
    for entry in std::fs::read_dir(input)? {
        let entry = entry?;
        xcc.add_file(&entry.path())?;
    }
    xcc.save(output)?;
    Ok(())
}

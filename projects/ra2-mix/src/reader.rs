//! Reader module for RA2 MIX files

use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path};

use byteorder::{LittleEndian, ReadBytesExt};

use crate::checksum::ra2_crc;
use crate::constants::*;
use crate::crypto::{decrypt_blowfish_key, decrypt_mix_header, get_decryption_block_sizing};
use crate::MixError;

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

/// Checks if a MIX header is encrypted
fn header_is_encrypted(header: &Header) -> bool {
    header.flags.is_some() && (header.flags.unwrap() & 0x20000) != 0
}

/// Parses file entries from index data
fn get_file_entries(file_count: usize, index_data: &[u8]) -> Result<Vec<FileEntry>, MixError> {
    let mut file_entries = Vec::with_capacity(file_count);
    let mut cursor = std::io::Cursor::new(index_data);
    
    for _ in 0..file_count {
        let id = cursor.read_i32::<LittleEndian>()?;
        let offset = cursor.read_u32::<LittleEndian>()?;
        let size = cursor.read_u32::<LittleEndian>()?;
        
        file_entries.push(FileEntry { id, offset, size });
    }
    
    Ok(file_entries)
}

/// Extracts filenames from a MIX database file
fn get_filenames_from_mix_db(mix_db_file_data: &[u8]) -> Vec<String> {
    let mut filenames = Vec::new();
    let mut start = XCC_HEADER_SIZE;
    
    while start < mix_db_file_data.len() {
        let mut end = start;
        while end < mix_db_file_data.len() && mix_db_file_data[end] != 0 {
            end += 1;
        }
        
        if start < end {
            if let Ok(filename) = std::str::from_utf8(&mix_db_file_data[start..end]) {
                filenames.push(filename.to_string());
            }
        }
        
        end += 1; // Skip the null terminator
        start = end;
    }
    
    filenames
}

/// Extracts file data from MIX body
fn get_file_data_from_mix_body(file_entry: &FileEntry, mix_body_data: &[u8]) -> Vec<u8> {
    let start = file_entry.offset as usize;
    let end = start + file_entry.size as usize;
    
    if end <= mix_body_data.len() {
        mix_body_data[start..end].to_vec()
    } else {
        Vec::new()
    }
}

/// Loads the global mix database
#[cfg(feature = "serde-support")]
pub fn load_global_mix_database() -> Result<HashMap<String, i32>, MixError> {
    // In a real implementation, this would load from an embedded resource
    // For now, we'll return an empty map
    Ok(HashMap::new())
}

/// Reads file information from a MIX file
pub fn read_file_info<P: AsRef<Path>>(mix_filepath: Option<P>, mix_data: Option<&[u8]>) 
    -> Result<(Header, Vec<FileEntry>, Vec<u8>), MixError> {
    
    if mix_filepath.is_none() && mix_data.is_none() {
        return Err(MixError::InvalidArgument("Must specify either mix_filepath or mix_data".to_string()));
    }
    
    if mix_filepath.is_some() && mix_data.is_some() {
        return Err(MixError::InvalidArgument("Cannot specify both mix_filepath and mix_data".to_string()));
    }
    
    // 创建一个拥有所有权的变量来存储文件内容
    let mut owned_data;
    
    // 确定数据来源
    let mix_data_ref = if let Some(data) = mix_data {
        data
    } else {
        let mut file = File::open(mix_filepath.unwrap())?;
        owned_data = Vec::new();
        file.read_to_end(&mut owned_data)?;
        &owned_data
    };
    
    let mut cursor = std::io::Cursor::new(mix_data_ref);
    
    // Check if this is an old format MIX file
    let first_word = cursor.read_u16::<LittleEndian>()?;
    cursor.seek(SeekFrom::Start(0))?; // Reset cursor position
    
    let header: Header;
    let header_size: usize;
    
    if first_word != 0 {
        // Old format
        let count = cursor.read_u16::<LittleEndian>()?;
        let size = cursor.read_u32::<LittleEndian>()?;
        header = Header {
            flags: None,
            file_count: count,
            data_size: size,
        };
        header_size = MIN_HEADER_SIZE;
    } else {
        // New format
        let flags = cursor.read_u32::<LittleEndian>()?;
        let count = cursor.read_u16::<LittleEndian>()?;
        let size = cursor.read_u32::<LittleEndian>()?;
        header = Header {
            flags: Some(flags),
            file_count: count,
            data_size: size,
        };
        header_size = HEADER_SIZE;
    }
    
    let file_entries: Vec<FileEntry>;
    let mut updated_header = header.clone();
    
    if header_is_encrypted(&header) {
        // Handle encrypted header
        let encrypted_key_start = SIZE_OF_FLAGS;
        let encrypted_key_end = encrypted_key_start + SIZE_OF_ENCRYPTED_KEY;
        
        let encrypted_blowfish_key = &mix_data_ref[encrypted_key_start..encrypted_key_end];
        let decrypted_blowfish_key = decrypt_blowfish_key(encrypted_blowfish_key)?;
        
        let (file_count, data_size, index_data) = decrypt_mix_header(mix_data_ref, &decrypted_blowfish_key)?;
        
        file_entries = get_file_entries(file_count as usize, &index_data)?;
        updated_header.file_count = file_count;
        updated_header.data_size = data_size;
    } else {
        // Handle unencrypted header
        let index_start = header_size;
        let index_end = index_start + (header.file_count as usize * FILE_ENTRY_SIZE);
        
        if index_end > mix_data_ref.len() {
            return Err(MixError::InvalidFormat("File too small for index".to_string()));
        }
        
        let index_data = &mix_data_ref[index_start..index_end];
        file_entries = get_file_entries(header.file_count as usize, index_data)?;
    }
    
    // Convert mix_data_ref to owned Vec<u8>
    let mix_data_vec = mix_data_ref.to_vec();
    
    Ok((updated_header, file_entries, mix_data_vec))
}

/// Creates a file map from file entries and mix data
pub fn get_file_map(file_entries: &[FileEntry], mix_data: &[u8], header: &Header) 
    -> Result<HashMap<String, Vec<u8>>, MixError> {
    
    if file_entries.len() <= 1 {
        return Ok(HashMap::new());
    }
    
    let mix_db_id = ra2_crc(MIX_DB_FILENAME);
    
    // Calculate body start position
    let mut body_start = if header.flags.is_none() {
        MIN_HEADER_SIZE
    } else {
        HEADER_SIZE
    } + (FILE_ENTRY_SIZE * file_entries.len());
    
    if header_is_encrypted(header) {
        body_start += SIZE_OF_ENCRYPTED_KEY;
        body_start += get_decryption_block_sizing(header.file_count).1;
    }
    
    // Find local mix database if it exists
    let mut local_mix_db_file_entry = None;
    for entry in file_entries {
        if entry.id == mix_db_id {
            local_mix_db_file_entry = Some(*entry);
            break;
        }
    }
    
    let mix_body_data = &mix_data[body_start..];
    
    // Get filename to ID mapping
    let id_filename_map: HashMap<i32, String>;
    
    if let Some(db_entry) = local_mix_db_file_entry {
        // Use local mix database
        let local_mix_db_data = get_file_data_from_mix_body(&db_entry, mix_body_data);
        let filenames = get_filenames_from_mix_db(&local_mix_db_data);
        
        id_filename_map = filenames.iter()
            .map(|filename| (ra2_crc(filename), filename.clone()))
            .collect();
    } else {
        // Use global mix database
        #[cfg(feature = "serde-support")]
        {
            let global_db = load_global_mix_database()?;
            id_filename_map = global_db.iter()
                .map(|(filename, id)| (*id, filename.clone()))
                .collect();
        }
        
        #[cfg(not(feature = "serde-support"))]
        {
            // Without serde support, we can't load the global database
            // Return an empty map
            id_filename_map = HashMap::new();
        }
    }
    
    // Create file map
    let mut filemap = HashMap::new();
    
    for entry in file_entries {
        let file_data = get_file_data_from_mix_body(entry, mix_body_data);
        
        if let Some(filename) = id_filename_map.get(&entry.id) {
            filemap.insert(filename.clone(), file_data);
        }
    }
    
    Ok(filemap)
}

/// Reads a MIX file and returns a map of filenames to file data
pub fn read<P: AsRef<Path>>(mix_filepath: Option<P>, mix_data: Option<&[u8]>) 
    -> Result<HashMap<String, Vec<u8>>, MixError> {
    
    if mix_filepath.is_none() && mix_data.is_none() {
        return Err(MixError::InvalidArgument("Must specify either mix_filepath or mix_data".to_string()));
    }
    
    if mix_filepath.is_some() && mix_data.is_some() {
        return Err(MixError::InvalidArgument("Cannot specify both mix_filepath and mix_data".to_string()));
    }
    
    let (header, file_entries, mix_data_vec) = read_file_info(mix_filepath, mix_data)?;
    
    get_file_map(&file_entries, &mix_data_vec, &header)
}

/// Extracts a MIX file to a folder
pub fn extract<P: AsRef<Path>, Q: AsRef<Path>>(mix_filepath: P, folder_path: Q) -> Result<(), MixError> {
    let file_map = read(Some(mix_filepath), None)?;
    
    let folder = folder_path.as_ref();
    std::fs::create_dir_all(folder)?;
    
    for (filename, file_data) in file_map {
        let file_path = folder.join(filename);
        let mut file = File::create(file_path)?;
        file.write_all(&file_data)?;
    }
    
    Ok(())
}
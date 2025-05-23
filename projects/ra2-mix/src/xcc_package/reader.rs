//! Reader module for RA2 MIX files

use super::*;

impl MixPackage {
    /// # Arguments
    ///
    /// * `input`:
    ///
    /// returns: Result<XccPackage, MixError>
    ///
    /// # Examples
    ///
    /// ```
    /// ```
    pub fn load(mix_path: &Path) -> Result<Self, MixError> {
        let data = std::fs::read(mix_path)?;
        MixPackage::decode(&data)
    }
    /// Reads a MIX file and returns a map of filenames to file data
    ///
    /// # Arguments
    ///
    /// * `input`:
    ///
    /// returns: Result<XccPackage, MixError>
    ///
    /// # Examples
    ///
    /// ```
    /// ```
    pub fn decode(mix_data: &[u8]) -> Result<Self, MixError> {
        let (header, file_entries, mix_data_vec) = read_file_info(mix_data)?;
        let map = get_file_map(&file_entries, &mix_data_vec, &header)?;
        Ok(Self { game: Default::default(), files: map })
    }
}

/// Checks if a MIX header is encrypted
fn header_is_encrypted(header: &MixHeader) -> bool {
    header.flags.is_some() && (header.flags.unwrap() & 0x20000) != 0
}

/// Parses file entries from index data
fn get_file_entries(file_count: usize, index_data: &[u8]) -> Result<Vec<FileEntry>, MixError> {
    let mut file_entries = Vec::with_capacity(file_count);
    let mut cursor = std::io::Cursor::new(index_data);

    for _ in 0..file_count {
        let id = cursor.read_i32::<LittleEndian>()?;
        let offset = cursor.read_i32::<LittleEndian>()?;
        let size = cursor.read_i32::<LittleEndian>()?;
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

    if end <= mix_body_data.len() { mix_body_data[start..end].to_vec() } else { Vec::new() }
}

/// Loads the global mix database
#[cfg(feature = "serde_json")]
pub fn load_global_mix_database() -> Result<HashMap<String, i32>, MixError> {
    // In a real implementation, this would load from an embedded resource
    // For now, we'll return an empty map
    Ok(HashMap::new())
}

/// Reads file information from a MIX file
fn read_file_info(mix_data: &[u8]) -> Result<(MixHeader, Vec<FileEntry>, Vec<u8>), MixError> {
    let mut cursor = std::io::Cursor::new(mix_data);

    // Check if this is an old format MIX file
    let first_word = cursor.read_u16::<LittleEndian>()?;
    cursor.seek(SeekFrom::Start(0))?; // Reset cursor position

    let header: MixHeader;
    let header_size: usize;

    if first_word != 0 {
        // Old format
        let count = cursor.read_u16::<LittleEndian>()?;
        let size = cursor.read_u32::<LittleEndian>()?;
        header = MixHeader { flags: None, file_count: count, data_size: size };
        header_size = MIN_HEADER_SIZE;
    }
    else {
        // New format
        let flags = cursor.read_u32::<LittleEndian>()?;
        let count = cursor.read_u16::<LittleEndian>()?;
        let size = cursor.read_u32::<LittleEndian>()?;
        header = MixHeader { flags: Some(flags), file_count: count, data_size: size };
        header_size = HEADER_SIZE;
    }

    let file_entries: Vec<FileEntry>;
    let mut updated_header = header.clone();

    if header_is_encrypted(&header) {
        // Handle encrypted header
        let encrypted_key_start = SIZE_OF_FLAGS;
        let encrypted_key_end = encrypted_key_start + SIZE_OF_ENCRYPTED_KEY;

        let encrypted_blowfish_key = &mix_data[encrypted_key_start..encrypted_key_end];
        let decrypted_blowfish_key = decrypt_blowfish_key(encrypted_blowfish_key)?;

        let (file_count, data_size, index_data) = decrypt_mix_header(mix_data, &decrypted_blowfish_key)?;

        file_entries = get_file_entries(file_count as usize, &index_data)?;
        updated_header.file_count = file_count;
        updated_header.data_size = data_size;
    }
    else {
        // Handle unencrypted header
        let index_start = header_size;
        let index_end = index_start + (header.file_count as usize * FILE_ENTRY_SIZE);

        if index_end > mix_data.len() {
            return Err(MixError::InvalidFormat("File too small for index".to_string()));
        }

        let index_data = &mix_data[index_start..index_end];
        file_entries = get_file_entries(header.file_count as usize, index_data)?;
    }

    // Convert mix_data_ref to owned Vec<u8>
    let mix_data_vec = mix_data.to_vec();

    Ok((updated_header, file_entries, mix_data_vec))
}

/// Creates a file map from file entries and mix data
fn get_file_map(file_entries: &[FileEntry], mix_data: &[u8], header: &MixHeader) -> Result<HashMap<String, Vec<u8>>, MixError> {
    if file_entries.len() <= 1 {
        return Ok(HashMap::new());
    }

    let mix_db_id = ra2_crc(MIX_DB_FILENAME);
    debug_assert_eq!(mix_db_id, 0x366E051F);

    // Calculate body start position
    let mut body_start =
        if header.flags.is_none() { MIN_HEADER_SIZE } else { HEADER_SIZE } + (FILE_ENTRY_SIZE * file_entries.len());

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
        if db_entry.offset < 0 { 
            return Err(MixError::InvalidFormat("This `mix` file is protected".to_string()));
        }
        // Use local mix database
        let local_mix_db_data = get_file_data_from_mix_body(&db_entry, mix_body_data);
        let filenames = get_filenames_from_mix_db(&local_mix_db_data);

        id_filename_map = filenames.iter().map(|filename| (ra2_crc(filename), filename.clone())).collect();
    }
    else {
        println!("No local mix database found, please add global mix database");
        // Use global mix database
        #[cfg(feature = "serde_json")]
        {
            let global_db = load_global_mix_database()?;
            id_filename_map = global_db.iter().map(|(filename, id)| (*id, filename.clone())).collect();
        }

        #[cfg(not(feature = "serde_json"))]
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

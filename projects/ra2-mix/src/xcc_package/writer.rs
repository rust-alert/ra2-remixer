//! Writer module for RA2 MIX files

use super::*;

impl MixPackage {
    /// # Arguments
    ///
    /// * `output`:
    ///
    /// # Examples
    ///
    /// ```
    /// ```
    pub fn save(self, output: &Path) -> Result<usize, Ra2Error> {
        let data = self.encode()?;
        std::fs::write(output, &data)?;
        Ok(data.len())
    }
    /// # Arguments
    ///
    /// * `output`:
    ///
    /// # Examples
    ///
    /// ```
    /// ```
    pub fn encode(self) -> Result<Vec<u8>, Ra2Error> {
        let file_map = coalesce_input_files(self.game, &self.files)?;

        // Create file information list
        let mut file_information_list: Vec<FileInfo> =
            file_map.iter().map(|(filename, data)| FileInfo { file_id: ra2_crc(filename), data: data.clone() }).collect();

        // Sort by file ID
        file_information_list.sort_by_key(|file_info| file_info.file_id);

        // Generate file entries and body data
        let mut offset = 0u32;
        let mut file_entry_data = Vec::new();
        let mut body_data = Vec::new();

        for file_info in &file_information_list {
            let size = file_info.data.len() as u32;

            // Write file entry
            file_entry_data.write_i32::<LittleEndian>(file_info.file_id)?;
            file_entry_data.write_u32::<LittleEndian>(offset)?;
            file_entry_data.write_u32::<LittleEndian>(size)?;

            // Write file data
            body_data.extend_from_slice(&file_info.data);

            offset += size;
        }

        // Combine all parts
        let mut mix_data = create_mix_header(&file_map)?;
        mix_data.extend_from_slice(&file_entry_data);
        mix_data.extend_from_slice(&body_data);

        Ok(mix_data)
    }
}

/// Creates MIX database data
fn get_mix_db_data(filenames: &[String], game: CncGame) -> Vec<u8> {
    let num_files = filenames.len();
    let db_size_in_bytes = XCC_HEADER_SIZE + filenames.iter().map(|filename| filename.len() + 1).sum::<usize>();

    let mut bytes_data = Vec::with_capacity(db_size_in_bytes);

    // Write XCC ID bytes
    bytes_data.extend_from_slice(XCC_ID_BYTES);
    // Pad to 32 bytes
    bytes_data.resize(32, 0);

    // Write header fields
    bytes_data.write_u32::<LittleEndian>(db_size_in_bytes as u32).unwrap();
    bytes_data.write_u32::<LittleEndian>(XCC_FILE_TYPE).unwrap();
    bytes_data.write_u32::<LittleEndian>(XCC_FILE_VERSION).unwrap();
    bytes_data.write_u32::<LittleEndian>(game as u32).unwrap();
    bytes_data.write_u32::<LittleEndian>(num_files as u32).unwrap();

    // Write filenames with null terminators
    for filename in filenames {
        bytes_data.extend_from_slice(filename.as_bytes());
        bytes_data.push(0); // Null terminator
    }

    bytes_data
}

/// Processes input files and creates a file map
pub fn coalesce_input_files(game: CncGame, file_map: &HashMap<String, Vec<u8>>) -> Result<HashMap<String, Vec<u8>>, Ra2Error> {
    let mut extra_file_map = file_map.clone();
    // Get filenames and create mix database
    let mut filenames: Vec<String> = extra_file_map.keys().cloned().collect();
    filenames.push(MIX_DB_FILENAME.to_string());
    // Sort filenames
    let db_data = get_mix_db_data(&filenames, game);
    extra_file_map.insert(MIX_DB_FILENAME.to_string(), db_data);
    Ok(extra_file_map)
}

/// Creates a MIX file header
fn create_mix_header(file_map: &HashMap<String, Vec<u8>>) -> Result<Vec<u8>, Ra2Error> {
    let flags = 0u32;
    let file_count = file_map.len() as u16;
    let data_size = file_map.values().map(|data| data.len() as u32).sum();

    let mut header = Vec::with_capacity(HEADER_SIZE);
    header.write_u32::<LittleEndian>(flags)?;
    header.write_u16::<LittleEndian>(file_count)?;
    header.write_u32::<LittleEndian>(data_size)?;

    Ok(header)
}

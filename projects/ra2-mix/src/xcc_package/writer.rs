//! Writer module for RA2 MIX files

use super::*;


impl XccPackage {
    /// 
    /// 
    /// # Arguments 
    /// 
    /// * `output`: 
    ///
    /// # Examples 
    /// 
    /// ```
    /// 
    /// ```
    pub fn save(self, output: &Path) -> Result<usize, MixError> {
        let data = self.encode()?;
        std::fs::write(output, &data)?;
        Ok(data.len())
    }
    ///
    ///
    /// # Arguments 
    ///
    /// * `output`: 
    ///
    /// # Examples 
    ///
    /// ```
    ///
    /// ```
    pub fn encode(self) -> Result<Vec<u8>, MixError> {
        let file_map = coalesce_input_files(self.game, Some(self.files), None::<&Path>, None)?;

        // Create file information list
        let mut file_information_list: Vec<FileInfo> = file_map
            .iter()
            .map(|(filename, data)| FileInfo {
                file_id: ra2_crc(filename),
                data: data.clone(),
            })
            .collect();

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
        let mut mix_data = create_mix_header(&file_map);
        mix_data.extend_from_slice(&file_entry_data);
        mix_data.extend_from_slice(&body_data);

        Ok(mix_data)
    }
}


/// Creates MIX database data
fn get_mix_db_data(filenames: &[String], game: XccGame) -> Vec<u8> {
    let num_files = filenames.len();
    let db_size_in_bytes = XCC_HEADER_SIZE + 
        filenames.iter().map(|filename| filename.len() + 1).sum::<usize>();
    
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
pub fn coalesce_input_files(
    game: XccGame,
    file_map: Option<HashMap<String, Vec<u8>>>,
    input_folder: Option<&Path>,
    filepaths: Option<Vec<PathBuf>>,
) -> Result<HashMap<String, Vec<u8>>, MixError> {
    // Validate input arguments
    let args_provided = [file_map.is_some(), input_folder.is_some(), filepaths.is_some()]
        .iter().filter(|&&x| x).count();
    
    if args_provided > 1 {
        return Err(MixError::InvalidArgument(
            "Must provide exactly one of the following args: file_map, input_folder, filepaths".to_string()
        ));
    }
    
    let mut result_file_map = if let Some(map) = file_map {
        map
    } else if let Some(folder) = input_folder {
        let folder_path = folder;
        if !folder_path.exists() {
            return Err(MixError::FileNotFound(format!("{:?} does not exist!", folder_path)));
        }
        
        let mut map = HashMap::new();
        for entry in std::fs::read_dir(folder_path)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() {
                if let Some(filename) = path.file_name() {
                    if let Some(filename_str) = filename.to_str() {
                        let data = std::fs::read(&path)?;
                        map.insert(filename_str.to_string(), data);
                    }
                }
            }
        }
        map
    } else if let Some(paths) = filepaths {
        let mut map = HashMap::new();
        for path in paths {
            if !path.exists() {
                return Err(MixError::FileNotFound(format!("{:?} does not exist!", path)));
            }
            
            if let Some(filename) = path.file_name() {
                if let Some(filename_str) = filename.to_str() {
                    let data = std::fs::read(&path)?;
                    map.insert(filename_str.to_string(), data);
                }
            }
        }
        map
    } else {
        return Err(MixError::InvalidArgument(
            "Must provide one of the following args: file_map, input_folder, filepaths".to_string()
        ));
    };
    
    // Get filenames and create mix database
    let mut filenames: Vec<String> = result_file_map.keys().cloned().collect();
    filenames.push(MIX_DB_FILENAME.to_string());
    
    let db_data = get_mix_db_data(&filenames, game);
    result_file_map.insert(MIX_DB_FILENAME.to_string(), db_data);
    
    // Log file types (commented out in production code)
    /*
    let mut filetypes = HashMap::new();
    for filename in result_file_map.keys() {
        if let Some(extension) = Path::new(filename).extension() {
            if let Some(ext_str) = extension.to_str() {
                *filetypes.entry(ext_str.to_string()).or_insert(0) += 1;
            }
        }
    }
    */
    
    Ok(result_file_map)
}

/// Creates a MIX file header
fn create_mix_header(file_map: &HashMap<String, Vec<u8>>) -> Vec<u8> {
    let flags = 0u32;
    let file_count = file_map.len() as u16;
    let data_size = file_map.values().map(|data| data.len() as u32).sum();
    
    let mut header = Vec::with_capacity(HEADER_SIZE);
    header.write_u32::<LittleEndian>(flags).unwrap();
    header.write_u16::<LittleEndian>(file_count).unwrap();
    header.write_u32::<LittleEndian>(data_size).unwrap();
    
    header
}

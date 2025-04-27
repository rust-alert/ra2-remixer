use crate::{checksum::ra2_crc, constants::XCC_HEADER_SIZE};
use ra2_types::Ra2Error;
use std::{collections::BTreeMap, path::Path};

mod reader;
mod writer;

/// The reverse of the MIX database
#[derive(Clone, Debug, Default)]
pub struct MixDatabase {
    map: BTreeMap<i32, String>,
}

impl MixDatabase {
    pub fn decode_dat(dat: &[u8]) -> Result<MixDatabase, Ra2Error> {
        let mut out = MixDatabase::default();
        let names = get_filenames_from_mix_db(dat);
        for name in names.into_iter() {
            out.add(name);
        }
        Ok(out)
    }
    #[cfg(feature = "serde_json")]
    pub fn decode_json(json: &str) -> Result<MixDatabase, Ra2Error> {
        Ok(Self { map: serde_json::from_str(json)? })
    }

    pub fn load(path: &Path) -> Result<MixDatabase, Ra2Error> {
        #[cfg(feature = "serde_json")]
        if path.extension().eq(Some("json")) {
            return Self::decode_json(&std::fs::read_to_string(path)?);
        }
        Ok(Self::decode_dat(&std::fs::read(path)?)?)
    }

    #[cfg(feature = "serde_json")]
    pub fn encode_json(self) -> Result<String, Ra2Error> {
        Ok(serde_json::to_string(&self.map)?)
    }
}

impl MixDatabase {
    pub fn get(&self, crc_id: i32) -> Option<&String> {
        self.map.get(&crc_id)
    }
    pub fn add(&mut self, filename: String) {
        self.map.insert(ra2_crc(&filename), filename);
    }
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

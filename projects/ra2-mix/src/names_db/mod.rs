use crate::{checksum::ra2_crc, constants::XCC_HEADER_SIZE};
use ra2_types::Ra2Error;
use std::{collections::BTreeMap, ffi::OsStr, io::Write, path::Path};

mod reader;
mod writer;

/// The reverse of the MIX database
#[derive(Clone, Debug, Default)]
pub struct MixDatabase {
    map: BTreeMap<u32, String>,
}

impl MixDatabase {
    pub fn decode(dat: &[u8]) -> Result<MixDatabase, Ra2Error> {
        let mut out = MixDatabase::default();
        let names = get_filenames_from_mix_db(dat);
        names.into_iter().for_each(|name| out.add(name));
        Ok(out)
    }
    pub fn load(path: &Path) -> Result<MixDatabase, Ra2Error> {
        match path.extension() {
            #[cfg(feature = "toml")]
            Some(s) if s.eq("toml") => {
                let text = std::fs::read_to_string(path)?;
                Ok(Self { map: toml::from_str(&text)? })
            }
            _ => Ok(Self::decode(&std::fs::read(path)?)?),
        }
    }
}

impl MixDatabase {
    #[cfg(feature = "serde_json")]
    pub fn encode_json(self) -> Result<String, Ra2Error> {
        Ok(serde_json::to_string(&self.map)?)
    }

    pub fn save(&self, path: &Path) -> Result<(), Ra2Error> {
        match path.extension() {
            #[cfg(feature = "toml")]
            Some(s) if s.eq("toml") => {
                let mut file = std::fs::File::create(path)?;
                for (crc_id, filename) in &self.map {
                    writeln!(file, "{} = {:?}", crc_id, filename)?;
                }
            }
            _ => {}
        }
        Ok(())
    }
}

impl MixDatabase {
    pub fn get(&self, crc_id: u32) -> Option<&String> {
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

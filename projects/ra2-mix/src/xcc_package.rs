use std::collections::HashMap;
use std::path::Path;
use crate::{decrypt, encrypt, MixError, XccGame};

#[derive(Debug)]
pub struct XccPackage {
    pub file_map: HashMap<String, Vec<u8>>,
    mix_data: Vec<u8>,
}

impl Default for XccPackage {
    fn default() -> Self {
        Self {
            file_map: Default::default(),
            mix_data: vec![],
        }
    }
}

impl XccPackage {
    pub fn add_file(&mut self, filename: String, data: Vec<u8>) {
        self.file_map.insert(filename.to_string(), data);
    }
    pub fn dump(self, output: &Path) -> Result<usize, MixError> {
        let data = encrypt(            XccGame::RA2,            Some(self.file_map),            None::<&Path>,            None,        )?;
        std::fs::write(output, &data)?;
        Ok(data.len())
    }
    pub fn load(input: &Path) -> Result<Self, MixError> {
        let data = std::fs::read(input)?;
        Ok(Self {
            file_map: decrypt(&data)?,
            mix_data: vec![],
        })
    }
}
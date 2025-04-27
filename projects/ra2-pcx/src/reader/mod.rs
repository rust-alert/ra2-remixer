use std::path::Path;
use ra2_types::Ra2Error;
use crate::{Palette, Ra2Color};

impl Palette {
    /// Load PAL file
    ///
    /// # Arguments 
    ///
    /// * `path`: 
    ///
    /// returns: Result<Palette, Ra2Error> 
    ///
    /// # Examples 
    ///
    /// ```
    ///
    /// ```
    pub fn load(path: &Path) -> Result<Self, Ra2Error> {
        let bytes = std::fs::read(path)?;
        Self::decode(&bytes)
    }

    ///
    ///
    /// # Arguments 
    ///
    /// * `bytes`: 
    ///
    /// returns: Result<Palette, Ra2Error> 
    ///
    /// # Examples 
    ///
    /// ```
    ///
    /// ```
    pub fn decode(bytes: &[u8]) -> Result<Self, Ra2Error> {
        if bytes.len() != 256 * 3 {
            return Err(Ra2Error::InvalidFormat {
                message: "字节数组长度不正确，PAL 文件应为 256 * 3 字节".to_string()
            });
        }

        let mut colors: [Ra2Color; 256] = [Ra2Color { red: 0, green: 0, blue: 0 }; 256];

        for i in 0..256 {
            colors[i].red = bytes[i * 3];
            colors[i].green = bytes[i * 3 + 1];
            colors[i].blue = bytes[i * 3 + 2];
        }

        Ok(Palette { colors })
    }
}

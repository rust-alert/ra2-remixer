//! Constants used in RA2 MIX file format

/// Block size for encryption/decryption
pub const BLOCK_SIZE: usize = 8;
/// Minimum header size for MIX files
pub const MIN_HEADER_SIZE: usize = 6;
/// Standard header size for MIX files
pub const HEADER_SIZE: usize = 10;
/// Size of each file entry in the MIX file
pub const FILE_ENTRY_SIZE: usize = 12;
/// Size of flags field in the header
pub const SIZE_OF_FLAGS: usize = 4;
/// Size of file count field in the header
pub const SIZE_OF_FILE_COUNT: usize = 2;
/// Size of data size field in the header
pub const SIZE_OF_DATA_SIZE: usize = 4;
/// Size of encrypted key in the header
pub const SIZE_OF_ENCRYPTED_KEY: usize = 80;

/// Name of the local mix database file
pub const MIX_DB_FILENAME: &str = "local mix database.dat";

/// Size of XCC header
pub const XCC_HEADER_SIZE: usize = 52;
/// XCC file type
pub const XCC_FILE_TYPE: u32 = 0;
/// XCC file version
pub const XCC_FILE_VERSION: u32 = 0;

/// XCC ID bytes
pub const XCC_ID_BYTES: &[u8] = b"XCC by Olaf van der Spek\x1a\x04\x17\x27\x10\x19\x80";

/// Enum representing different games that use XCC format
#[allow(non_camel_case_types)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XCCGame {
    /// Tiberian Dawn
    TD = 0,
    /// Red Alert
    RA = 1,
    /// Tiberian Sun
    TS = 2,
    /// Dune 2
    DUNE2 = 3,
    /// Dune 2000
    DUNE2000 = 4,
    /// Red Alert 2
    RA2 = 5,
    /// Red Alert 2: Yuri's Revenge
    RA2_YR = 6,
    /// Renegade
    RG = 7,
    /// Generals
    GR = 8,
    /// Generals: Zero Hour
    GR_ZH = 9,
    /// Emperor: Battle for Dune
    EBFD = 10,
    /// Nox
    NOX = 11,
    /// Battle for Middle Earth
    BFME = 12,
    /// Battle for Middle Earth 2
    BFME2 = 13,
    /// Tiberium Wars
    TW = 14,
    /// Tiberian Sun: Firestorm
    TS_FS = 15,
    /// Unknown game
    UNKNOWN = 16,
}

impl Default for XCCGame {
    fn default() -> Self {
        Self::RA2
    }
}
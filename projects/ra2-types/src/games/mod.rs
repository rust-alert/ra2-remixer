

/// Enum representing different games that use XCC format
#[allow(non_camel_case_types)]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum CncGame {
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
    #[default]
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

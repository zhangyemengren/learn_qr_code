use std::{cmp::Ordering, ops::Not};

use crate::cast::As;

#[derive(Debug, PartialEq, Eq, Copy, Clone, PartialOrd, Ord)]
pub enum EcLevel {
    /// Low error correction. Allows up to 7% of wrong blocks.
    L = 0,

    /// Medium error correction (default). Allows up to 15% of wrong blocks.
    M = 1,

    /// "Quartile" error correction. Allows up to 25% of wrong blocks.
    Q = 2,

    /// High error correction. Allows up to 30% of wrong blocks.
    H = 3,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum QrError {
    /// The data is too long to encode into a QR code for the given version.
    DataTooLong,

    /// The provided version / error correction level combination is invalid.
    InvalidVersion,

    /// Some characters in the data cannot be supported by the provided QR code
    /// version.
    UnsupportedCharacterSet,

    /// A character not belonging to the character set is found.
    InvalidCharacter,
}


pub type QrResult<T> = Result<T, QrError>;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Mode {
    /// The data contains only characters 0 to 9.
    Numeric,

    /// The data contains only uppercase letters (A–Z), numbers (0–9) and a few
    /// punctuations marks (space, `$`, `%`, `*`, `+`, `-`, `.`, `/`, `:`).
    Alphanumeric,

    /// The data contains arbitrary binary data.
    Byte,

    /// The data contains Shift-JIS-encoded double-byte text.
    Kanji,
}

impl Mode{
    pub fn length_bits_count(self, version: Version) -> usize {
        match version {
            Version::Micro(a) => {
                let a = a.as_usize();
                match self {
                    Self::Numeric => 2 + a,
                    Self::Alphanumeric | Self::Byte => 1 + a,
                    Self::Kanji => a,
                }
            }
            Version::Normal(1..=9) => match self {
                Self::Numeric => 10,
                Self::Alphanumeric => 9,
                Self::Byte | Self::Kanji => 8,
            },
            Version::Normal(10..=26) => match self {
                Self::Numeric => 12,
                Self::Alphanumeric => 11,
                Self::Byte => 16,
                Self::Kanji => 10,
            },
            Version::Normal(_) => match self {
                Self::Numeric => 14,
                Self::Alphanumeric => 13,
                Self::Byte => 16,
                Self::Kanji => 12,
            },
        }
    }

    pub const fn data_bits_count(self, raw_data_len: usize) -> usize {
        match self {
            Self::Numeric => (raw_data_len * 10 + 2) / 3,
            Self::Alphanumeric => (raw_data_len * 11 + 1) / 2,
            Self::Byte => raw_data_len * 8,
            Self::Kanji => raw_data_len * 13,
        }
    }
    pub fn max(self, other: Self) -> Self {
        match self.partial_cmp(&other) {
            Some(Ordering::Greater) => self,
            Some(_) => other,
            None => Self::Byte,
        }
    }
}

impl PartialOrd for Mode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (*self, *other) {
            (a, b) if a == b => Some(Ordering::Equal),
            (Self::Numeric, Self::Alphanumeric) | (_, Self::Byte) => Some(Ordering::Less),
            (Self::Alphanumeric, Self::Numeric) | (Self::Byte, _) => Some(Ordering::Greater),
            _ => None,
        }
    }   
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Version {
    /// A normal QR code version. The parameter should be between 1 and 40.
    Normal(i16),

    /// A Micro QR code version. The parameter should be between 1 and 4.
    Micro(i16),
}

impl Version {
    pub const fn width(self) -> i16 {
        match self {
            Self::Normal(v) => v * 4 + 17,
            Self::Micro(v) => v * 2 + 9,
        }
    }
    pub fn mode_bits_count(self) -> usize {
        if let Self::Micro(a) = self {
            (a - 1).as_usize()
        } else {
            4
        }
    }

    pub fn fetch<T>(self, ec_level: EcLevel, table: &[[T; 4]]) -> QrResult<T>
    where
        T: PartialEq + Default + Copy,
    {
        match self {
            Self::Normal(v @ 1..=40) => {
                return Ok(table[(v - 1).as_usize()][ec_level as usize]);
            }
            Self::Micro(v @ 1..=4) => {
                let obj = table[(v + 39).as_usize()][ec_level as usize];
                if obj != T::default() {
                    return Ok(obj);
                }
            }
            _ => {}
        }
        Err(QrError::InvalidVersion)
    }
    pub const fn is_micro(self) -> bool {
        matches!(self, Self::Micro(_))
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Color {
    /// The module is light colored.
    Light,
    /// The module is dark colored.
    Dark,
}

impl Color {
    pub fn select<T>(self, dark: T, light: T) -> T {
        match self {
            Self::Light => light,
            Self::Dark => dark,
        }
    }
}

impl Not for Color {
    type Output = Self;
    fn not(self) -> Self {
        match self {
            Self::Light => Self::Dark,
            Self::Dark => Self::Light,
        }
    }
}
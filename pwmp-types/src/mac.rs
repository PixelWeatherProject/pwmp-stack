use serde::{Deserialize, Serialize};
use std::{
    fmt::Display,
    num::ParseIntError,
    ops::{Index, IndexMut},
    str::FromStr,
};

const MAC_STR_LEN: usize = "11:22:33:44:55:66".len();

/// MAC address.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Mac(u8, u8, u8, u8, u8, u8);

/// MAC address parse error.
#[derive(Debug, PartialEq, Eq)]
pub struct MacParseError;

impl Mac {
    /// Create a new instance with the specified octets.
    #[must_use]
    #[allow(clippy::many_single_char_names)]
    pub const fn new(a: u8, b: u8, c: u8, d: u8, e: u8, f: u8) -> Self {
        Self(a, b, c, d, e, f)
    }
}

impl FromStr for Mac {
    type Err = MacParseError;

    #[allow(clippy::cast_possible_truncation)]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != MAC_STR_LEN {
            return Err(MacParseError);
        }

        let split = s.split(':');
        let mut mac = Self::new(0, 0, 0, 0, 0, 0);

        for (i, octet) in split.enumerate() {
            let value = u8::from_str_radix(octet, 16)?;
            mac[i] = value;
        }

        Ok(mac)
    }
}

impl Display for Mac {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
            self.0, self.1, self.2, self.3, self.4, self.5
        )
    }
}

impl Index<usize> for Mac {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            3 => &self.3,
            4 => &self.4,
            5 => &self.5,
            _ => panic!("Bad octet index"),
        }
    }
}

impl IndexMut<usize> for Mac {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            3 => &mut self.3,
            4 => &mut self.4,
            5 => &mut self.5,
            _ => panic!("Bad octet index"),
        }
    }
}

impl From<ParseIntError> for MacParseError {
    fn from(_: ParseIntError) -> Self {
        Self
    }
}

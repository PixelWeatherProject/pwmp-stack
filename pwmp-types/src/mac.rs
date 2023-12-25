use serde::{Deserialize, Serialize};
use std::{fmt::Display, num::ParseIntError, str::FromStr};

const MAC_STR_LEN: usize = "11:22:33:44:55:66".len();

/// MAC address.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Mac(u8, u8, u8, u8, u8, u8);

/// MAC address parse error.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MacParseError;

impl Mac {
    /// Create a new instance with the specified octets.
    #[must_use]
    #[allow(clippy::many_single_char_names)]
    pub const fn new(a: u8, b: u8, c: u8, d: u8, e: u8, f: u8) -> Self {
        Self(a, b, c, d, e, f)
    }

    /// Get n-th octet of the MAC address.
    /// # Panics
    /// This will panic if `n` is out of range `0..=5`.
    #[must_use]
    pub const fn nth_octet(&self, n: u8) -> u8 {
        match n {
            0 => self.0,
            1 => self.1,
            2 => self.2,
            3 => self.3,
            4 => self.4,
            5 => self.5,
            _ => panic!("Bad octet index"),
        }
    }

    fn nth_octet_mut(&mut self, n: u8) -> &mut u8 {
        match n {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            3 => &mut self.3,
            4 => &mut self.4,
            5 => &mut self.5,
            _ => unreachable!("Bad octet index"),
        }
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
            *mac.nth_octet_mut(i as u8) = value;
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

impl From<ParseIntError> for MacParseError {
    fn from(_: ParseIntError) -> Self {
        Self
    }
}

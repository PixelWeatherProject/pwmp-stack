use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Default)]
pub struct Decimal(i8, u8);

impl Decimal {
    #[must_use]
    pub const fn new(whole: i8, fraction: u8) -> Self {
        Self(whole, fraction)
    }

    #[must_use]
    pub fn from_f32(v: f32, scale: u8) -> Self {
        let string = format!("{:.*}", scale as usize, v);
        let (whole, fraction) = string.split_once('.').unwrap();

        let whole: i8 = whole.parse().unwrap();
        let fraction: u8 = fraction.parse().unwrap();

        Self::new(whole, fraction)
    }

    #[must_use]
    pub const fn whole(self) -> i8 {
        self.0
    }

    #[must_use]
    pub const fn fraction(self) -> u8 {
        self.1
    }

    #[must_use]
    pub const fn scale(self) -> u8 {
        let mut fract = self.1;
        let mut count = 0;

        while fract != 0 {
            fract /= 10;
            count += 1;
        }

        count
    }
}

impl From<f32> for Decimal {
    fn from(value: f32) -> Self {
        // default to scale 2
        Self::from_f32(value, 2)
    }
}

impl Display for Decimal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.0, self.1)
    }
}

impl From<Decimal> for f32 {
    #[allow(clippy::cast_lossless)]
    fn from(value: Decimal) -> Self {
        let mut v = value.0 as Self;
        let mut fract = value.1 as Self;

        while fract > 0.0 {
            fract /= 10.0;
        }

        v += fract;
        v
    }
}

#[cfg(feature = "bigdecimal")]
#[allow(clippy::fallible_impl_from)]
impl From<Decimal> for bigdecimal::BigDecimal {
    fn from(value: Decimal) -> Self {
        use std::str::FromStr;
        Self::from_str(value.to_string().as_str()).unwrap()
    }
}

#[cfg(feature = "bigdecimal")]
use bigdecimal::BigDecimal;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// A type that allows containing multiple data types.
/// Used to represent node setting values.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum SettingValue {
    Number(u16),
    Decimal(Decimal),
    Boolean(bool),
}

macro_rules! impl_simple_from {
    ($src: ty, $variant: ident) => {
        impl From<$src> for SettingValue {
            fn from(value: $src) -> Self {
                Self::$variant(value)
            }
        }
    };
}

macro_rules! impl_simple_getter {
    ($name: ident, $t: ty, $variant: ident) => {
        #[must_use]
        pub fn $name(self) -> Option<$t> {
            if let Self::$variant(value) = self {
                Some(value)
            } else {
                None
            }
        }
    };
}

impl_simple_from!(u16, Number);
impl_simple_from!(Decimal, Decimal);
impl_simple_from!(bool, Boolean);

impl SettingValue {
    impl_simple_getter!(as_number, u16, Number);
    impl_simple_getter!(as_decimal, Decimal, Decimal);
    impl_simple_getter!(as_bool, bool, Boolean);

    #[cfg(feature = "bigdecimal")]
    pub fn as_bigdecimal(self) -> Option<BigDecimal> {
        use std::str::FromStr;

        let dec = self.as_decimal()?;
        BigDecimal::from_str(&dec.to_string()).ok()
    }
}

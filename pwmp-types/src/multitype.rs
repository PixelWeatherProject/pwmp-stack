use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

/// A type that allows containing multiple data types.
/// Used to represent node setting values.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum SettingValue {
    Number(u16),
    Decimal(BigDecimal),
    Text(String),
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
impl_simple_from!(BigDecimal, Decimal);
impl_simple_from!(bool, Boolean);

impl From<&str> for SettingValue {
    fn from(value: &str) -> Self {
        Self::Text(value.to_string())
    }
}

impl SettingValue {
    impl_simple_getter!(as_number, u16, Number);
    impl_simple_getter!(as_decimal, BigDecimal, Decimal);
    impl_simple_getter!(as_text, String, Text);
    impl_simple_getter!(as_bool, bool, Boolean);
}

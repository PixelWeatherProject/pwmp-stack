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

impl_simple_from!(u16, Number);
impl_simple_from!(BigDecimal, Decimal);
impl_simple_from!(bool, Boolean);

impl From<&str> for SettingValue {
    fn from(value: &str) -> Self {
        Self::Text(value.to_string())
    }
}

impl SettingValue {
    #[must_use]
    pub fn as_number(self) -> Option<u16> {
        if let Self::Number(value) = self {
            Some(value)
        } else {
            None
        }
    }

    #[must_use]
    pub fn as_decimal(self) -> Option<BigDecimal> {
        if let Self::Decimal(value) = self {
            Some(value)
        } else {
            None
        }
    }

    #[must_use]
    pub fn as_text(self) -> Option<String> {
        if let Self::Text(value) = self {
            Some(value)
        } else {
            None
        }
    }
}

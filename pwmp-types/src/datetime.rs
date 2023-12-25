use serde::{Deserialize, Serialize};

/// An object containing both date and time information.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct DateTime(pub Date, pub Time);

/// Date information.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Date {
    pub year: u16,
    pub month: u8,
    pub day: u8,
}

/// Time information.
/// Hours, minutes, seconds.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Time(pub u8, pub u8, pub u8);

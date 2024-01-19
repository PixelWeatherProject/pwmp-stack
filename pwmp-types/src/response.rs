use crate::multitype::SettingValue;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum Response {
    Pong,
    Ok,
    Reject,
    Settings(Box<[SettingValue]>),
}

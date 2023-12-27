use crate::multitype::SettingValue;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum Response {
    Pong,
    Ok,
    Reject,
    Setting(SettingValue),
    Settings(Vec<SettingValue>),
}

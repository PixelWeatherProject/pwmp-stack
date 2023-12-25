use crate::{datetime::DateTime, multitype::SettingValue};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum Response {
    Pong,
    Ok,
    Reject,
    DateTime(DateTime),
    Setting(SettingValue),
    Settings(Vec<SettingValue>),
}

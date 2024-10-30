use crate::multitype::SettingValue;
use serde::{Deserialize, Serialize};

/// A response message used by the PWMP server to respond to [`Request`](crate::request::Request)s.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum Response {
    /// A response message to [`Request::Ping`](crate::request::Request::Ping).
    Pong,

    /// Indicate a successfully processed request. This is usually used as a response to `Request::Post*` messages.
    Ok,

    /// The server has rejected authentication. The node/client will be disconnected.
    Reject,

    /// List of setting values requested by [`Request::GetSettings`](crate::request::Request::GetSettings).
    Settings(Box<[SettingValue]>),
}

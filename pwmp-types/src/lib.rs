#![allow(clippy::module_name_repetitions, clippy::missing_panics_doc)]
use request::Request;
use response::Response;
use serde::{Deserialize, Serialize};

/// Type aliases
pub mod aliases;
/// Contains the [`Mac`](mac::Mac) address type
pub mod mac;
/// Contains the [`SettingValue`](multitype::SettingValue) type
pub mod multitype;
/// Contains the [`Request`]type and it's implementations
pub mod request;
/// Contains the [`Response`] type and it's implementations
pub mod response;
/// Contains the [`SettingName`](setting::SettingName) type
pub mod setting;

/// Node ID type alias
pub type NodeId = i16;

/// A Message object.
/// Can either be a request or a response.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum Message {
    /// Server requested data from a client or vice-versa.
    Request(request::Request),
    /// Server responded to a request from a client or vice-versa.
    Response(response::Response),
}

impl Message {
    /// Serialize the message into raw bytes.
    #[must_use]
    pub fn to_raw(self) -> Vec<u8> {
        serde_cbor::to_vec(&self).unwrap()
    }

    /// Deserialize a message from raw bytes.
    #[must_use]
    pub fn from_raw(bytes: &[u8]) -> Option<Self> {
        serde_cbor::from_slice(bytes).ok()
    }

    /// Returns a reference to the contained [`Request`].
    /// If the message contains a [`Response`] instead, `None` is returned.
    #[must_use]
    pub const fn request(&self) -> Option<&Request> {
        if let Self::Request(req) = self {
            Some(req)
        } else {
            None
        }
    }

    /// Returns a reference to the contained [`Response`].
    /// If the message contains a [`Request`] instead, `None` is returned.
    #[must_use]
    pub const fn response(&self) -> Option<&Response> {
        if let Self::Response(resp) = self {
            Some(resp)
        } else {
            None
        }
    }

    /// Similar to [`request()`](Self::request), but consumes the message itself.
    #[must_use]
    pub fn to_request(self) -> Option<Request> {
        if let Self::Request(req) = self {
            Some(req)
        } else {
            None
        }
    }

    /// Similar to [`response()`](Self::response), but consumes the message itself.
    #[must_use]
    pub fn to_response(self) -> Option<Response> {
        if let Self::Response(resp) = self {
            Some(resp)
        } else {
            None
        }
    }

    /// Returns the length of the message if it was serialized.
    /// ```rust
    /// # use pwmp_types::{Message, response::Response, request::Request};
    /// let ping = Message::Request(Request::Ping);
    /// let pong = Message::Response(Response::Pong);
    ///
    /// assert_eq!(ping.size(), 14);
    /// assert_eq!(pong.size(), 15);
    /// ```
    #[must_use]
    pub fn size(&self) -> usize {
        serde_cbor::to_vec(self).unwrap().len()
    }
}

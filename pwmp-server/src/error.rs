use std::{fmt::Display, io};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Failed to parse a `Message`
    MessageParse,
    /// Expected a message of type `Request`, got `Response` instead
    NotRequest,
    /// Expected the first message to be of type `Hello`
    NotHello,
    /// Request was malformed or cannot be processed
    BadRequest,
    /// Connection closed unexpectedly
    Quit,
    /// Generic I/O error
    Io(#[from] io::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MessageParse => write!(f, "Failed to parse message"),
            Self::NotRequest => write!(
                f,
                "Expected message of variant `Request`, got `Response` instead"
            ),
            Self::NotHello => write!(f, "Expected a `Hello` request"),
            Self::BadRequest => write!(f, "Malformed or unprocessable request"),
            Self::Quit => write!(f, "Connection closed unexpectedly"),
            Self::Io(why) => write!(f, "{why}"),
        }
    }
}

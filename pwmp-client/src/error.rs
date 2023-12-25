use std::io;

/// Errors.
#[derive(Debug)]
pub enum Error {
    /// Generic I/O error.
    Io(io::Error),
    /// Server rejected the client.
    Rejected,
    /// Expected a response message, got request instead.
    NotResponse,
    /// Expected a request message, got response instead.
    MessageParse,
    /// Unexpected variant of a response or request.
    UnexpectedVariant,
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

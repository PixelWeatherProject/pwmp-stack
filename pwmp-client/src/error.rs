use std::io;

/// Errors.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Generic I/O error.
    #[error("generic i/o error")]
    Io(#[from] io::Error),
    /// Server rejected the client.
    #[error("server rejected")]
    Rejected,
    /// Expected a response message, got request instead.
    #[error("not response")]
    NotResponse,
    /// Expected a request message, got response instead.
    #[error("parse")]
    MessageParse,
    /// Unexpected variant of a response or request.
    #[error("bad variant")]
    UnexpectedVariant,
    /// Malformed response
    #[error("malformed response")]
    MalformedResponse,
}

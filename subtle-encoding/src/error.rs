//! Error type

#[cfg(all(feature = "alloc", not(feature = "std")))]
use alloc::string::FromUtf8Error;
#[cfg(feature = "std")]
use std::{io, string::FromUtf8Error};

/// Error type
#[derive(Clone, Eq, PartialEq, Debug, Fail)]
pub enum Error {
    /// Checksum fdoes not match expected value
    #[fail(display = "checksum mismatch")]
    ChecksumInvalid,

    /// Data is not encoded correctly
    #[fail(display = "bad encoding")]
    EncodingInvalid,

    /// Error performing I/O operation
    #[cfg(feature = "std")]
    #[fail(display = "I/O error")]
    IoError,

    /// Input or output buffer is an incorrect length
    #[fail(display = "invalid length")]
    LengthInvalid,

    /// Padding missing/invalid
    #[fail(display = "padding invalid")]
    PaddingInvalid,

    /// Trailing whitespace detected
    // TODO: handle trailing whitespace?
    #[fail(display = "trailing whitespace")]
    TrailingWhitespace,
}

/// Assert that the provided condition is true, or else return the given error
macro_rules! ensure {
    ($condition:expr, $err:ident) => {
        if !($condition) {
            Err($err)?;
        }
    };
}

#[cfg(feature = "std")]
impl From<io::Error> for Error {
    fn from(_err: io::Error) -> Error {
        // TODO: preserve cause or error message?
        Error::IoError
    }
}

#[cfg(feature = "alloc")]
impl From<FromUtf8Error> for Error {
    fn from(_err: FromUtf8Error) -> Error {
        // TODO: preserve cause or error message?
        Error::EncodingInvalid
    }
}

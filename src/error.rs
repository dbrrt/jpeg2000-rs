//! Errors surfaced by the crate.

use std::fmt;

/// Something went wrong while applying a JPEG 2000–related operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// Geometry does not satisfy encoder / transform constraints.
    InvalidDimensions {
        width: usize,
        height: usize,
        reason: &'static str,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidDimensions {
                width,
                height,
                reason,
            } => write!(f, "invalid dimensions {width}x{height}: {reason}",),
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;

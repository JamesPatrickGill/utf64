use std::fmt;

/// Errors that can occur during UTF64 encoding and decoding operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Utf64Error {
    /// The input contains invalid UTF-8 data that cannot be encoded to UTF64.
    InvalidUtf8,

    /// The UTF64 data is malformed or contains invalid sequences.
    InvalidUtf64,

    /// Reserved bits are not zero (violates UTF64 v1.0 specification).
    NonZeroReservedBits,
}

impl fmt::Display for Utf64Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Utf64Error::InvalidUtf8 => write!(f, "invalid UTF-8 data"),
            Utf64Error::InvalidUtf64 => write!(f, "invalid UTF64 encoding"),
            Utf64Error::NonZeroReservedBits => {
                write!(f, "reserved bits must be zero in UTF64 v1.0")
            }
        }
    }
}

impl std::error::Error for Utf64Error {}

/// A specialized Result type for UTF64 operations.
pub type Result<T> = std::result::Result<T, Utf64Error>;

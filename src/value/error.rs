use thiserror::Error;

use super::FrcType;

#[derive(Debug, Clone, Copy)]
pub enum CastErrorReason {
    Type,
    Overflow,
    Underflow,
    Deserialization,
}

/// An error that occurs when casting between [``FrcValue``](super::FrcValue) and other types
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, Error)]
pub enum FrcValueCastError {
    #[error("Could not cast {0} variant to {1} type ({2:?})")]
    InvalidCastTo(FrcType, &'static str, CastErrorReason),
    #[error("Could not cast {0} type to {1} variant ({2:?})")]
    InvalidCastFrom(&'static str, FrcType, CastErrorReason),
    #[error("Could not represent the casted data as an FrcValue")]
    UnrepresentableCast,
}

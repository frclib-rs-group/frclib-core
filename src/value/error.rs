use thiserror::Error;

use super::FrcType;

#[derive(Debug, Clone, Copy)]
pub enum CastErrorReason {
    Type,
    Overflow,
    Underflow,
    Deserialization
}

#[derive(Debug, Clone, Copy, Error)]
pub enum FrcValueCastError {
    #[error("Could not cast {0} variant to {1} type ({2:?})")]
    InvalidCastTo(FrcType, &'static str, CastErrorReason),
    #[error("Could not cast {0} type to {1} variant ({2:?})")]
    InvalidCastFrom(&'static str, FrcType, CastErrorReason),
    #[error("Could not represent the casted data as an FrcValue")]
    UnrepresentableCast,
}

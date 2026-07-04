use thiserror::Error;

pub type TinyJuiceResult<T> = Result<T, TinyJuiceError>;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum TinyJuiceError {
    #[error("compression input was empty")]
    EmptyInput,
    #[error("target ratio must be greater than 0 and at most 1")]
    InvalidTargetRatio,
    #[error("compression strategy is not implemented: {0}")]
    NotImplemented(&'static str),
}

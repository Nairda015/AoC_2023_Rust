use std::array::TryFromSliceError;
use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AocError {
    #[error("IO error")]
    IoError(#[from] io::Error),
    #[error("Invalid configuration provider: {0}")]
    InvalidConfigurationProvider(String),
    #[error("Cannot load configuration: {0}")]
    CannotLoadConfiguration(String),
    #[error("Invalid configuration")]
    InvalidConfiguration,
    // #[error("Write error")]
    // WriteError(#[from] WriteError),
    // #[error("Read to end error")]
    // ReadToEndError(#[from] ReadToEndError),
    #[error("Try from slice error")]
    TryFromSliceError(#[from] TryFromSliceError),
    #[error("Logging filter reload failure")]
    FilterReloadFailure,
    #[error("Logging stdout reload failure")]
    StdoutReloadFailure,
    #[error("Logging file reload failure")]
    FileReloadFailure,
    #[error("Cache config validation failure: {0}")]
    CacheConfigValidationFailure(String),
}
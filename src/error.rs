use thiserror::Error;

#[derive(Error, Debug)]
pub enum BIP39Error {
    #[error("Invalid word count: {0}. Must be either 12 or 24")]
    InvalidWordCount(u8),

    #[error("Invalid entropy: {0}")]
    InvalidEntropy(String),

    #[error("Checksum validation failed")]
    ChecksumError,

    #[error("Word list error: {0}")]
    WordListError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, BIP39Error>;
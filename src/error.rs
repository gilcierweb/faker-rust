//! Error types for Faker-Rust

use std::fmt;

#[derive(Debug, Clone)]
pub enum FakerError {
    /// Locale not found
    LocaleNotFound(String),
    /// Data not found
    DataNotFound(String),
    /// Unique generator exhausted
    UniqueGeneratorExhausted(usize),
    /// Invalid argument
    InvalidArgument(String),
    /// Random number generation error
    RandError(String),
}

impl fmt::Display for FakerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FakerError::LocaleNotFound(locale) => write!(f, "Locale not found: {}", locale),
            FakerError::DataNotFound(key) => write!(f, "Data not found: {}", key),
            FakerError::UniqueGeneratorExhausted(retries) => {
                write!(f, "Unique generator exhausted after {} retries", retries)
            }
            FakerError::InvalidArgument(msg) => write!(f, "Invalid argument: {}", msg),
            FakerError::RandError(msg) => write!(f, "Random error: {}", msg),
        }
    }
}

impl std::error::Error for FakerError {}

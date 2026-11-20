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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_messages_are_descriptive() {
        assert_eq!(
            FakerError::LocaleNotFound("fr".into()).to_string(),
            "Locale not found: fr"
        );
        assert_eq!(
            FakerError::DataNotFound("name.first".into()).to_string(),
            "Data not found: name.first"
        );
        assert_eq!(
            FakerError::UniqueGeneratorExhausted(10).to_string(),
            "Unique generator exhausted after 10 retries"
        );
        assert_eq!(
            FakerError::InvalidArgument("bad".into()).to_string(),
            "Invalid argument: bad"
        );
        assert_eq!(
            FakerError::RandError("oops".into()).to_string(),
            "Random error: oops"
        );
    }

    #[test]
    fn implements_std_error() {
        fn assert_error<E: std::error::Error>() {}
        assert_error::<FakerError>();
    }
}

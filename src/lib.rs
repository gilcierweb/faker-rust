//! Faker-Rust - Generate realistic fake data for testing and development.
//!
//! This is a Rust port of the Ruby Faker gem.
//!
//! # Quick Start
//!
//! ```rust
//! use faker;
//!
//! // Generate random data
//! let name = faker::name::name();
//! let email = faker::internet::email(None, None, None);
//! let city = faker::address::city();
//! ```

#[macro_use]
extern crate rust_i18n;

i18n!("locales", fallback = "en");

pub mod base;
pub mod config;
pub mod error;
pub mod locale;

pub mod address;
pub mod company;
pub mod internet;
pub mod name;

// Re-export commonly used types
pub use config::FakerConfig;
pub use error::FakerError;

/// Main Faker struct providing access to all generators
pub struct Faker;

impl Faker {
    /// Get the current Faker configuration
    pub fn config() -> FakerConfig {
        FakerConfig::current()
    }

    /// Set the random seed for deterministic output
    pub fn set_seed(seed: u64) {
        FakerConfig::set_seed(seed);
    }
}

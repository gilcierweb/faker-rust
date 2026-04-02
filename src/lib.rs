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
pub mod bank;
pub mod book;
pub mod commerce;
pub mod company;
pub mod date;
pub mod educator;
pub mod gender;
pub mod hobby;
pub mod internet;
pub mod job;
pub mod lorem;
pub mod military;
pub mod name;
pub mod number;
pub mod phone_number;
pub mod relationship;
pub mod university;

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

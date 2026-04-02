//! Faker-Rust - Generate realistic fake data for testing and development.
//! 
//! This is a Rust port of the Ruby Faker gem.
//! 
//! # Quick Start
//! 
//! ```rust
//! use faker::Faker;
//! 
//! // Generate random data
//! let name = Faker::Name::name();
//! let email = Faker::Internet::email();
//! let city = Faker::Address::city();
//! ```
//! 
//! # Modules
//! 
//! - [`Faker::Name`] - Generate names
//! - [`Faker::Internet`] - Generate internet-related data
//! - [`Faker::Address`] - Generate addresses
//! - [`Faker::PhoneNumber`] - Generate phone numbers
//! - [`Faker::Number`] - Generate random numbers
//! - [`Faker::Lorem`] - Generate lorem ipsum text
//! - [`Faker::Date`] - Generate dates
//! - And many more...

#[macro_use]
extern crate rust_i18n;

i18n!("locales", fallback = "en");

pub mod base;
pub mod config;
pub mod error;

pub mod name;
pub mod internet;
pub mod address;
pub mod company;

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
    
    /// Set the locale for Faker
    pub fn set_locale(locale: &str) {
        rust_i18n::set_locale(locale);
    }
    
    /// Get the current locale
    pub fn locale() -> String {
        rust_i18n::locale().to_string()
    }
    
    /// Set the random seed for deterministic output
    pub fn set_seed(seed: u64) {
        FakerConfig::set_seed(seed);
    }
}
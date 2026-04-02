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

// Default category modules (like Ruby Faker's default folder)
#[path = "default/address.rs"]
pub mod address;
#[path = "default/bank.rs"]
pub mod bank;
#[path = "default/boolean.rs"]
pub mod boolean;
#[path = "default/color.rs"]
pub mod color;
#[path = "default/commerce.rs"]
pub mod commerce;
#[path = "default/company.rs"]
pub mod company;
#[path = "default/date.rs"]
pub mod date;
#[path = "default/gender.rs"]
pub mod gender;
#[path = "default/internet.rs"]
pub mod internet;
#[path = "default/job.rs"]
pub mod job;
#[path = "default/lorem.rs"]
pub mod lorem;
#[path = "default/name.rs"]
pub mod name;
#[path = "default/number.rs"]
pub mod number;
#[path = "default/phone_number.rs"]
pub mod phone_number;
#[path = "default/university.rs"]
pub mod university;

// Other category modules (like Ruby Faker's books, games, movies, music folders)
pub mod books;
pub mod games;
pub mod movies;
pub mod music;

// Additional modules
pub mod device;
pub mod educator;
pub mod hobby;
pub mod military;
pub mod relationship;
pub mod vehicle;

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

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
#[path = "default/app.rs"]
pub mod app;
#[path = "default/appliance.rs"]
pub mod appliance;
#[path = "default/artist.rs"]
pub mod artist;
#[path = "default/avatar.rs"]
pub mod avatar;
#[path = "default/bank.rs"]
pub mod bank;
#[path = "default/barcode.rs"]
pub mod barcode;
#[path = "default/beer.rs"]
pub mod beer;
#[path = "default/blood.rs"]
pub mod blood;
#[path = "default/boolean.rs"]
pub mod boolean;
#[path = "default/business.rs"]
pub mod business;
#[path = "default/camera.rs"]
pub mod camera;
#[path = "default/cannabis.rs"]
pub mod cannabis;
#[path = "default/chess.rs"]
pub mod chess;
#[path = "default/chuck_norris.rs"]
pub mod chuck_norris;
#[path = "default/code.rs"]
pub mod code;
#[path = "default/coffee.rs"]
pub mod coffee;
#[path = "default/coin.rs"]
pub mod coin;
#[path = "default/color.rs"]
pub mod color;
#[path = "default/commerce.rs"]
pub mod commerce;
#[path = "default/company.rs"]
pub mod company;
#[path = "default/compass.rs"]
pub mod compass;
#[path = "default/computer.rs"]
pub mod computer;
#[path = "default/construction.rs"]
pub mod construction;
#[path = "default/crypto.rs"]
pub mod crypto;
#[path = "default/currency.rs"]
pub mod currency;
#[path = "default/date.rs"]
pub mod date;
#[path = "default/device.rs"]
pub mod device;
#[path = "default/educator.rs"]
pub mod educator;
#[path = "default/file.rs"]
pub mod file;
#[path = "default/food.rs"]
pub mod food;
#[path = "default/gender.rs"]
pub mod gender;
#[path = "default/hobby.rs"]
pub mod hobby;
#[path = "default/internet.rs"]
pub mod internet;
#[path = "default/job.rs"]
pub mod job;
#[path = "default/lorem.rs"]
pub mod lorem;
#[path = "default/military.rs"]
pub mod military;
#[path = "default/name.rs"]
pub mod name;
#[path = "default/number.rs"]
pub mod number;
#[path = "default/phone_number.rs"]
pub mod phone_number;
#[path = "default/relationship.rs"]
pub mod relationship;
#[path = "default/science.rs"]
pub mod science;
#[path = "default/space.rs"]
pub mod space;
#[path = "default/time.rs"]
pub mod time;
#[path = "default/university.rs"]
pub mod university;
#[path = "default/vehicle.rs"]
pub mod vehicle;

#[path = "default/ancient.rs"]
pub mod ancient;
#[path = "default/culture.rs"]
pub mod culture;
#[path = "default/demographic.rs"]
pub mod demographic;
#[path = "default/driving_license.rs"]
pub mod driving_license;
#[path = "default/emotion.rs"]
pub mod emotion;
#[path = "default/humor.rs"]
pub mod humor;
#[path = "default/nation.rs"]
pub mod nation;
#[path = "default/quote.rs"]
pub mod quote;
#[path = "default/religion.rs"]
pub mod religion;
#[path = "default/supernatural.rs"]
pub mod supernatural;

// Other category modules (like Ruby Faker's books, games, movies, music folders)
pub mod blockchain;
pub mod books;
pub mod creature;
pub mod games;
pub mod japanese_media;
pub mod movies;
pub mod music;
pub mod sports;
pub mod travel;
pub mod tv_shows;

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

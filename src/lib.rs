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
#[path = "default/supernatural.rs"]
pub mod supernatural;

#[path = "default/dessert.rs"]
pub mod dessert;
#[path = "default/finance.rs"]
pub mod finance;
#[path = "default/funny_name.rs"]
pub mod funny_name;
#[path = "default/greek_philosophers.rs"]
pub mod greek_philosophers;
#[path = "default/hacker.rs"]
pub mod hacker;
#[path = "default/hipster.rs"]
pub mod hipster;
#[path = "default/house.rs"]
pub mod house;
#[path = "default/json.rs"]
pub mod json;
#[path = "default/kpop.rs"]
pub mod kpop;
#[path = "default/markdown.rs"]
pub mod markdown;

// New modules added for Ruby Faker parity
#[path = "default/adjective.rs"]
pub mod adjective;
#[path = "default/alphanumeric.rs"]
pub mod alphanumeric;
#[path = "default/chile_rut.rs"]
pub mod chile_rut;
#[path = "default/cosmere.rs"]
pub mod cosmere;
#[path = "default/crypto_coin.rs"]
pub mod crypto_coin;
#[path = "default/dc_comics.rs"]
pub mod dc_comics;
#[path = "default/drone.rs"]
pub mod drone;
#[path = "default/driving_licence.rs"]
pub mod driving_licence;
#[path = "default/electrical_components.rs"]
pub mod electrical_components;
#[path = "default/esport.rs"]
pub mod esport;
#[path = "default/html.rs"]
pub mod html;
#[path = "default/id_number.rs"]
pub mod id_number;
#[path = "default/industry_segments.rs"]
pub mod industry_segments;
#[path = "default/internet_http.rs"]
pub mod internet_http;
#[path = "default/invoice.rs"]
pub mod invoice;
#[path = "default/lorem_flickr.rs"]
pub mod lorem_flickr;
#[path = "default/marketing.rs"]
pub mod marketing;
#[path = "default/measurement.rs"]
pub mod measurement;
#[path = "default/mountain.rs"]
pub mod mountain;
#[path = "default/national_health_service.rs"]
pub mod national_health_service;
#[path = "default/nato_phonetic_alphabet.rs"]
pub mod nato_phonetic_alphabet;
#[path = "default/omniauth.rs"]
pub mod omniauth;
#[path = "default/placeholdit.rs"]
pub mod placeholdit;
#[path = "default/programming_language.rs"]
pub mod programming_language;
#[path = "default/restaurant.rs"]
pub mod restaurant;
#[path = "default/slack_emoji.rs"]
pub mod slack_emoji;
#[path = "default/source.rs"]
pub mod source;
#[path = "default/south_africa.rs"]
pub mod south_africa;
#[path = "default/string.rs"]
pub mod string;
#[path = "default/stripe.rs"]
pub mod stripe;
#[path = "default/subscription.rs"]
pub mod subscription;
#[path = "default/superhero.rs"]
pub mod superhero;
#[path = "default/team.rs"]
pub mod team;
#[path = "default/tea.rs"]
pub mod tea;
#[path = "default/theater.rs"]
pub mod theater;
#[path = "default/types.rs"]
pub mod types;
#[path = "default/verb.rs"]
pub mod verb;
#[path = "default/vulnerability_identifier.rs"]
pub mod vulnerability_identifier;
#[path = "default/world_cup.rs"]
pub mod world_cup;
#[path = "default/x.rs"]
pub mod x;

// Other category modules (like Ruby Faker's books, games, movies, music folders)
pub mod blockchain;
pub mod books;
pub mod creature;
pub mod fantasy;
pub mod games;
pub mod japanese_media;
pub mod locations;
pub mod movies;
pub mod music;
pub mod quotes;
pub mod religion;
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

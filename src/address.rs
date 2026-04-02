//! Address generator - generates random addresses

use crate::base::{bothify, sample};
use crate::locale::{fetch_locale_with_context, sample_with_resolve};

/// Generate a random city name
pub fn city() -> String {
    fetch_locale_with_context("address.city", "en", Some("address"))
        .map(|v| sample_with_resolve(&v, Some("address")))
        .unwrap_or_else(|| sample(&FALLBACK_CITIES).to_string())
}

/// Generate a random street name
pub fn street_name() -> String {
    let prefix = fetch_locale_with_context("address.street_prefix", "en", Some("address"))
        .map(|v| sample_with_resolve(&v, Some("address")))
        .unwrap_or_else(|| sample(&FALLBACK_STREET_PREFIXES).to_string());
    let suffix = fetch_locale_with_context("address.street_suffix", "en", Some("address"))
        .map(|v| sample_with_resolve(&v, Some("address")))
        .unwrap_or_else(|| sample(&FALLBACK_STREET_SUFFIXES).to_string());
    format!("{} {}", prefix, suffix)
}

/// Generate a random street address
pub fn street_address() -> String {
    let num = fetch_locale_with_context("address.street_number", "en", Some("address"))
        .map(|v| sample_with_resolve(&v, Some("address")))
        .unwrap_or_else(|| sample(&FALLBACK_STREET_NUMBERS).to_string());
    format!("{} {}", num, street_name())
}

/// Generate a random secondary address
pub fn secondary_address() -> String {
    let pattern = fetch_locale_with_context("address.secondary_address", "en", Some("address"))
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Apt. ###".to_string());
    bothify(&pattern)
}

/// Generate a random zip code
pub fn zip_code() -> String {
    bothify("#####")
}

/// Generate a random zip code with extension
pub fn zip_code_with_extension() -> String {
    bothify("#####-####")
}

/// Generate a random country name
pub fn country() -> String {
    fetch_locale_with_context("address.country", "en", Some("address"))
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(&FALLBACK_COUNTRIES).to_string())
}

/// Generate a random country code
pub fn country_code() -> String {
    sample(&FALLBACK_COUNTRY_CODES).to_string()
}

/// Generate a random full address
pub fn full_address() -> String {
    let pattern = crate::config::FakerConfig::current().rand_range(0, 4);

    match pattern {
        0 => format!("{}, {}", street_address(), city_state_zip()),
        1 => format!("{}\n{}", street_address(), city_state_zip()),
        2 => format!("{}, {}", secondary_address(), city_state_zip()),
        _ => format!(
            "{}\n{}, {}",
            street_address(),
            secondary_address(),
            city_state_zip()
        ),
    }
}

fn city_state_zip() -> String {
    let city_val = city();
    let state = fetch_locale_with_context("address.state", "en", Some("address"))
        .map(|v| sample(&v))
        .unwrap_or_else(|| "CA".to_string());
    format!("{}, {} {}", city_val, state, zip_code())
}

/// Generate a random time zone
pub fn time_zone() -> String {
    sample(&FALLBACK_TIME_ZONES).to_string()
}

/// Generate a random latitude
pub fn latitude() -> String {
    let config = crate::config::FakerConfig::current();
    let lat = config.rand_range(0, 180_000_000) as f64 - 90_000_000.0;
    format!("{:.6}", lat / 1_000_000.0)
}

/// Generate a random longitude
pub fn longitude() -> String {
    let config = crate::config::FakerConfig::current();
    let lon = config.rand_range(0, 360_000_000) as f64 - 180_000_000.0;
    format!("{:.6}", lon / 1_000_000.0)
}

// Fallback data
const FALLBACK_CITIES: &[&str] = &[
    "New York",
    "Los Angeles",
    "Chicago",
    "Houston",
    "Phoenix",
    "Philadelphia",
    "San Antonio",
    "San Diego",
    "Dallas",
    "San Jose",
    "Austin",
    "Jacksonville",
];

const FALLBACK_STREET_PREFIXES: &[&str] = &[
    "Main",
    "Oak",
    "Pine",
    "Maple",
    "Cedar",
    "Elm",
    "Washington",
    "Lake",
    "Hill",
    "Park",
    "Forest",
    "River",
];

const FALLBACK_STREET_SUFFIXES: &[&str] = &[
    "Street",
    "Avenue",
    "Road",
    "Boulevard",
    "Drive",
    "Lane",
    "Way",
    "Court",
];

const FALLBACK_STREET_NUMBERS: &[&str] = &[
    "1", "2", "3", "4", "5", "10", "11", "12", "13", "14", "15", "20", "21", "22", "30", "40",
    "50", "100", "200", "300", "400", "500",
];

const FALLBACK_COUNTRIES: &[&str] = &[
    "United States",
    "United Kingdom",
    "Canada",
    "Australia",
    "Germany",
    "France",
    "Japan",
    "China",
    "India",
    "Brazil",
];

const FALLBACK_COUNTRY_CODES: &[&str] =
    &["US", "CA", "GB", "DE", "FR", "JP", "CN", "IN", "BR", "AU"];

const FALLBACK_TIME_ZONES: &[&str] = &[
    "America/New_York",
    "America/Chicago",
    "America/Denver",
    "America/Los_Angeles",
    "Europe/London",
    "Europe/Paris",
    "Asia/Tokyo",
    "Asia/Shanghai",
];

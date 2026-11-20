//! Airport travel generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Gem-compatible generator: airport name for `region` and `size`.
///
/// Both arguments are required in `Faker::Travel::Airport.name(size:, region:)`.
/// Supported regions are `united_states` and `european_union`; sizes are
/// `large`, `medium` and `small` (the EU locale has no `small` bucket).
pub fn name(size: &str, region: &str) -> String {
    fetch_locale(
        &format!("airport.{region}.{size}"),
        &crate::config::FakerConfig::locale(),
    )
    .map(|v| sample(&v))
    .unwrap_or_else(|| sample(FALLBACK_NAMES).to_string())
}

/// Gem-compatible generator: IATA code for `region` and `size`.
///
/// Both arguments are required in `Faker::Travel::Airport.iata(size:, region:)`.
pub fn iata(size: &str, region: &str) -> String {
    fetch_locale(
        &format!("airport.{region}.iata_code.{size}"),
        &crate::config::FakerConfig::locale(),
    )
    .map(|v| sample(&v))
    .unwrap_or_else(generate_iata_code)
}

/// Generate a random airport name from the aggregate list.
///
/// This is a Rust convenience helper; the gem only exposes [`name`] with
/// explicit `size` and `region` arguments.
pub fn name_any() -> String {
    fetch_locale("airport.names", &crate::config::FakerConfig::locale())
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_NAMES).to_string())
}

/// Generate a random IATA airport code (3 letters).
///
/// This is a Rust convenience helper; the gem only exposes [`iata`] with
/// explicit `size` and `region` arguments.
pub fn iata_code() -> String {
    generate_iata_code()
}

/// Generate a random ICAO airport code (4 letters)
pub fn icao_code() -> String {
    generate_icao_code()
}

/// Generate a random city with airport
pub fn city() -> String {
    fetch_locale("airport.cities", &crate::config::FakerConfig::locale())
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_CITIES).to_string())
}

fn generate_iata_code() -> String {
    let config = crate::config::FakerConfig::current();
    const LETTERS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut code = String::new();

    for _ in 0..3 {
        let idx = config.rand_usize(LETTERS.len());
        code.push(LETTERS[idx] as char);
    }

    code
}

fn generate_icao_code() -> String {
    let config = crate::config::FakerConfig::current();
    const LETTERS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut code = String::new();

    for _ in 0..4 {
        let idx = config.rand_usize(LETTERS.len());
        code.push(LETTERS[idx] as char);
    }

    code
}

// Fallback data
const FALLBACK_NAMES: &[&str] = &[
    "John F. Kennedy International Airport",
    "Los Angeles International Airport",
    "Heathrow Airport",
    "Charles de Gaulle Airport",
    "Tokyo Haneda Airport",
    "Dubai International Airport",
    "Singapore Changi Airport",
    "Frankfurt Airport",
    "Amsterdam Schiphol Airport",
    "Sydney Kingsford Smith Airport",
    "Toronto Pearson International Airport",
    "São Paulo/Guarulhos International Airport",
];

const FALLBACK_CITIES: &[&str] = &[
    "New York",
    "Los Angeles",
    "London",
    "Paris",
    "Tokyo",
    "Dubai",
    "Singapore",
    "Frankfurt",
    "Amsterdam",
    "Sydney",
    "Toronto",
    "São Paulo",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        assert!(!name("large", "united_states").is_empty());
        assert!(!name("medium", "european_union").is_empty());
        assert!(!name_any().is_empty());
    }

    #[test]
    fn test_iata() {
        let code = iata("large", "united_states");
        assert_eq!(code.len(), 3);
        assert!(code.chars().all(|c| c.is_ascii_uppercase()));
    }

    #[test]
    fn test_iata_code() {
        let code = iata_code();
        assert_eq!(code.len(), 3);
        assert!(code.chars().all(|c| c.is_ascii_uppercase()));
    }

    #[test]
    fn test_icao_code() {
        let code = icao_code();
        assert_eq!(code.len(), 4);
        assert!(code.chars().all(|c| c.is_ascii_uppercase()));
    }

    #[test]
    fn test_city() {
        assert!(!city().is_empty());
    }
}

//! Airport travel generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random airport name
pub fn name() -> String {
    fetch_locale("airport.names", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_NAMES).to_string())
}

/// Generate a random IATA airport code (3 letters)
pub fn iata_code() -> String {
    fetch_locale("airport.iata_codes", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(generate_iata_code)
}

/// Generate a random ICAO airport code (4 letters)
pub fn icao_code() -> String {
    fetch_locale("airport.icao_codes", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(generate_icao_code)
}

/// Generate a random city with airport
pub fn city() -> String {
    fetch_locale("airport.cities", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_CITIES).to_string())
}

fn generate_iata_code() -> String {
    let config = crate::config::FakerConfig::current();
    const LETTERS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut code = String::new();

    for _ in 0..3 {
        let idx = config.rand_range(0, (LETTERS.len() - 1) as u32) as usize;
        code.push(LETTERS[idx] as char);
    }

    code
}

fn generate_icao_code() -> String {
    let config = crate::config::FakerConfig::current();
    const LETTERS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut code = String::new();

    for _ in 0..4 {
        let idx = config.rand_range(0, (LETTERS.len() - 1) as u32) as usize;
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
        assert!(!name().is_empty());
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

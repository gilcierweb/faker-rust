//! Train station generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random train station name
pub fn name() -> String {
    fetch_locale("train_station.names", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_NAMES).to_string())
}

/// Generate a random train station city
pub fn city() -> String {
    fetch_locale("train_station.cities", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_CITIES).to_string())
}

/// Generate a random railway line
pub fn line() -> String {
    fetch_locale("train_station.lines", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_LINES).to_string())
}

// Fallback data
const FALLBACK_NAMES: &[&str] = &[
    "Grand Central Terminal",
    "Penn Station",
    "Waterloo Station",
    "King's Cross Station",
    "Gare du Nord",
    "Tokyo Station",
    "Shinjuku Station",
    "Shibuya Station",
    "Berlin Hauptbahnhof",
    "München Hauptbahnhof",
    "Milano Centrale",
    "Roma Termini",
    "São Paulo - Luz",
    "Rio de Janeiro - Central",
    "Toronto Union Station",
    "Sydney Central Station",
    "Chhatrapati Shivaji Terminus",
    "Moscow Kazansky",
    "Beijing Railway Station",
    "Seoul Station",
];

const FALLBACK_CITIES: &[&str] = &[
    "New York",
    "London",
    "Paris",
    "Tokyo",
    "Berlin",
    "Munich",
    "Milan",
    "Rome",
    "São Paulo",
    "Rio de Janeiro",
    "Toronto",
    "Sydney",
    "Mumbai",
    "Moscow",
    "Beijing",
    "Seoul",
    "Madrid",
    "Barcelona",
    "Amsterdam",
    "Vienna",
];

const FALLBACK_LINES: &[&str] = &[
    "High-Speed Rail",
    "Commuter Rail",
    "Metro/Subway",
    "Light Rail",
    "Tram",
    "Monorail",
    "Maglev",
    "Regional Express",
    "Intercity",
    "Freight Line",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        assert!(!name().is_empty());
    }

    #[test]
    fn test_city() {
        assert!(!city().is_empty());
    }

    #[test]
    fn test_line() {
        assert!(!line().is_empty());
    }
}

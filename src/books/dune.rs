//! Dune book generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Dune character
pub fn character() -> String {
    fetch_locale("dune.characters", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_CHARACTERS).to_string())
}

/// Generate a random Dune planet
pub fn planet() -> String {
    fetch_locale("dune.planets", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_PLANETS).to_string())
}

/// Generate a random Dune quote
pub fn quote() -> String {
    fetch_locale("dune.quotes", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_QUOTES).to_string())
}

/// Generate a random Dune title
pub fn title() -> String {
    fetch_locale("dune.titles", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_TITLES).to_string())
}

// Fallback data
const FALLBACK_CHARACTERS: &[&str] = &[
    "Paul Atreides",
    "Duke Leto Atreides",
    "Lady Jessica",
    "Baron Vladimir Harkonnen",
    "Stilgar",
    "Chani",
    "Feyd-Rautha Harkonnen",
    "Emperor Shaddam IV",
    "Duncan Idaho",
    "Thufir Hawat",
    "Gurney Halleck",
    "Dr. Wellington Yueh",
    "Alia Atreides",
    "Liet Kynes",
    "Piter De Vries",
];

const FALLBACK_PLANETS: &[&str] = &[
    "Arrakis",
    "Caladan",
    "Giedi Prime",
    "Kaitain",
    "Salusa Secundus",
    "Ix",
    "Tleilax",
    "Richese",
];

const FALLBACK_QUOTES: &[&str] = &[
    "He who controls the spice controls the universe.",
    "Fear is the mind-killer.",
    "I must not fear. Fear is the mind-killer.",
    "The spice must flow.",
    "A beginning is the time for taking the most delicate care that the balances are correct.",
    "It is by will alone I set my mind in motion.",
];

const FALLBACK_TITLES: &[&str] = &[
    "Kwisatz Haderach",
    "Reverend Mother",
    "Mentat",
    "Swordmaster",
    "Padishah Emperor",
    "Fremen Naib",
    "Bene Gesserit",
    "Spacing Guild Navigator",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_character() {
        assert!(!character().is_empty());
    }

    #[test]
    fn test_planet() {
        assert!(!planet().is_empty());
    }

    #[test]
    fn test_quote() {
        assert!(!quote().is_empty());
    }

    #[test]
    fn test_title() {
        assert!(!title().is_empty());
    }
}

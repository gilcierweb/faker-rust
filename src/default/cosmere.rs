//! Cosmere generator (Brandon Sanderson's fictional universe)

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Cosmere character
pub fn character() -> String {
    fetch_locale("cosmere.characters", &crate::config::FakerConfig::locale())
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_CHARACTERS).to_string())
}

/// Generate a random Cosmere location/world
pub fn location() -> String {
    fetch_locale("cosmere.locations", &crate::config::FakerConfig::locale())
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_LOCATIONS).to_string())
}

/// Generate a random Cosmere book
pub fn book() -> String {
    fetch_locale("cosmere.books", &crate::config::FakerConfig::locale())
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_BOOKS).to_string())
}

// Fallback data
const FALLBACK_CHARACTERS: &[&str] = &[
    "Kaladin",
    "Shallan",
    "Dalinar",
    "Vin",
    "Kelsier",
    "Elend",
    "Sazed",
    "Raoden",
    "Hrathen",
    "Siri",
    "Vivenna",
    "Lightsong",
    "Waxillium",
    "Wayne",
    "Marasi",
    "Shai",
    "Hoid",
    "Adolin",
    "Navani",
    "Jasnah",
    "Renarin",
];

const FALLBACK_LOCATIONS: &[&str] = &[
    "Roshar",
    "Scadrial",
    "Sel",
    "Nalthis",
    "Taldain",
    "Threnody",
    "First of the Sun",
    "Yolen",
    "Braize",
    "Ashyn",
    "Rosharan System",
    "Scadrian System",
];

const FALLBACK_BOOKS: &[&str] = &[
    "The Way of Kings",
    "Words of Radiance",
    "Oathbringer",
    "Rhythm of War",
    "Mistborn: The Final Empire",
    "The Well of Ascension",
    "The Hero of Ages",
    "The Alloy of Law",
    "Shadows of Self",
    "The Bands of Mourning",
    "Elantris",
    "Warbreaker",
    "Arcanum Unbounded",
    "Elantris",
];

// --- gem parity generators (automated, locale-backed) ---
fn parity_pick(key: &str) -> String {
    crate::locale::fetch_locale(key, &crate::config::FakerConfig::locale())
        .map(|v| crate::base::sample(&v))
        .unwrap_or_default()
}

/// Gem-compatible generator: `cosmere.allomancers`.
pub fn allomancer() -> String {
    parity_pick("cosmere.allomancers")
}

/// Gem-compatible generator: `cosmere.aons`.
pub fn aon() -> String {
    parity_pick("cosmere.aons")
}

/// Gem-compatible generator: `cosmere.feruchemists`.
pub fn feruchemist() -> String {
    parity_pick("cosmere.feruchemists")
}

/// Gem-compatible generator: `cosmere.heralds`.
pub fn herald() -> String {
    parity_pick("cosmere.heralds")
}

/// Gem-compatible generator: `cosmere.metals`.
pub fn metal() -> String {
    parity_pick("cosmere.metals")
}

/// Gem-compatible generator: `cosmere.shards`.
pub fn shard() -> String {
    parity_pick("cosmere.shards")
}

/// Gem-compatible generator: `cosmere.shard_worlds`.
pub fn shard_world() -> String {
    parity_pick("cosmere.shard_worlds")
}

/// Gem-compatible generator: `cosmere.sprens`.
pub fn spren() -> String {
    parity_pick("cosmere.sprens")
}

/// Gem-compatible generator: `cosmere.surges`.
pub fn surge() -> String {
    parity_pick("cosmere.surges")
}
// --- gem parity generators (locale-backed) ---
/// Gem-compatible generator: Knight Radiant order.
pub fn knight_radiant() -> String {
    crate::locale::fetch_locale(
        "cosmere.knights_radiant",
        &crate::config::FakerConfig::locale(),
    )
    .map(|v| crate::base::sample(&v))
    .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_character() {
        assert!(!character().is_empty());
    }

    #[test]
    fn test_location() {
        assert!(!location().is_empty());
    }

    #[test]
    fn test_book() {
        assert!(!book().is_empty());
    }
}

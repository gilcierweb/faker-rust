//! Cosmere generator (Brandon Sanderson's fictional universe)

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Cosmere character
pub fn character() -> String {
    fetch_locale("cosmere.characters", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_CHARACTERS).to_string())
}

/// Generate a random Cosmere location/world
pub fn location() -> String {
    fetch_locale("cosmere.locations", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_LOCATIONS).to_string())
}

/// Generate a random Cosmere book
pub fn book() -> String {
    fetch_locale("cosmere.books", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_BOOKS).to_string())
}

// Fallback data
const FALLBACK_CHARACTERS: &[&str] = &[
    "Kaladin", "Shallan", "Dalinar", "Vin", "Kelsier", "Elend", "Sazed",
    "Raoden", "Hrathen", "Siri", "Vivenna", "Lightsong", "Waxillium", "Wayne",
    "Marasi", "Shai", "Hoid", "Adolin", "Navani", "Jasnah", "Renarin",
];

const FALLBACK_LOCATIONS: &[&str] = &[
    "Roshar", "Scadrial", "Sel", "Nalthis", "Taldain", "Threnody", "First of the Sun",
    "Yolen", "Braize", "Ashyn", "Rosharan System", "Scadrian System",
];

const FALLBACK_BOOKS: &[&str] = &[
    "The Way of Kings", "Words of Radiance", "Oathbringer", "Rhythm of War",
    "Mistborn: The Final Empire", "The Well of Ascension", "The Hero of Ages",
    "The Alloy of Law", "Shadows of Self", "The Bands of Mourning",
    "Elantris", "Warbreaker", "Arcanum Unbounded", "Elantris",
];

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

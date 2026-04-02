//! Avatar (James Cameron) movie generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Avatar character
pub fn character() -> String {
    fetch_locale("avatar.characters", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_CHARACTERS).to_string())
}

/// Generate a random Avatar location
pub fn location() -> String {
    fetch_locale("avatar.locations", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_LOCATIONS).to_string())
}

/// Generate a random Avatar creature
pub fn creature() -> String {
    fetch_locale("avatar.creatures", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_CREATURES).to_string())
}

// Fallback data
const FALLBACK_CHARACTERS: &[&str] = &[
    "Jake Sully",
    "Neytiri",
    "Dr. Grace Augustine",
    "Colonel Miles Quaritch",
    "Tsutey",
    "Mo'at",
    "Eytukan",
];

const FALLBACK_LOCATIONS: &[&str] = &[
    "Pandora",
    "Hometree",
    "Tree of Souls",
    "Hell's Gate",
    "Floating Mountains",
    "Omaticaya Village",
];

const FALLBACK_CREATURES: &[&str] = &[
    "Banshee",
    "Direhorse",
    "Viperwolf",
    "Thanator",
    "Titanothere",
    "Hexapede",
    "Prolemuris",
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
    fn test_creature() {
        assert!(!creature().is_empty());
    }
}

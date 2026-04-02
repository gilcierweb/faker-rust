//! TRON movie generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random TRON character
pub fn character() -> String {
    fetch_locale("tron.characters", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_CHARACTERS).to_string())
}

/// Generate a random TRON location
pub fn location() -> String {
    fetch_locale("tron.locations", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_LOCATIONS).to_string())
}

/// Generate a random TRON quote
pub fn quote() -> String {
    fetch_locale("tron.quotes", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_QUOTES).to_string())
}

// Fallback data
const FALLBACK_CHARACTERS: &[&str] = &[
    "Kevin Flynn",
    "Tron",
    "CLU",
    "Sark",
    "Master Control Program",
    "Alan Bradley",
    "Lora Baines",
    "Walter Gibbs",
    "Ed Dillinger",
    "Sam Flynn",
    "Quorra",
    "Castor",
    "Gem",
];

const FALLBACK_LOCATIONS: &[&str] = &[
    "The Grid",
    "Flynn's Arcade",
    "End of Line Club",
    "Recognizer Hangar",
    "Input/Output Tower",
    "Sea of Simulation",
];

const FALLBACK_QUOTES: &[&str] = &[
    "Greetings, programs!",
    "I fight for the users!",
    "End of line.",
    "The Grid. A digital frontier.",
    "On the other side of the screen, it all looks so easy.",
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
    fn test_quote() {
        assert!(!quote().is_empty());
    }
}

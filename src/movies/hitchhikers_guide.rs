//! Hitchhiker's Guide to the Galaxy movie generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Hitchhiker's Guide character
pub fn character() -> String {
    fetch_locale("hitchhikers_guide.characters", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_CHARACTERS).to_string())
}

/// Generate a random Hitchhiker's Guide quote
pub fn quote() -> String {
    fetch_locale("hitchhikers_guide.quotes", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_QUOTES).to_string())
}

/// Generate a random Hitchhiker's Guide location
pub fn location() -> String {
    fetch_locale("hitchhikers_guide.locations", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_LOCATIONS).to_string())
}

// Fallback data
const FALLBACK_CHARACTERS: &[&str] = &[
    "Arthur Dent",
    "Ford Prefect",
    "Zaphod Beeblebrox",
    "Trillian",
    "Marvin",
    "Slartibartfast",
    "Deep Thought",
    "Eddie",
    "Fenchurch",
];

const FALLBACK_QUOTES: &[&str] = &[
    "Don't Panic.",
    "42.",
    "So long, and thanks for all the fish.",
    "The answer to life, the universe, and everything is 42.",
    "I love deadlines. I like the whooshing sound they make as they fly by.",
    "Time is an illusion. Lunchtime doubly so.",
];

const FALLBACK_LOCATIONS: &[&str] = &[
    "Earth",
    "Magrathea",
    "Vogon Constructor Fleet",
    "Heart of Gold",
    "Milliways",
    "Frogstar World B",
    "Krikkit",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_character() {
        assert!(!character().is_empty());
    }

    #[test]
    fn test_quote() {
        assert!(!quote().is_empty());
    }

    #[test]
    fn test_location() {
        assert!(!location().is_empty());
    }
}

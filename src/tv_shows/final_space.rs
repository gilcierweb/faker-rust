//! Final Space TV show generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Final Space character
pub fn character() -> String {
    fetch_locale("final_space.characters", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_CHARACTERS).to_string())
}

/// Generate a random Final Space quote
pub fn quote() -> String {
    fetch_locale("final_space.quotes", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_QUOTES).to_string())
}

// Fallback data
const FALLBACK_CHARACTERS: &[&str] = &[
    "Gary Goodspeed",
    "Mooncake",
    "Avocato",
    "Quinn Ergon",
    "KVN",
    "H.U.E.",
    "Little Cato",
    "Ash Graven",
    "Fox",
    "Clarence",
    "Sheryl Goodspeed",
    "Tribore",
    "Invictus",
];

const FALLBACK_QUOTES: &[&str] = &[
    "Chookity!",
    "That's the power of friendship!",
    "I'm Gary Goodspeed, and I'm a good speed.",
    "KVN, get out of here!",
    "I love you, Mooncake.",
    "We're not doing that.",
    "Final Space...",
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
}

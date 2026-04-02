//! The Departed movie generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Departed character
pub fn character() -> String {
    fetch_locale("departed.characters", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_CHARACTERS).to_string())
}

/// Generate a random Departed quote
pub fn quote() -> String {
    fetch_locale("departed.quotes", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_QUOTES).to_string())
}

// Fallback data
const FALLBACK_CHARACTERS: &[&str] = &[
    "Billy Costigan",
    "Colin Sullivan",
    "Frank Costello",
    "Dignam",
    "Queenan",
    "Madolyn Madden",
    "Brown",
    "Barrigan",
];

const FALLBACK_QUOTES: &[&str] = &[
    "I'm a cop killer.",
    "I don't want to be a product of my environment. I want my environment to be a product of me.",
    "You're a cop killer, you're a cop killer.",
    "What we generally do, in fact, is lie.",
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

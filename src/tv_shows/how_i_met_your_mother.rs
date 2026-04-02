//! How I Met Your Mother TV show generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random HIMYM character
pub fn character() -> String {
    fetch_locale("how_i_met_your_mother.characters", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_CHARACTERS).to_string())
}

/// Generate a random HIMYM catchphrase
pub fn catch_phrase() -> String {
    fetch_locale("how_i_met_your_mother.catch_phrases", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_CATCH_PHRASES).to_string())}

/// Generate a random HIMYM quote
pub fn high_five() -> String {
    fetch_locale("how_i_met_your_mother.high_fives", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_HIGH_FIVES).to_string())
}

// Fallback data
const FALLBACK_CHARACTERS: &[&str] = &[
    "Ted Mosby",
    "Marshall Eriksen",
    "Lily Aldrin",
    "Barney Stinson",
    "Robin Scherbatsky",
    "Tracy McConnell",
    "Ranjit",
    "Carl",
    "Wendy",
    "Sandy Rivers",
    "Patrice",
    "Stella Zinman",
    "Victoria",
    "Zoey Pierson",
    "Quinn Garvey",
];

const FALLBACK_CATCH_PHRASES: &[&str] = &[
    "Legendary!",
    "Suit up!",
    "Challenge accepted!",
    "Have you met Ted?",
    "It's gonna be legend... wait for it... dary!",
    "True story.",
    "Daddy's home.",
    "What up!",
    "Awesome!",
];

const FALLBACK_HIGH_FIVES: &[&str] = &[
    "High Five!",
    "Self Five!",
    "Door Five!",
    "Freeze Frame High Five!",
    "Arthritis Five!",
    "Phone Five!",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_character() {
        assert!(!character().is_empty());
    }

    #[test]
    fn test_catch_phrase() {
        assert!(!catch_phrase().is_empty());
    }

    #[test]
    fn test_high_five() {
        assert!(!high_five().is_empty());
    }
}

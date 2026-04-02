//! Suits TV show generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Suits character
pub fn character() -> String {
    fetch_locale("suits.characters", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_CHARACTERS).to_string())
}

/// Generate a random Suits quote
pub fn quote() -> String {
    fetch_locale("suits.quotes", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_QUOTES).to_string())
}

// Fallback data
const FALLBACK_CHARACTERS: &[&str] = &[
    "Harvey Specter",
    "Mike Ross",
    "Louis Litt",
    "Donna Paulsen",
    "Rachel Zane",
    "Jessica Pearson",
    "Katrina Bennett",
    "Alex Williams",
    "Samantha Wheeler",
    "Katherine Heigl",
    "Daniel Hardman",
    "Travis Tanner",
];

const FALLBACK_QUOTES: &[&str] = &[
    "I don't have dreams, I have goals.",
    "You just got Litt up!",
    "I don't get lucky. I make my own luck.",
    "I'm not about caring, I'm about winning.",
    "That's the difference between you and me. You wanna lose small, I wanna win big.",
    "Life is this, and I like this.",
    "I'm not a superhero. I'm just Harvey Specter.",
    "You always have a choice.",
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

//! Archer TV show generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Archer character
pub fn character() -> String {
    fetch_locale("archer.characters", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_CHARACTERS).to_string())
}

/// Generate a random Archer quote
pub fn quote() -> String {
    fetch_locale("archer.quotes", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_QUOTES).to_string())
}

/// Generate a random Sterling Archer catchphrase
pub fn catch_phrase() -> String {
    fetch_locale("archer.catch_phrases", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_CATCH_PHRASES).to_string())
}

// Fallback data
const FALLBACK_CHARACTERS: &[&str] = &[
    "Sterling Archer",
    "Lana Kane",
    "Malory Archer",
    "Cyril Figgis",
    "Cheryl Tunt",
    "Pam Poovey",
    "Dr. Krieger",
    "Ray Gillette",
    "Woodhouse",
    "Barry Dylan",
    "Katya Kazanova",
    "Brett Buckman",
];

const FALLBACK_QUOTES: &[&str] = &[
    "Danger Zone!",
    "Phrasing!",
    "Do you want ants? Because that's how you get ants.",
    "LANA!",
    "Read a book.",
    "I'm sorry, I can't hear you over the sound of my giant paycheck.",
    "Holy shit-snacks!",
    "Boop.",
];

const FALLBACK_CATCH_PHRASES: &[&str] = &[
    "Danger Zone!",
    "Phrasing!",
    "Benoit... balls.",
    "Rampage!",
    "Sploosh!",
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
    fn test_catch_phrase() {
        assert!(!catch_phrase().is_empty());
    }
}

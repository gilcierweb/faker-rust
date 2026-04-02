//! Brooklyn Nine-Nine TV show generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Brooklyn Nine-Nine character
pub fn character() -> String {
    fetch_locale("brooklyn_nine_nine.characters", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_CHARACTERS).to_string())
}

/// Generate a random Brooklyn Nine-Nine quote
pub fn quote() -> String {
    fetch_locale("brooklyn_nine_nine.quotes", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_QUOTES).to_string())
}

// Fallback data
const FALLBACK_CHARACTERS: &[&str] = &[
    "Jake Peralta",
    "Amy Santiago",
    "Rosa Diaz",
    "Charles Boyle",
    "Raymond Holt",
    "Terry Jeffords",
    "Gina Linetti",
    "Michael Hitchcock",
    "Norm Scully",
    "Madeline Wuntch",
    "Doug Judy",
    "Kevin Cozner",
    "Adrian Pimento",
    "Cheddar",
];

const FALLBACK_QUOTES: &[&str] = &[
    "Nine-Nine!",
    "Cool, cool, cool, cool, cool.",
    "No doubt, no doubt, no doubt.",
    "Title of your sex tape.",
    "Bingpot!",
    "Toit!",
    "Velvet thunder.",
    "I was hiding in the bathroom.",
    "I'm talking about the bisexual community.",
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

//! Back to the Future movie generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Back to the Future character
pub fn character() -> String {
    fetch_locale("back_to_the_future.characters", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_CHARACTERS).to_string())
}

/// Generate a random Back to the Future date
pub fn date() -> String {
    fetch_locale("back_to_the_future.dates", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_DATES).to_string())
}

/// Generate a random Back to the Future quote
pub fn quote() -> String {
    fetch_locale("back_to_the_future.quotes", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_QUOTES).to_string())
}

// Fallback data
const FALLBACK_CHARACTERS: &[&str] = &[
    "Marty McFly",
    "Doc Brown",
    "Biff Tannen",
    "Lorraine Baines",
    "George McFly",
    "Jennifer Parker",
    "Einstein",
];

const FALLBACK_DATES: &[&str] = &[
    "October 21, 2015",
    "November 12, 1955",
    "October 26, 1985",
    "September 2, 1885",
];

const FALLBACK_QUOTES: &[&str] = &[
    "Great Scott!",
    "Where we're going, we don't need roads.",
    "1.21 gigawatts!",
    "Heavy!",
    "This is heavy, Doc.",
    "Why don't you make like a tree and get out of here?",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_character() {
        assert!(!character().is_empty());
    }

    #[test]
    fn test_date() {
        assert!(!date().is_empty());
    }

    #[test]
    fn test_quote() {
        assert!(!quote().is_empty());
    }
}

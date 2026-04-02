//! Hackers movie generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Hackers character
pub fn character() -> String {
    fetch_locale("hackers.characters", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_CHARACTERS).to_string())
}

/// Generate a random Hackers quote
pub fn quote() -> String {
    fetch_locale("hackers.quotes", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_QUOTES).to_string())
}

/// Generate a random Hackers handle/nickname
pub fn handle() -> String {
    fetch_locale("hackers.handles", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_HANDLES).to_string())
}

// Fallback data
const FALLBACK_CHARACTERS: &[&str] = &[
    "Dade Murphy",
    "Kate",
    "Joey",
    "Phantom Phreak",
    "Cereal Killer",
    "Lord Nikon",
    "Crash Override",
    "Acid Burn",
    "The Plague",
    "Agent Bob",
];

const FALLBACK_QUOTES: &[&str] = &[
    "Mess with the best, die like the rest.",
    "There is no right and wrong. There's only fun and boring.",
    "Beat that Gibson!",
    "Hack the planet!",
    "We have just gotten a wake-up call from the Nintendo generation.",
    "Crash Override, at your service.",
    "Acid Burn sez: leave B 4 u'r ex-posed.",
];

const FALLBACK_HANDLES: &[&str] = &[
    "Zero Cool",
    "Acid Burn",
    "Crash Override",
    "Phantom Phreak",
    "Cereal Killer",
    "Lord Nikon",
    "The Plague",
    "Razor",
    "Blade",
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
    fn test_handle() {
        assert!(!handle().is_empty());
    }
}

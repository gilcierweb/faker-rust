//! The Expanse TV show generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random The Expanse character
pub fn character() -> String {
    fetch_locale("the_expanse.characters", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_CHARACTERS).to_string())
}

/// Generate a random The Expanse quote
pub fn quote() -> String {
    fetch_locale("the_expanse.quotes", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_QUOTES).to_string())
}

/// Generate a random The Expanse location
pub fn location() -> String {
    fetch_locale("the_expanse.locations", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_LOCATIONS).to_string())
}

// Fallback data
const FALLBACK_CHARACTERS: &[&str] = &[
    "James Holden",
    "Naomi Nagata",
    "Amos Burton",
    "Alex Kamal",
    "Josephus Miller",
    "Julie Mao",
    "Chrisjen Avasarala",
    "Bobbie Draper",
    "Fred Johnson",
    "Camina Drummer",
    "Marco Inaros",
    "Filip Inaros",
    "Clarissa Mao",
];

const FALLBACK_QUOTES: &[&str] = &[
    "The truth is rarely pure and never simple.",
    "I'm a Belter. I was born in space.",
    "We're not so different, you and I.",
    "You can't take the sky from me.",
    "I am that guy.",
    "Remember the Cant!",
    "Beltalowda!",
    "Sa sa, bossmang.",
];

const FALLBACK_LOCATIONS: &[&str] = &[
    "Ceres Station",
    "Tycho Station",
    "Ganymede",
    "Eros",
    "The Rocinante",
    "Earth",
    "Mars",
    "The Belt",
    "Ilus",
    "Laconia",
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

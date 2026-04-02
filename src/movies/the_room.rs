//! The Room movie generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Room character
pub fn character() -> String {
    fetch_locale("the_room.characters", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_CHARACTERS).to_string())
}

/// Generate a random Room quote
pub fn quote() -> String {
    fetch_locale("the_room.quotes", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_QUOTES).to_string())
}

// Fallback data
const FALLBACK_CHARACTERS: &[&str] = &[
    "Johnny",
    "Lisa",
    "Mark",
    "Denny",
    "Peter",
    "Michelle",
    "Mike",
    "Claudette",
    "Steven",
    "Chris-R",
];

const FALLBACK_QUOTES: &[&str] = &[
    "Oh hi, Mark!",
    "You're tearing me apart, Lisa!",
    "I did not hit her, it's not true! It's bullshit! I did not hit her!",
    "Hi, doggy.",
    "You're my favorite customer.",
    "Anyway, how is your sex life?",
    "Chicken, Peter. You're just a little chicken! Cheep cheep cheep cheep!",
    "It seems to me like you're the real expert, Mark!",
    "I definitely have breast cancer.",
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

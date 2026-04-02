//! The Hobbit movie generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Hobbit character
pub fn character() -> String {
    fetch_locale("hobbit.characters", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_CHARACTERS).to_string())
}

/// Generate a random Hobbit location
pub fn location() -> String {
    fetch_locale("hobbit.locations", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_LOCATIONS).to_string())
}

/// Generate a random Hobbit quote
pub fn quote() -> String {
    fetch_locale("hobbit.quotes", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_QUOTES).to_string())
}

// Fallback data
const FALLBACK_CHARACTERS: &[&str] = &[
    "Bilbo Baggins",
    "Gandalf",
    "Thorin Oakenshield",
    "Smaug",
    "Gollum",
    "Balin",
    "Dwalin",
    "Kili",
    "Fili",
    "Bombur",
    "Bofur",
    "Bifur",
    "Oin",
    "Gloin",
    "Dori",
    "Nori",
    "Ori",
    "Thranduil",
    "Legolas",
    "Tauriel",
    "Bard",
];

const FALLBACK_LOCATIONS: &[&str] = &[
    "The Shire",
    "Bag End",
    "Rivendell",
    "Mirkwood",
    "Erebor",
    "Lake-town",
    "Lonely Mountain",
    "Goblin Town",
];

const FALLBACK_QUOTES: &[&str] = &[
    "I'm going on an adventure!",
    "What have we done, O king?",
    "If more of us valued food and cheer and song above hoarded gold, it would be a merrier world.",
    "There is nothing like looking, if you want to find something.",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_character() {
        assert!(!character().is_empty());
    }

    #[test]
    fn test_location() {
        assert!(!location().is_empty());
    }

    #[test]
    fn test_quote() {
        assert!(!quote().is_empty());
    }
}

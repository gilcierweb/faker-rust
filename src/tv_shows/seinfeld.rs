//! Seinfeld TV show generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Seinfeld character
pub fn character() -> String {
    fetch_locale("seinfeld.characters", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_CHARACTERS).to_string())
}

/// Generate a random Seinfeld quote
pub fn quote() -> String {
    fetch_locale("seinfeld.quotes", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_QUOTES).to_string())
}

/// Generate a random Seinfeld location
pub fn location() -> String {
    fetch_locale("seinfeld.locations", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_LOCATIONS).to_string())
}

// Fallback data
const FALLBACK_CHARACTERS: &[&str] = &[
    "Jerry Seinfeld",
    "George Costanza",
    "Elaine Benes",
    "Cosmo Kramer",
    "Newman",
    "Frank Costanza",
    "Estelle Costanza",
    "Uncle Leo",
    "Morty Seinfeld",
    "Helen Seinfeld",
    "Susan Ross",
    "David Puddy",
    "Jackie Chiles",
];

const FALLBACK_QUOTES: &[&str] = &[
    "No soup for you!",
    "It's not a lie if you believe it.",
    "Yada, yada, yada...",
    "These pretzels are making me thirsty!",
    "Serenity now!",
    "Not that there's anything wrong with that.",
    "Get out!",
    "Hello, Newman.",
    "I'm the master of my domain.",
];

const FALLBACK_LOCATIONS: &[&str] = &[
    "Jerry's Apartment",
    "Monk's Cafe",
    "George's Apartment",
    "Kramer's Apartment",
    "Elaine's Apartment",
    "Newman's Apartment",
    "The Soup Nazi's Kitchen",
    "Reggie's",
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

//! The Lord of the Rings movie generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Lord of the Rings character
pub fn character() -> String {
    fetch_locale("lord_of_the_rings.characters", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_CHARACTERS).to_string())
}

/// Generate a random Lord of the Rings location
pub fn location() -> String {
    fetch_locale("lord_of_the_rings.locations", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_LOCATIONS).to_string())
}

/// Generate a random Lord of the Rings quote
pub fn quote() -> String {
    fetch_locale("lord_of_the_rings.quotes", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_QUOTES).to_string())
}

// Fallback data
const FALLBACK_CHARACTERS: &[&str] = &[
    "Frodo Baggins",
    "Samwise Gamgee",
    "Aragorn",
    "Legolas",
    "Gimli",
    "Gandalf",
    "Boromir",
    "Meriadoc Brandybuck",
    "Peregrin Took",
    "Arwen",
    "Galadriel",
    "Elrond",
    "Saruman",
    "Sauron",
    "Gollum",
    "Bilbo Baggins",
    "Théoden",
    "Éowyn",
    "Faramir",
];

const FALLBACK_LOCATIONS: &[&str] = &[
    "The Shire",
    "Rivendell",
    "Mordor",
    "Gondor",
    "Rohan",
    "Isengard",
    "Lothlórien",
    "Minas Tirith",
    "Helm's Deep",
    "Mount Doom",
    "Fangorn Forest",
    "Moria",
];

const FALLBACK_QUOTES: &[&str] = &[
    "One does not simply walk into Mordor.",
    "My precious.",
    "You shall not pass!",
    "A wizard is never late, nor is he early. He arrives precisely when he means to.",
    "Even the smallest person can change the course of the future.",
    "There is some good in this world, and it's worth fighting for.",
    "I can't carry it for you, but I can carry you!",
    "Fly, you fools!",
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

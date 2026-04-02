//! Family Guy TV show generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Family Guy character
pub fn character() -> String {
    fetch_locale("family_guy.characters", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_CHARACTERS).to_string())
}

/// Generate a random Family Guy quote
pub fn quote() -> String {
    fetch_locale("family_guy.quotes", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_QUOTES).to_string())
}

/// Generate a random Family Guy location
pub fn location() -> String {
    fetch_locale("family_guy.locations", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_LOCATIONS).to_string())
}

// Fallback data
const FALLBACK_CHARACTERS: &[&str] = &[
    "Peter Griffin",
    "Lois Griffin",
    "Meg Griffin",
    "Chris Griffin",
    "Stewie Griffin",
    "Brian Griffin",
    "Glenn Quagmire",
    "Cleveland Brown",
    "Joe Swanson",
    "Bonnie Swanson",
    "Herbert",
    "Ernie the Giant Chicken",
    "Carter Pewterschmidt",
    "Barbara Pewterschmidt",
    "Mayor Adam West",
    "Consuela",
];

const FALLBACK_QUOTES: &[&str] = &[
    "Freakin' sweet!",
    "Giggity giggity goo!",
    "What the deuce?",
    "Victory is mine!",
    "Shut up, Meg.",
    "Bird is the word.",
    "Roadhouse!",
    "That's what really grinds my gears.",
    "All right!",
    "Damn it, Brian!",
];

const FALLBACK_LOCATIONS: &[&str] = &[
    "31 Spooner Street",
    "Quahog",
    "The Drunken Clam",
    "Quahog Hospital",
    "Quahog Police Department",
    "James Woods Regional High School",
    "Goldman's Pharmacy",
    "Griffin Family Home",
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

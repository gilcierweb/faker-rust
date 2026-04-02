//! The Big Lebowski movie generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Big Lebowski character
pub fn character() -> String {
    fetch_locale("lebowski.characters", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_CHARACTERS).to_string())
}

/// Generate a random Big Lebowski quote
pub fn quote() -> String {
    fetch_locale("lebowski.quotes", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_QUOTES).to_string())
}

// Fallback data
const FALLBACK_CHARACTERS: &[&str] = &[
    "Jeff 'The Dude' Lebowski",
    "Walter Sobchak",
    "Donny Kerabatsos",
    "Maude Lebowski",
    "The Big Lebowski",
    "Jesus Quintana",
    "Bunny Lebowski",
    "Karl Hungus",
    "The Stranger",
    "Brandt",
];

const FALLBACK_QUOTES: &[&str] = &[
    "The Dude abides.",
    "That rug really tied the room together.",
    "Yeah, well, you know, that's just, like, your opinion, man.",
    "This aggression will not stand, man.",
    "I'm the Dude, so that's what you call me.",
    "Eight-year-olds, Dude.",
    "Nobody f***s with the Jesus.",
    "Mark it zero!",
    "You want a toe? I can get you a toe.",
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

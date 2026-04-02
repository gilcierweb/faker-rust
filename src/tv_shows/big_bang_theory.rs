//! The Big Bang Theory TV show generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Big Bang Theory character
pub fn character() -> String {
    fetch_locale("big_bang_theory.characters", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_CHARACTERS).to_string())
}

/// Generate a random Big Bang Theory quote
pub fn quote() -> String {
    fetch_locale("big_bang_theory.quotes", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_QUOTES).to_string())
}

/// Generate a random Big Bang Theory item/gadget
pub fn gadget() -> String {
    fetch_locale("big_bang_theory.gadgets", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_GADGETS).to_string())
}

// Fallback data
const FALLBACK_CHARACTERS: &[&str] = &[
    "Sheldon Cooper",
    "Leonard Hofstadter",
    "Penny",
    "Howard Wolowitz",
    "Raj Koothrappali",
    "Amy Farrah Fowler",
    "Bernadette Rostenkowski",
    "Stuart Bloom",
    "Leslie Winkle",
    "Barry Kripke",
    "Beverly Hofstadter",
    "Mary Cooper",
    "Mrs. Wolowitz",
    "Emily Sweeney",
    "Lucy",
];

const FALLBACK_QUOTES: &[&str] = &[
    "Bazinga!",
    "That's my spot!",
    "Soft kitty, warm kitty...",
    "I'm not crazy. My mother had me tested.",
    "Penny. Penny. Penny.",
    "It's a trap!",
    "Wheaton!",
    "Howard, you know I'm not allowed to eat at the desk.",
];

const FALLBACK_GADGETS: &[&str] = &[
    "Time Machine",
    "The Dice",
    "The Clothes Folder",
    "The Napkin Holder",
    "The Flash Costume",
    "Green Lantern Ring",
    "Thor's Hammer",
    "Battlestar Galactica",
    "Star Trek Communicator",
    "DNA Model",
    "Robot",
    "Tardis",
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
    fn test_gadget() {
        assert!(!gadget().is_empty());
    }
}

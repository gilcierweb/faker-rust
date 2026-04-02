//! Aqua Teen Hunger Force TV show generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Aqua Teen Hunger Force character
pub fn character() -> String {
    fetch_locale("aqua_teen_hunger_force.characters", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_CHARACTERS).to_string())
}

/// Generate a random Aqua Teen Hunger Force quote
pub fn quote() -> String {
    fetch_locale("aqua_teen_hunger_force.quotes", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_QUOTES).to_string())
}

// Fallback data
const FALLBACK_CHARACTERS: &[&str] = &[
    "Master Shake",
    "Frylock",
    "Meatwad",
    "Carl Brutananadilewski",
    "Dr. Weird",
    "Steve",
    "The Mooninites",
    "Err",
    "Ignignokt",
    "Oglethorpe",
    "Emory",
    "Cybernetic Ghost",
    "MC Pee Pants",
    "Hand Banana",
];

const FALLBACK_QUOTES: &[&str] = &[
    "Dancing is forbidden!",
    "I am the Drizzle!",
    "Tonight... you.",
    "Do what now?",
    "Gentlemen, behold!",
    "I want my name to be Spaghetti.",
    "It doesn't matter. None of this matters.",
    "My mind is telling me no...",
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

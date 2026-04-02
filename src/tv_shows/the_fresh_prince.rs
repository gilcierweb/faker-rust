//! The Fresh Prince of Bel-Air TV show generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Fresh Prince character
pub fn character() -> String {
    fetch_locale("fresh_prince.characters", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_CHARACTERS).to_string())
}

/// Generate a random Fresh Prince quote
pub fn quote() -> String {
    fetch_locale("fresh_prince.quotes", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_QUOTES).to_string())
}

// Fallback data
const FALLBACK_CHARACTERS: &[&str] = &[
    "Will Smith",
    "Philip Banks",
    "Vivian Banks",
    "Carlton Banks",
    "Hilary Banks",
    "Ashley Banks",
    "Geoffrey Butler",
    "Jazz",
    "Nicky Banks",
    "Vy Smith",
];

const FALLBACK_QUOTES: &[&str] = &[
    "Yo homes, smell ya later!",
    "How come he don't want me, man?",
    "Carlton, you need to learn some rhythm.",
    "I'm the Prince of Bel-Air!",
    "Woo!",
    "Hey, hey, hey!",
    "That's not what I'm talking about.",
    "He got in one little fight and his mom got scared.",
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

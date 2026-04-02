//! Dumb and Dumber movie/TV generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Dumb and Dumber character
pub fn character() -> String {
    fetch_locale("dumb_and_dumber.characters", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_CHARACTERS).to_string())
}

/// Generate a random Dumb and Dumber quote
pub fn quote() -> String {
    fetch_locale("dumb_and_dumber.quotes", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_QUOTES).to_string())
}

// Fallback data
const FALLBACK_CHARACTERS: &[&str] = &[
    "Lloyd Christmas",
    "Harry Dunne",
    "Mary Swanson",
    "Joe Mentalino",
    "J.P. Shay",
    "Nicholas Andre",
    "Sea Bass",
    "Beth Jordan",
    "Fraida Felcher",
    "Penny Pinchelow",
];

const FALLBACK_QUOTES: &[&str] = &[
    "So you're telling me there's a chance?",
    "I like it a lot.",
    "Just when I thought you couldn't possibly be any dumber...",
    "Wanna hear the most annoying sound in the world?",
    "We got no food, we got no jobs... our pets' heads are falling off!",
    "Pretty bird... pretty bird...",
    "Kick his ass, Sea Bass!",
    "Harry, I took care of it.",
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

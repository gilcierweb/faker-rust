//! Buffy the Vampire Slayer generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Buffy character
pub fn character() -> String {
    fetch_locale("buffy.characters", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Buffy Summers".to_string())
}

/// Generate a random Buffy quote
pub fn quote() -> String {
    fetch_locale("buffy.quotes", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "I'm the Slayer. The Chosen One.".to_string())
}

/// Generate a random Buffy actor
pub fn actor() -> String {
    fetch_locale("buffy.actors", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Sarah Michelle Gellar".to_string())
}

/// Generate a random Buffy big bad (villain)
pub fn big_bad() -> String {
    fetch_locale("buffy.big_bads", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "The Master".to_string())
}

/// Generate a random Buffy episode
pub fn episode() -> String {
    fetch_locale("buffy.episodes", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Welcome to the Hellmouth".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffy_generation() {
        assert!(!character().is_empty());
        assert!(!quote().is_empty());
        assert!(!actor().is_empty());
        assert!(!big_bad().is_empty());
        assert!(!episode().is_empty());
    }
}

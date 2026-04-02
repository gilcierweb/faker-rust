//! Rick and Morty generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Rick and Morty character
pub fn character() -> String {
    fetch_locale("rick_and_morty.characters", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Rick Sanchez".to_string())
}

/// Generate a random Rick and Morty location
pub fn location() -> String {
    fetch_locale("rick_and_morty.locations", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Earth (C-137)".to_string())
}

/// Generate a random Rick and Morty quote
pub fn quote() -> String {
    fetch_locale("rick_and_morty.quotes", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Wubba Lubba Dub Dub!".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rick_and_morty_generation() {
        assert!(!character().is_empty());
        assert!(!location().is_empty());
        assert!(!quote().is_empty());
    }
}

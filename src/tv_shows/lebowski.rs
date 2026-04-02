//! The Big Lebowski generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Lebowski actor
pub fn actor() -> String {
    fetch_locale("lebowski.actors", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Jeff Bridges".to_string())
}

/// Generate a random Lebowski character
pub fn character() -> String {
    fetch_locale("lebowski.characters", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "The Dude".to_string())
}

/// Generate a random Lebowski quote
pub fn quote() -> String {
    fetch_locale("lebowski.quotes", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "The Dude abides.".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lebowski_generation() {
        assert!(!actor().is_empty());
        assert!(!character().is_empty());
        assert!(!quote().is_empty());
    }
}

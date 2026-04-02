//! The Simpsons generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Simpsons character
pub fn character() -> String {
    fetch_locale("simpsons.characters", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Homer Simpson".to_string())
}

/// Generate a random Simpsons location
pub fn location() -> String {
    fetch_locale("simpsons.locations", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Springfield".to_string())
}

/// Generate a random Simpsons quote
pub fn quote() -> String {
    fetch_locale("simpsons.quotes", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "D'oh!".to_string())
}

/// Generate a random Simpsons episode title
pub fn episode_title() -> String {
    fetch_locale("simpsons.episode_titles", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Simpsons Roasting on an Open Fire".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simpsons_generation() {
        assert!(!character().is_empty());
        assert!(!location().is_empty());
        assert!(!quote().is_empty());
        assert!(!episode_title().is_empty());
    }
}

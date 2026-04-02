//! Stranger Things generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Stranger Things character
pub fn character() -> String {
    fetch_locale("stranger_things.character", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Eleven".to_string())
}

/// Generate a random Stranger Things quote
pub fn quote() -> String {
    fetch_locale("stranger_things.quote", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Friends don't lie.".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stranger_things_generation() {
        assert!(!character().is_empty());
        assert!(!quote().is_empty());
    }
}

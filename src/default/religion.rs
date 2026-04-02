//! Religion/Bible generator - generates quotes, character names, and locations from religious texts

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random quote from the Bible
pub fn quote() -> String {
    fetch_locale("bible.quote", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "The Lord is my shepherd; I shall not want.".to_string())
}

/// Generate a random character name from the Bible
pub fn character() -> String {
    fetch_locale("bible.character", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Noah".to_string())
}

/// Generate a random location from the Bible
pub fn location() -> String {
    fetch_locale("bible.location", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Jerusalem".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_religion_generation() {
        assert!(!quote().is_empty());
        assert!(!character().is_empty());
        assert!(!location().is_empty());
    }
}

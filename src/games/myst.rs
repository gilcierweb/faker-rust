//! Myst generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Myst game title
pub fn game() -> String {
    fetch_locale("myst.games", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Myst".to_string())
}

/// Generate a random Myst creature
pub fn creature() -> String {
    fetch_locale("myst.creatures", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Achenar".to_string())
}

/// Generate a random Myst character
pub fn character() -> String {
    fetch_locale("myst.characters", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Atrus".to_string())
}

/// Generate a random Myst Age
pub fn age() -> String {
    fetch_locale("myst.ages", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Selenitic Age".to_string())
}

/// Generate a random Myst quote
pub fn quote() -> String {
    fetch_locale("myst.quotes", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "I realized, is that they do not truly exist".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_myst_generation() {
        assert!(!game().is_empty());
        assert!(!creature().is_empty());
        assert!(!character().is_empty());
        assert!(!age().is_empty());
        assert!(!quote().is_empty());
    }
}

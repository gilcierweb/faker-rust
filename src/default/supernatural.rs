//! Supernatural generator - generates character names, weapons, and creatures from the Supernatural series

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random character name from Supernatural
pub fn character() -> String {
    fetch_locale("supernatural.character", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Dean Winchester".to_string())
}

/// Generate a random weapon from Supernatural
pub fn weapon() -> String {
    fetch_locale("supernatural.weapon", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "The Colt".to_string())
}

/// Generate a random creature from Supernatural
pub fn creature() -> String {
    fetch_locale("supernatural.creature", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Wendigo".to_string())
}

/// Generate a random location from Supernatural
pub fn location() -> String {
    fetch_locale("supernatural.location", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Men of Letters Bunker".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_supernatural_generation() {
        assert!(!character().is_empty());
        assert!(!weapon().is_empty());
        assert!(!creature().is_empty());
        assert!(!location().is_empty());
    }
}

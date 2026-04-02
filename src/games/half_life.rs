//! Half-Life generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Half-Life character
pub fn character() -> String {
    fetch_locale("half_life.character", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Gordon Freeman".to_string())
}

/// Generate a random Half-Life enemy
pub fn enemy() -> String {
    fetch_locale("half_life.enemy", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Headcrab".to_string())
}

/// Generate a random Half-Life location
pub fn location() -> String {
    fetch_locale("half_life.location", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Black Mesa".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_half_life_generation() {
        assert!(!character().is_empty());
        assert!(!enemy().is_empty());
        assert!(!location().is_empty());
    }
}

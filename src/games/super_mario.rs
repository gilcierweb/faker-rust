//! Super Mario generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Super Mario character
pub fn character() -> String {
    fetch_locale("super_mario.characters", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Mario".to_string())
}

/// Generate a random Super Mario game title
pub fn game() -> String {
    fetch_locale("super_mario.games", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Super Mario Bros.".to_string())
}

/// Generate a random Super Mario location
pub fn location() -> String {
    fetch_locale("super_mario.locations", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Mushroom Kingdom".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_super_mario_generation() {
        assert!(!character().is_empty());
        assert!(!game().is_empty());
        assert!(!location().is_empty());
    }
}

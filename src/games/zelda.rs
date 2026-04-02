//! The Legend of Zelda generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Zelda game title
pub fn game() -> String {
    fetch_locale("zelda.games", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "The Legend of Zelda: Breath of the Wild".to_string())
}

/// Generate a random Zelda character
pub fn character() -> String {
    fetch_locale("zelda.characters", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Link".to_string())
}

/// Generate a random Zelda location
pub fn location() -> String {
    fetch_locale("zelda.locations", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Hyrule".to_string())
}

/// Generate a random Zelda item
pub fn item() -> String {
    fetch_locale("zelda.items", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Master Sword".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zelda_generation() {
        assert!(!game().is_empty());
        assert!(!character().is_empty());
        assert!(!location().is_empty());
        assert!(!item().is_empty());
    }
}

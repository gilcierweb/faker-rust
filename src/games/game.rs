//! Generic Game generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Game title
pub fn title() -> String {
    fetch_locale("game.title", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Super Mario Bros.".to_string())
}

/// Generate a random Game genre
pub fn genre() -> String {
    fetch_locale("game.genre", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Platformer".to_string())
}

/// Generate a random Game platform
pub fn platform() -> String {
    fetch_locale("game.platform", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Nintendo Entertainment System".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_generation() {
        assert!(!title().is_empty());
        assert!(!genre().is_empty());
        assert!(!platform().is_empty());
    }
}

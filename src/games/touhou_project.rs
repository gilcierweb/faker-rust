//! Touhou Project generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Touhou Project game title
pub fn game() -> String {
    fetch_locale("games.touhou.games", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Embodiment of Scarlet Devil".to_string())
}

/// Generate a random Touhou Project character
pub fn character() -> String {
    fetch_locale("games.touhou.characters", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Reimu Hakurei".to_string())
}

/// Generate a random Touhou Project spell card
pub fn spell_card() -> String {
    fetch_locale("games.touhou.spell_cards", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Fantasy Seal".to_string())
}

/// Generate a random Touhou Project location
pub fn location() -> String {
    fetch_locale("games.touhou.locations", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Hakurei Shrine".to_string())
}

/// Generate a random Touhou Project song title
pub fn song() -> String {
    fetch_locale("games.touhou.songs", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Septette for the Dead Princess".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_touhou_generation() {
        assert!(!game().is_empty());
        assert!(!character().is_empty());
        assert!(!spell_card().is_empty());
        assert!(!location().is_empty());
        assert!(!song().is_empty());
    }
}

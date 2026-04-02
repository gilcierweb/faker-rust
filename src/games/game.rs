//! Game generator - generates random game titles, genres, and platforms

use crate::base::sample;
use crate::locale::{fetch_locale_with_context, sample_with_resolve};

/// Generate a random game title
pub fn title() -> String {
    fetch_locale_with_context("game.title", "en", Some("game"))
        .map(|v| sample_with_resolve(&v, Some("game")))
        .unwrap_or_else(|| sample(FALLBACK_TITLES).to_string())
}

/// Generate a random game genre
pub fn genre() -> String {
    fetch_locale_with_context("game.genre", "en", Some("game"))
        .map(|v| sample_with_resolve(&v, Some("game")))
        .unwrap_or_else(|| sample(FALLBACK_GENRES).to_string())
}

/// Generate a random game platform
pub fn platform() -> String {
    fetch_locale_with_context("game.platform", "en", Some("game"))
        .map(|v| sample_with_resolve(&v, Some("game")))
        .unwrap_or_else(|| sample(FALLBACK_PLATFORMS).to_string())
}

// Fallback data
const FALLBACK_TITLES: &[&str] = &[
    "The Legend of Zelda",
    "Super Mario Bros.",
    "Half-Life",
    "Portal",
    "Minecraft",
    "Dota 2",
];

const FALLBACK_GENRES: &[&str] = &[
    "First-person shooter",
    "RPG",
    "Platformer",
    "Puzzle",
    "Strategy",
];

const FALLBACK_PLATFORMS: &[&str] = &[
    "PC",
    "PlayStation 4",
    "Xbox One",
    "Nintendo Switch",
    "Mobile",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_title() {
        assert!(!title().is_empty());
    }
}

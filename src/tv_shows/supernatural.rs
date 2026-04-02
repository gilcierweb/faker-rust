//! Supernatural TV show generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Supernatural character
pub fn character() -> String {
    fetch_locale("supernatural.characters", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_CHARACTERS).to_string())
}

/// Generate a random Supernatural quote
pub fn quote() -> String {
    fetch_locale("supernatural.quotes", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_QUOTES).to_string())
}

/// Generate a random Supernatural monster/creature
pub fn creature() -> String {
    fetch_locale("supernatural.creatures", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_CREATURES).to_string())
}

// Fallback data
const FALLBACK_CHARACTERS: &[&str] = &[
    "Sam Winchester",
    "Dean Winchester",
    "Castiel",
    "Crowley",
    "Bobby Singer",
    "John Winchester",
    "Mary Winchester",
    "Lucifer",
    "Gabriel",
    "Chuck/God",
    "Jack Kline",
    "Rowena MacLeod",
    "Jody Mills",
    "Charlie Bradbury",
];

const FALLBACK_QUOTES: &[&str] = &[
    "Saving people, hunting things. The family business.",
    "Driver picks the music, shotgun shuts his cakehole.",
    "I'm Batman.",
    "I lost my shoe!",
    "Hello, boys.",
    "Idjit!",
    "Assbutt!",
    "Dean and I do share a more profound bond.",
    "The family business.",
    "It's called anime. And it's an art form.",
];

const FALLBACK_CREATURES: &[&str] = &[
    "Vampire",
    "Werewolf",
    "Demon",
    "Angel",
    "Ghost",
    "Wendigo",
    "Shapeshifter",
    "Leviathan",
    "Nephilim",
    "Chimera",
    "Hellhound",
    "Djinn",
    "Kitsune",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_character() {
        assert!(!character().is_empty());
    }

    #[test]
    fn test_quote() {
        assert!(!quote().is_empty());
    }

    #[test]
    fn test_creature() {
        assert!(!creature().is_empty());
    }
}

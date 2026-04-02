//! Ghostbusters movie generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Ghostbusters character
pub fn character() -> String {
    fetch_locale("ghostbusters.characters", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_CHARACTERS).to_string())
}

/// Generate a random Ghostbusters quote
pub fn quote() -> String {
    fetch_locale("ghostbusters.quotes", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_QUOTES).to_string())
}

/// Generate a random ghost or creature name
pub fn creature() -> String {
    fetch_locale("ghostbusters.creatures", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_CREATURES).to_string())
}

// Fallback data
const FALLBACK_CHARACTERS: &[&str] = &[
    "Peter Venkman",
    "Ray Stantz",
    "Egon Spengler",
    "Winston Zeddemore",
    "Dana Barrett",
    "Louis Tully",
    "Janine Melnitz",
    "Walter Peck",
    "Gozer",
];

const FALLBACK_QUOTES: &[&str] = &[
    "Who you gonna call? Ghostbusters!",
    "I ain't afraid of no ghost.",
    "Don't cross the streams!",
    "Back off, man. I'm a scientist.",
    "Dogs and cats living together... mass hysteria!",
    "He slimed me.",
    "Let's show this prehistoric bitch how we do things downtown!",
];

const FALLBACK_CREATURES: &[&str] = &[
    "Slimer",
    "Stay Puft Marshmallow Man",
    "Zuul",
    "Vinz Clortho",
    "Gozer the Gozerian",
    "Terror Dog",
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

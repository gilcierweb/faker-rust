//! Cowboy Bebop anime generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Cowboy Bebop character
pub fn character() -> String {
    fetch_locale("cowboy_bebop.characters", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_CHARACTERS).to_string())
}

/// Generate a random Cowboy Bebop quote
pub fn quote() -> String {
    fetch_locale("cowboy_bebop.quotes", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_QUOTES).to_string())
}

/// Generate a random Cowboy Bebop spaceship
pub fn ship() -> String {
    fetch_locale("cowboy_bebop.ships", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_SHIPS).to_string())
}

// Fallback data
const FALLBACK_CHARACTERS: &[&str] = &[
    "Spike Spiegel",
    "Jet Black",
    "Faye Valentine",
    "Edward Wong Hau Pepelu Tivrusky IV",
    "Ein",
    "Vicious",
    "Julia",
    "Punch",
    "Judy",
    "Gren",
];

const FALLBACK_QUOTES: &[&str] = &[
    "See you, space cowboy...",
    "Whatever happens, happens.",
    "I'm not going there to die. I'm going to find out if I'm really alive.",
    "The past is the past. The future is now.",
    "Bang.",
    "I have a bad feeling about this.",
    "You're gonna carry that weight.",
];

const FALLBACK_SHIPS: &[&str] = &[
    "Swordfish II",
    "Red Tail",
    "Hammer Head",
    "Bebop",
    "Mono Boat",
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
    fn test_ship() {
        assert!(!ship().is_empty());
    }
}

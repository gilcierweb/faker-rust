//! Doraemon anime/manga generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Doraemon character
pub fn character() -> String {
    fetch_locale("doraemon.characters", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_CHARACTERS).to_string())
}

/// Generate a random Doraemon gadget
pub fn gadget() -> String {
    fetch_locale("doraemon.gadgets", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_GADGETS).to_string())
}

/// Generate a random Doraemon location
pub fn location() -> String {
    fetch_locale("doraemon.locations", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_LOCATIONS).to_string())
}

// Fallback data
const FALLBACK_CHARACTERS: &[&str] = &[
    "Doraemon",
    "Nobita Nobi",
    "Shizuka Minamoto",
    "Takeshi Goda (Gian)",
    "Suneo Honekawa",
    "Dorami",
    "Nobisuke Nobi",
    "Tamako Nobi",
    "Sewashi Nobi",
    "Jaiko Goda",
    "Mii-chan",
    "Dekisugi Hidetoshi",
];

const FALLBACK_GADGETS: &[&str] = &[
    "Anywhere Door",
    "Bamboo Copter",
    "Time Machine",
    "Doraemon's Pocket",
    "Small Light",
    "Big Light",
    "Translation Gummy",
    "Memory Bread",
    "Time Cloth",
    "4D Pocket",
    "Take-copter",
    "Pass Loop",
    "Air Cannon",
    "Gadget Cat",
    "Doctor Bag",
];

const FALLBACK_LOCATIONS: &[&str] = &[
    "Nobita's House",
    "School",
    "Empty Lot",
    "Gian's House",
    "Suneo's House",
    "Shizuka's House",
    "22nd Century",
    "Future Department Store",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_character() {
        assert!(!character().is_empty());
    }

    #[test]
    fn test_gadget() {
        assert!(!gadget().is_empty());
    }

    #[test]
    fn test_location() {
        assert!(!location().is_empty());
    }
}

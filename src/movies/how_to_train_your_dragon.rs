//! How to Train Your Dragon movie generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random How to Train Your Dragon character
pub fn character() -> String {
    fetch_locale("how_to_train_your_dragon.characters", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_CHARACTERS).to_string())
}

/// Generate a random How to Train Your Dragon dragon
pub fn dragon() -> String {
    fetch_locale("how_to_train_your_dragon.dragons", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_DRAGONS).to_string())
}

/// Generate a random How to Train Your Dragon location
pub fn location() -> String {
    fetch_locale("how_to_train_your_dragon.locations", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_LOCATIONS).to_string())
}

// Fallback data
const FALLBACK_CHARACTERS: &[&str] = &[
    "Hiccup Horrendous Haddock III",
    "Astrid Hofferson",
    "Stoick the Vast",
    "Gobber the Belch",
    "Fishlegs Ingerman",
    "Snotlout Jorgenson",
    "Tuffnut Thorston",
    "Ruffnut Thorston",
    "Eret",
    "Valka",
    "Drago Bludvist",
];

const FALLBACK_DRAGONS: &[&str] = &[
    "Toothless",
    "Stormfly",
    "Meatlug",
    "Hookfang",
    "Barf and Belch",
    "Cloudjumper",
    "Skullcrusher",
    "Grump",
    "Light Fury",
    "Bewilderbeast",
    "Red Death",
];

const FALLBACK_LOCATIONS: &[&str] = &[
    "Berk",
    "Hidden World",
    "Dragon's Edge",
    "Valka's Mountain",
    "Isle of Berk",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_character() {
        assert!(!character().is_empty());
    }

    #[test]
    fn test_dragon() {
        assert!(!dragon().is_empty());
    }

    #[test]
    fn test_location() {
        assert!(!location().is_empty());
    }
}

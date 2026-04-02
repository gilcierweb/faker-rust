//! Detective Conan anime/manga generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Detective Conan character
pub fn character() -> String {
    fetch_locale("conan.characters", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_CHARACTERS).to_string())
}

/// Generate a random Detective Conan gadget
pub fn gadget() -> String {
    fetch_locale("conan.gadgets", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_GADGETS).to_string())
}

/// Generate a random Detective Conan location
pub fn location() -> String {
    fetch_locale("conan.locations", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_LOCATIONS).to_string())
}

// Fallback data
const FALLBACK_CHARACTERS: &[&str] = &[
    "Conan Edogawa",
    "Shinichi Kudo",
    "Ran Mouri",
    "Kogoro Mouri",
    "Ai Haibara",
    "Ayumi Yoshida",
    "Mitsuhiko Tsuburaya",
    "Genta Kojima",
    "Hiroshi Agasa",
    "Sonoko Suzuki",
    "Heiji Hattori",
    "Kazuha Toyama",
    "Juzo Megure",
    "Wataru Takagi",
    "Miwako Sato",
    "Ninzaburo Shiratori",
    "Gin",
    "Vodka",
    "Vermouth",
    "Kaito Kid",
];

const FALLBACK_GADGETS: &[&str] = &[
    "Voice-Changing Bowtie",
    "Stun-Gun Wristwatch",
    "Solar-Powered Skateboard",
    "Detective Boys Badge",
    "Criminal Tracking Glasses",
    "Voice-Changing Face Mask",
    "Watch-Type Radio",
    "Elasticity Suspenders",
    "Power-Enhancing Kick Shoes",
    " Turbo Engine Skateboard",
    "Anywhere Ball Dispensing Belt",
];

const FALLBACK_LOCATIONS: &[&str] = &[
    "Beika City",
    "Beika Elementary School",
    "Mouri Detective Agency",
    "Professor Agasa's House",
    "Teitan Elementary School",
    "Police Headquarters",
    "Beika Department Store",
    "Tropical Land",
    "Haido City Hotel",
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

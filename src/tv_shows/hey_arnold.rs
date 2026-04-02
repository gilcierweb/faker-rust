//! Hey Arnold! TV show generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Hey Arnold character
pub fn character() -> String {
    fetch_locale("hey_arnold.characters", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_CHARACTERS).to_string())
}

/// Generate a random Hey Arnold quote
pub fn quote() -> String {
    fetch_locale("hey_arnold.quotes", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_QUOTES).to_string())
}

/// Generate a random Hey Arnold location
pub fn location() -> String {
    fetch_locale("hey_arnold.locations", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_LOCATIONS).to_string())
}

// Fallback data
const FALLBACK_CHARACTERS: &[&str] = &[
    "Arnold Shortman",
    "Gerald Johanssen",
    "Helga Pataki",
    "Phoebe Heyerdahl",
    "Harold Berman",
    "Sid",
    "Stinky Peterson",
    "Rhonda Wellington Lloyd",
    "Eugene Horowitz",
    "Curly Gammelthorpe",
    "Grandpa Phil",
    "Grandma Gertie",
    "Oskar Kokoshka",
    "Susie Kokoshka",
    "Ernie Potts",
];

const FALLBACK_QUOTES: &[&str] = &[
    "Hey Arnold!",
    "Football head!",
    "Move it, Football Head!",
    "I'm Helga G. Pataki, and I love Arnold!",
    "You are a bold kid.",
    "Stoop Kid's afraid to leave his stoop!",
    "Monkeys! I love monkeys!",
];

const FALLBACK_LOCATIONS: &[&str] = &[
    "Sunset Arms Boarding House",
    "P.S. 118",
    "Arnold's Room",
    "Helga's Shrine",
    "Stoop Kid's Stoop",
    "Green Meats",
    "Slausen's Ice Cream",
    "Civic Center",
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
    fn test_location() {
        assert!(!location().is_empty());
    }
}

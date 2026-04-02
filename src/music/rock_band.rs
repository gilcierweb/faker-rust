//! Rock Band generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random rock band name
pub fn name() -> String {
    fetch_locale("rock_band.names", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_NAMES).to_string())
}

// Fallback data
const FALLBACK_NAMES: &[&str] = &[
    "Led Zeppelin",
    "The Beatles",
    "Pink Floyd",
    "The Rolling Stones",
    "Queen",
    "The Who",
    "The Doors",
    "Nirvana",
    "AC/DC",
    "Metallica",
    "Guns N' Roses",
    "Aerosmith",
    "The Jimi Hendrix Experience",
    "Red Hot Chili Peppers",
    "Foo Fighters",
    "Radiohead",
    "U2",
    "Coldplay",
    "The Killers",
    "Arctic Monkeys",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        assert!(!name().is_empty());
    }
}

//! Rush music generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Rush song
pub fn song() -> String {
    fetch_locale("rush.songs", &crate::config::FakerConfig::locale())
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_SONGS).to_string())
}

/// Generate a random Rush album
pub fn album() -> String {
    fetch_locale("rush.albums", &crate::config::FakerConfig::locale())
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_ALBUMS).to_string())
}

/// Generate a random Rush member
pub fn member() -> String {
    fetch_locale("rush.members", &crate::config::FakerConfig::locale())
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_MEMBERS).to_string())
}

// Fallback data
const FALLBACK_SONGS: &[&str] = &[
    "Tom Sawyer",
    "The Spirit of Radio",
    "Limelight",
    "YYZ",
    "The Trees",
    "Red Barchetta",
    "2112",
    "Subdivisions",
    "Closer to the Heart",
    "Freewill",
    "Working Man",
    "La Villa Strangiato",
    "Xanadu",
    "Fly by Night",
];

const FALLBACK_ALBUMS: &[&str] = &[
    "Moving Pictures",
    "2112",
    "Permanent Waves",
    "Signals",
    "Hemispheres",
    "A Farewell to Kings",
    "Grace Under Pressure",
    "Fly by Night",
    "Rush",
];

const FALLBACK_MEMBERS: &[&str] = &["Geddy Lee", "Alex Lifeson", "Neil Peart", "John Rutsey"];

// --- gem parity generators (automated, locale-backed) ---
fn parity_pick(key: &str) -> String {
    crate::locale::fetch_locale(key, &crate::config::FakerConfig::locale())
        .map(|v| crate::base::sample(&v))
        .unwrap_or_default()
}

/// Gem-compatible generator: `rush.players`.
pub fn player() -> String {
    parity_pick("rush.players")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_song() {
        assert!(!song().is_empty());
    }

    #[test]
    fn test_album() {
        assert!(!album().is_empty());
    }

    #[test]
    fn test_member() {
        assert!(!member().is_empty());
    }
}

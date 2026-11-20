//! Smashing Pumpkins music generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Smashing Pumpkins song
pub fn song() -> String {
    fetch_locale(
        "smashing_pumpkins.songs",
        &crate::config::FakerConfig::locale(),
    )
    .map(|v| sample(&v))
    .unwrap_or_else(|| sample(FALLBACK_SONGS).to_string())
}

/// Generate a random Smashing Pumpkins album
pub fn album() -> String {
    fetch_locale(
        "smashing_pumpkins.albums",
        &crate::config::FakerConfig::locale(),
    )
    .map(|v| sample(&v))
    .unwrap_or_else(|| sample(FALLBACK_ALBUMS).to_string())
}

/// Generate a random Smashing Pumpkins member
pub fn member() -> String {
    fetch_locale(
        "smashing_pumpkins.members",
        &crate::config::FakerConfig::locale(),
    )
    .map(|v| sample(&v))
    .unwrap_or_else(|| sample(FALLBACK_MEMBERS).to_string())
}

// Fallback data
const FALLBACK_SONGS: &[&str] = &[
    "1979",
    "Bullet with Butterfly Wings",
    "Tonight, Tonight",
    "Disarm",
    "Cherub Rock",
    "Today",
    "Zero",
    "Mayonaise",
    "Today",
    "Ava Adore",
    "Perfect",
    "The Everlasting Gaze",
];

const FALLBACK_ALBUMS: &[&str] = &[
    "Siamese Dream",
    "Mellon Collie and the Infinite Sadness",
    "Adore",
    "Machina/The Machines of God",
    "Gish",
    "Zeitgeist",
    "Oceania",
];

const FALLBACK_MEMBERS: &[&str] = &[
    "Billy Corgan",
    "James Iha",
    "D'arcy Wretzky",
    "Jimmy Chamberlin",
    "Melissa Auf der Maur",
];

// --- gem parity generators (automated, locale-backed) ---
fn parity_pick(key: &str) -> String {
    crate::locale::fetch_locale(key, &crate::config::FakerConfig::locale())
        .map(|v| crate::base::sample(&v))
        .unwrap_or_default()
}

/// Gem-compatible generator: `smashing_pumpkins.lyric`.
pub fn lyric() -> String {
    parity_pick("smashing_pumpkins.lyric")
}

/// Gem-compatible generator: `smashing_pumpkins.musicians`.
pub fn musician() -> String {
    parity_pick("smashing_pumpkins.musicians")
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
